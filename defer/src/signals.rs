use std::{any::{type_name, Any, TypeId}, fmt::Debug, marker::PhantomData};
use triomphe::Arc;
use bevy::{ecs::{component::Component, entity::Entity, query::WorldQuery}, log::debug, utils::hashbrown::HashMap};
use once_cell::sync::Lazy;
use crate::object::{Object, AsObject};
use super::{AsyncExecutor, AsyncSystemParam, Signal, SignalData, SignalInner, YieldNow};

/// A marker type that indicates the type and purpose of a signal.
pub trait SignalId: Any + Send + Sync + 'static{
    type Data: AsObject;
}

/// Quickly construct multiple marker [`SignalId`]s at once.
/// 
/// # Example
/// ```
/// signal_ids!{
///     /// Shared factor as a f32
///     SharedFactor: f32,
///     /// Shared position as a Vec2
///     pub SharedPosition: Vec2,
/// }
/// ```
#[macro_export]
macro_rules! signal_ids {
    ($($(#[$($attr:tt)*])*$vis: vis $name: ident: $ty: ty),* $(,)?) => {
        $(
            $(#[$($attr)*])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            $vis enum $name {}

            impl $crate::SignalId for $name{
                type Data = $ty;
            }
        )*
    };
}


/// A type erased signal with a nominal type.
#[derive(Debug, Clone)]
pub struct TypedSignal<T: AsObject> {
    inner: Arc<SignalData<Object>>,
    p: PhantomData<T>,
}

impl<T: AsObject> Default for TypedSignal<T> {
    fn default() -> Self {
        Self { inner: Default::default(), p: PhantomData }
    }
}

impl<T: AsObject> TypedSignal<T> {

    pub fn new() -> Self {
        Self { inner: Default::default(), p: PhantomData }
    }

    pub fn from_inner(inner: Arc<SignalData<Object>>) -> Self {
        Self {
            inner,
            p: PhantomData
        }
    }
    
    pub fn into_inner(self) -> Arc<SignalData<Object>> {
        self.inner
    }

    pub fn type_erase(self) -> TypedSignal<Object> {
        TypedSignal { 
            inner: self.inner, 
            p: PhantomData 
        }
    }
}

impl TypedSignal<Object> {
    pub fn of_type<T: AsObject>(self) -> TypedSignal<T> {
        TypedSignal { 
            inner: self.inner, 
            p: PhantomData 
        }
    }
}

pub(super) static DUMMY_SIGNALS: Lazy<Signals> = Lazy::new(Signals::new);

pub(crate) trait SignalMapperTrait: Send + Sync + 'static {
    fn map(&self, obj: &mut Object);
    fn dyn_clone(&self) -> Box<dyn SignalMapperTrait>;
}

impl<T> SignalMapperTrait for T where T: Fn(&mut Object) + Clone + Send + Sync + 'static {
    fn map(&self, obj: &mut Object) {
        self(obj)
    }
    fn dyn_clone(&self) -> Box<dyn SignalMapperTrait> {
        Box::new(self.clone())
    }
}

/// A function that maps a signal's value.
pub struct SignalMapper(Box<dyn SignalMapperTrait>);

impl Debug for SignalMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalMapper").finish()
    }
}

impl Clone for SignalMapper {
    fn clone(&self) -> Self {
        Self(self.0.dyn_clone())
    }
}

impl SignalMapper {
    pub fn new<A: SignalId, B: SignalId>(f: impl Fn(A::Data) -> B::Data + Clone + Send + Sync + 'static) -> Self {
        Self(Box::new(move |obj: &mut Object| {
            let Some(item) = obj.clone().get::<A::Data>() else {return};
            *obj = Object::new(f(item));
        }))
    }

    pub fn map<T: AsObject>(&self, mut obj: Object) -> Option<T> {
        self.0.map(&mut obj);
        obj.get()
    }
}

/// A composable component that contains signals on an `Entity`.
#[derive(Debug, Component, Default)]
pub struct Signals {
    pub senders: HashMap<TypeId, Signal<Object>>,
    pub receivers: HashMap<TypeId, Signal<Object>>,
    pub adaptors: HashMap<TypeId, (TypeId, SignalMapper)>
}

impl Signals {
    pub fn new() -> Self {
        Self { senders: HashMap::new(), receivers: HashMap::new(), adaptors: HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.senders.is_empty() && self.receivers.is_empty()
    }

    pub fn from_sender<T: SignalId>(signal: TypedSignal<T::Data>) -> Self {
        let mut this = Self::new();
        this.add_sender::<T>(signal);
        this
    }

    pub fn from_receiver<T: SignalId>(signal: TypedSignal<T::Data>) -> Self {
        let mut this = Self::new();
        this.add_receiver::<T>(signal);
        this
    }

    pub fn from_adaptor<T: SignalId>(ty: TypeId, mapper: SignalMapper) -> Self {
        let mut this = Self::new();
        this.add_adaptor::<T>(ty, mapper);
        this
    }


    pub fn with_sender<T: SignalId>(mut self, signal: TypedSignal<T::Data>) -> Self {
        self.add_sender::<T>(signal);
        self
    }

    pub fn with_receiver<T: SignalId>(mut self, signal: TypedSignal<T::Data>) -> Self {
        self.add_receiver::<T>(signal);
        self
    }

    pub fn with_adaptor<T: SignalId>(mut self, ty: TypeId, mapper: SignalMapper) -> Self {
        self.add_adaptor::<T>(ty, mapper);
        self
    }

    pub fn send<T: SignalId>(&self, item: T::Data) {
        if let Some(x) = self.senders.get(&TypeId::of::<T>()) {
            debug!("Signal {} sent with value {:?}", std::any::type_name::<T>(), &item);
            x.write(Object::new(item))
        }
    }

    pub fn broadcast<T: SignalId>(&self, item: T::Data) {
        if let Some(x) = self.senders.get(&TypeId::of::<T>()) {
            debug!("Signal {} sent value {:?}", std::any::type_name::<T>(), &item);
            x.broadcast(Object::new(item))
        }
    }

    pub fn poll_once<T: SignalId>(&self) -> Option<T::Data>{
        if let Some(sig) = self.receivers.get(&TypeId::of::<T>()) {
            sig.try_read().and_then(|x| x.get()).map(|x| {
            debug!("Signal {} received value {:?}", std::any::type_name::<T>(), &x);
            x
        })} else {
            match &self.adaptors.get(&TypeId::of::<T>()) {
                Some((ty, map)) => match self.receivers.get(ty){
                    Some(sig) => sig.try_read().and_then(|x| {
                        map.map(x).map(|x| {
                            debug!("Signal {} received and adapted value {:?}", std::any::type_name::<T>(), &x);
                            x
                        })
                    }),
                    None => None
                }
                None => None
            }
        }
    }

    pub fn poll_sender_once<T: SignalId>(&self) -> Option<T::Data>{
        match self.senders.get(&TypeId::of::<T>()){
            Some(sig) => sig.try_read().and_then(|x| x.get()).map(|x| {
                debug!("Signal sender {} received value {:?}", std::any::type_name::<T>(), &x);
                x
            }),
            None => None,
        }
    }
    
    pub fn borrow_sender<T: SignalId>(&self) -> Option<Arc<SignalInner<Object>>> {
        self.senders.get(&TypeId::of::<T>()).map(|x| x.borrow_inner())
    }
    pub fn borrow_receiver<T: SignalId>(&self) ->  Option<Arc<SignalInner<Object>>> {
        self.receivers.get(&TypeId::of::<T>()).map(|x| x.borrow_inner())
    }
    pub fn add_sender<T: SignalId>(&mut self, signal: TypedSignal<T::Data>) {
        self.senders.insert(TypeId::of::<T>(), Signal::from_typed(signal));
    }
    pub fn add_receiver<T: SignalId>(&mut self, signal: TypedSignal<T::Data>) {
        self.receivers.insert(TypeId::of::<T>(), Signal::from_typed(signal));
    }
    pub fn add_adaptor<T: SignalId>(&mut self, ty: TypeId, mapper: SignalMapper) {
        self.adaptors.insert(TypeId::of::<T>(), (ty, mapper));
    }

    pub fn remove_sender<T: SignalId>(&mut self) {
        self.senders.remove(&TypeId::of::<T>());
    }
    pub fn remove_receiver<T: SignalId>(&mut self) {
        self.receivers.remove(&TypeId::of::<T>());
    }
    pub fn remove_adaptor<T: SignalId>(&mut self) {
        self.adaptors.remove(&TypeId::of::<T>());
    }

    pub fn has_sender<T: SignalId>(&self) -> bool {
        self.senders.contains_key(&TypeId::of::<T>())
    }
    pub fn has_receiver<T: SignalId>(&self) ->  bool {
        self.receivers.contains_key(&TypeId::of::<T>())
    }
}

/// `AsyncSystemParam` for sending a signal.
pub struct SigSend<T: SignalId>(Arc<SignalInner<Object>>, PhantomData<T>);

impl<T: SignalId> SigSend<T> {
    /// Send a value with a signal, can be polled by the same sender.
    pub fn send(self, item: T::Data) -> impl Fn() + Send + Sync + 'static  {
        let obj = Object::new(item);
        move ||self.0.write(obj.clone())
    }

    /// Send a value with a signal, cannot be polled by the same sender.
    pub fn broadcast(self, item: T::Data) -> impl Fn() + Send + Sync + 'static  {
        let obj = Object::new(item);
        move ||self.0.broadcast(obj.clone())
    }

    /// Receives a value from the sender.
    pub async fn recv(self) -> T::Data {
        loop {
            let signal = self.0.clone();
            let obj = signal.async_read().await;
            if let Some(data) = obj.get() {
                return data;
            } else {
                YieldNow::new().await
            }
        }
    }
}

impl <T: SignalId> AsyncSystemParam for SigSend<T>  {
    fn from_async_context(
            _: Entity,
            _: &Arc<AsyncExecutor>,
            signals: &Signals,
        ) -> Self {
        SigSend(
            signals.borrow_sender::<T>()
                .unwrap_or_else(|| panic!("Signal sender of type <{}> missing", type_name::<T>())),
            PhantomData
        )
    }
}

/// `AsyncSystemParam` for receiving a signal.
pub struct SigRecv<T: SignalId>(Arc<SignalInner<Object>>, PhantomData<T>);

impl<T: SignalId> SigRecv<T> {
    /// Receive a signal.
    pub async fn recv(&self) -> T::Data {
        loop {
            let signal = self.0.clone();
            let obj = signal.async_read().await;
            if let Some(data) = obj.get() {
                return data;
            } else {
                YieldNow::new().await
            }
        }
    }
}

impl<T: SignalId<Data = Object>> SigRecv<T> {
    /// Receives and downcasts a signal, discard all invalid typed values.
    pub async fn recv_as<A: AsObject>(&self) -> A {
        loop {
            let signal = self.0.clone();
            let obj = signal.async_read().await;
            if let Some(data) = obj.get() {
                return data;
            } else {
                YieldNow::new().await
            }
        }
    }
}


impl <T: SignalId> AsyncSystemParam for SigRecv<T>  {
    fn from_async_context(
            _: Entity,
            _: &Arc<AsyncExecutor>,
            signals: &Signals,
        ) -> Self {
        SigRecv(
            signals.borrow_receiver::<T>()
                .unwrap_or_else(|| panic!("Signal receiver of type <{}> missing", type_name::<T>())),
            PhantomData
        )
    }
}

/// `WorldQuery` for sending a signal synchronously.
#[derive(Debug, WorldQuery)]
pub struct SignalSender<T: SignalId>{
    signals: Option<&'static Signals>,
    p: PhantomData<T>,
}

impl<T: SignalId> SignalSenderItem<'_, T> {
    /// Check if a sender exists.
    pub fn exists(&self) -> bool{
        self.signals
            .map(|x| x.borrow_sender::<T>().is_some())
            .unwrap_or(false)
    }

    /// Send a item through a signal, can be polled from the same sender.
    pub fn send(&self, item: T::Data) {
        if let Some(signals) = self.signals {
            signals.send::<T>(item);
        }
    }
    
    /// Send a item through a signal, cannot be polled from the same sender.
    pub fn broadcast(&self, item: T::Data) {
        if let Some(signals) = self.signals {
            signals.broadcast::<T>(item);
        }
    }

    /// Poll the signal from a sender.
    pub fn poll_sender(&self) -> Option<T::Data> {
        self.signals.and_then(|s| s.poll_sender_once::<T>())
    }
}

/// `WorldQuery` for receiving a signal synchronously.
#[derive(Debug, WorldQuery)]
pub struct SignalReceiver<T: SignalId>{
    signals: Option<&'static Signals>,
    p: PhantomData<T>,
}

impl<T: SignalId> SignalReceiverItem<'_, T> {
    pub fn poll_once(&self) -> Option<T::Data> {
        self.signals.as_ref()
            .and_then(|sig| sig.poll_once::<T>())
    }

    pub fn poll_any(&self) -> bool {
        self.signals.as_ref()
            .and_then(|sig| sig.poll_once::<T>())
            .is_some()
    }
}

/// A signal with a role, that can be composed with [`Signals`].
pub enum RoleSignal<T: SignalId>{
    Sender(TypedSignal<T::Data>),
    Receiver(TypedSignal<T::Data>),
    Adaptor(TypeId, SignalMapper),
}

impl<T: SignalId> RoleSignal<T> {
    pub fn and<A: SignalId>(self, other: RoleSignal<A>) -> Signals {
        let base = match self {
            RoleSignal::Sender(s) => Signals::from_sender::<T>(s),
            RoleSignal::Receiver(r) => Signals::from_receiver::<T>(r),
            RoleSignal::Adaptor(t, a) => {
                let mut s = Signals::new();
                s.add_adaptor::<T>(t, a);
                s
            },
        };
        base.and(other)
    }

    pub fn into_signals(self) -> Signals {
        match self {
            RoleSignal::Sender(s) => Signals::from_sender::<T>(s),
            RoleSignal::Receiver(r) => Signals::from_receiver::<T>(r),
            RoleSignal::Adaptor(t, a) => {
                let mut s = Signals::new();
                s.add_adaptor::<T>(t, a);
                s
            },
        }
    }
}

impl Signals {
    pub fn and<A: SignalId>(self, other: RoleSignal<A>) -> Signals {
        match other {
            RoleSignal::Sender(s) => self.with_sender::<A>(s),
            RoleSignal::Receiver(r) => self.with_receiver::<A>(r),
            RoleSignal::Adaptor(t, a) => self.with_adaptor::<A>(t, a),
        }
    }

    pub fn into_signals(self) -> Signals {
        self
    }
}