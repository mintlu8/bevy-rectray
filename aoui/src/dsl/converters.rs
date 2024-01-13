use bevy::asset::{Asset, Handle};
use crate::{widgets::button::Payload, signals::AsObject};
use super::{AouiCommands, convert::DslConvert};


/// Extended `Option` for the DSL.
///
/// Since dependants of this crate cannot implemnt `DslFrom` on `Option<T>` with foreigh types,
/// [`DslFromOptionEx`](super::DslFromOptionEx) can be used to make conversion to OptionX.
///
/// Using a crate local option type also works here.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum OptionEx<T> {
    Some(T),
    #[default]
    None,
}

impl<T> OptionEx<T> {
    pub fn expect(self, s: &str) -> T {
        match self {
            OptionEx::Some(v) => v,
            OptionEx::None => panic!("{}", s),
        }
    }

    pub fn unwrap_or(self, or: T) -> T {
        match self {
            OptionEx::Some(v) => v,
            OptionEx::None => or,
        }
    }

    pub fn unwrap_or_else(self, or: impl FnOnce() -> T) -> T {
        match self {
            OptionEx::Some(v) => v,
            OptionEx::None => or(),
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            OptionEx::Some(x) => Some(x),
            OptionEx::None => None,
        }
    }
}

/// For downstream crates,
/// implement this for specialized `Option` conversion with [`OptionEx`].
///
/// Enables conversion from `T` to [`OptionEx<Self>`].
pub trait DslFromOptionEx<T> {
    fn dfrom_option(value: T) -> Self;
}

impl<T, U> DslConvert<OptionEx<U>, 5> for T where U: DslFromOptionEx<T> {
    fn parse(self) -> OptionEx<U> {
        OptionEx::Some(U::dfrom_option(self))
    }
}

impl<T> DslConvert<Option<Payload>, 2> for T where T: AsObject{
    fn parse(self) -> Option<Payload> {
        Some(Payload::new(self))
    }
}

/// An [`Asset`], [`Handle<Asset>`], string path of an asset or none/default.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum IntoAsset<T: Asset>{
    #[default]
    None,
    Raw(T),
    Handle(Handle<T>),
    String(String),
}

impl<T> DslConvert<IntoAsset<T>, 1> for T where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::Raw(self)
    }
}

impl<T> DslConvert<IntoAsset<T>, 1> for Handle<T> where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::Handle(self)
    }
}

impl<T> DslConvert<IntoAsset<T>, 1> for &Handle<T> where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::Handle(self.clone())
    }
}

impl<T> DslConvert<IntoAsset<T>, 2> for String where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::String(self)
    }
}

impl<T> DslConvert<IntoAsset<T>, 2> for &str where T: Asset {
    fn parse(self) -> IntoAsset<T> {
        IntoAsset::String(self.to_owned())
    }
}

impl AouiCommands<'_, '_>{
    /// Load a dsl `IntoAsset`, if `None`, returns the default value.
    pub fn load_or_default<T: Asset>(&self, asset: IntoAsset<T>) -> Handle<T> {
        match asset {
            IntoAsset::None => Default::default(),
            IntoAsset::Raw(val) => self.add_asset(val),
            IntoAsset::Handle(handle) => handle,
            IntoAsset::String(string) => self.load(string),
        }
    }

    /// Load a dsl `IntoAsset`, if `None`, panic.
    pub fn load_or_panic<T: Asset>(&self, asset: IntoAsset<T>, err_msg: &str) -> Handle<T> {
        match asset {
            IntoAsset::None => panic!("{}", err_msg),
            IntoAsset::Raw(val) => self.add_asset(val),
            IntoAsset::Handle(handle) => handle,
            IntoAsset::String(string) => self.load(string),
        }
    }

    /// Load a dsl `IntoAsset`, returns an `Option`.
    pub fn try_load<T: Asset>(&self, asset: IntoAsset<T>) -> Option<Handle<T>> {
        match asset {
            IntoAsset::None => None,
            IntoAsset::Raw(val) => Some(self.add_asset(val)),
            IntoAsset::Handle(handle) => Some(handle),
            IntoAsset::String(string) => Some(self.load(string)),
        }
    }
}


impl<T: Asset> IntoAsset<T> {
    pub fn is_some(&self) -> bool{
        !matches!(self, Self::None)
    }

    pub fn is_none(&self) -> bool{
        matches!(self, Self::None)
    }
}
