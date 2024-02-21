use std::marker::PhantomData;
use triomphe::Arc;
use bevy::ecs::{component::Component, entity::Entity, world::{EntityRef, World}};
use std::future::Future;
use crate::oneshot;

use super::{AsyncExecutor, AsyncFailure, BoxedReadonlyCallback, AsyncResult, AsyncSystemParam, Signals};

/// Tuple of [`Component`]s as a readonly query.
pub trait ComponentRefQuery {
    type Output<'t>;
    fn from_entity<'t>(entity: &'t EntityRef) -> Option<Self::Output<'t>>;
}

macro_rules! impl_component_query {
    () => {};
    ($($name: ident),*) => {
        impl<$($name: Component),*> ComponentRefQuery for ($($name,)*) {
            type Output<'t> = ($(&'t $name,)*);
            fn from_entity<'t>(entity: &'t EntityRef) -> Option<Self::Output<'t>>{
                Some(($(entity.get::<$name>()?,)*))
            }
        }
    };
}

macro_rules! impl_component_query_many {
    () => {};
    ($first: ident $(,$rest: ident)*) => {
        impl_component_query_many!($($rest),*);
        impl_component_query!($first $(,$rest)*);
    }
}


impl_component_query_many!(
    A, B, C, D, E,
    F, G, H, I, J,
    K, L, M, N, O
);

/// A fast readonly query for multiple components.
pub struct AsyncComponentsReadonly<T: ComponentRefQuery> {
    entity: Entity,
    executor: Arc<AsyncExecutor>,
    p: PhantomData<T>
}


impl<C: ComponentRefQuery> AsyncSystemParam for AsyncComponentsReadonly<C> {
    fn from_async_context(
        entity: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
    ) -> Self {
        Self {
            entity,
            executor: executor.clone(),
            p: PhantomData
        }
    }
}

impl<C: ComponentRefQuery> AsyncComponentsReadonly<C> {
    pub fn get<Out: Send + Sync + 'static>(&self, f: impl FnOnce(C::Output<'_>) -> Out + Send + Sync + 'static)
            -> impl Future<Output = AsyncResult<Out>> {
        let (sender, receiver) = oneshot::<Option<Out>>();
        let entity = self.entity;
        let query = BoxedReadonlyCallback::new(
            move |world: &World| {
                Some(f(C::from_entity(&world.entity(entity))?))
            },
            sender
        );
        {
            let mut lock = self.executor.readonly.lock();
            lock.push(query);
        }
        async {
            match receiver.await {
                Ok(Some(out)) => Ok(out),
                Ok(None) => Err(AsyncFailure::ComponentNotFound),
                Err(_) => Err(AsyncFailure::ChannelClosed),
            }
        }
    }
}