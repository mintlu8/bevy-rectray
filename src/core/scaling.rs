use bevy::{prelude::{Vec2, Resource}, reflect::Reflect};

/// The root font size of the window.
///
/// By default this is `16 px`.
#[derive(Debug, Resource, Reflect)]
pub struct RectrayRem(f32);

impl RectrayRem {
    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, rem: f32) {
        self.0 = rem
    }
}
impl Default for RectrayRem {
    fn default() -> Self {
        Self(16.0)
    }
}

/// Set the font size of the widget.
#[derive(Debug, Clone, Copy, Default, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontSize {
    #[default]
    None,
    Pixels(f32),
    Ems(f32),
    Rems(f32),
}

impl From<f32> for FontSize {
    fn from(value: f32) -> Self {
        Self::Pixels(value)
    }
}

/// The unit of a Size `px`, `em`, `rem`, `percent`
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SizeUnit{
    #[default]
    /// Pixels.
    Pixels,
    /// Font size.
    Em,
    /// Root font size.
    Rem,
    /// Percent of parent size.
    Percent,
    /// 100% + a px
    MarginPx,
    /// 100% + a em
    MarginEm,
    /// 100% + a rem
    MarginRem,
}


impl SizeUnit {

    /// Returns true if size is a percentage of parent's.
    #[inline]
    pub fn is_relative(&self) -> bool {
        matches!(self, SizeUnit::Percent | SizeUnit::MarginPx | SizeUnit::MarginEm | SizeUnit::MarginRem)
    }

    /// Compute size in pixels given parent info.
    #[inline]
    pub fn as_pixels(self, value: f32, parent: f32, em: f32, rem: f32) -> f32 {
        match self {
            SizeUnit::Pixels => value,
            SizeUnit::Em => value * em,
            SizeUnit::Rem => value * rem,
            SizeUnit::Percent => value * parent,
            SizeUnit::MarginPx => parent + value,
            SizeUnit::MarginEm => parent + value * em,
            SizeUnit::MarginRem => parent + value * rem,
        }
    }
}

/// A context sensitive Vec2.
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Size2 {
    x: SizeUnit,
    y: SizeUnit,
    raw: Vec2,
}

/// A context sensitive f32.
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Size {
    pub unit: SizeUnit,
    pub value: f32,
}

impl Size {

    pub const fn new(unit: SizeUnit, value: f32) -> Self{
        Size { unit, value }
    }

    /// Compute size in pixels given parent info.
    #[inline]
    pub fn as_pixels(self, parent: f32, em: f32, rem: f32) -> f32 {
        self.unit.as_pixels(self.value, parent, em, rem)
    }
}

impl Size2 {
    pub const ZERO: Self = Self {
        x: SizeUnit::Pixels,
        y: SizeUnit::Pixels,
        raw: Vec2::ZERO,
    };

    pub const MAX: Self = Self {
        x: SizeUnit::Pixels,
        y: SizeUnit::Pixels,
        raw: Vec2::MAX,
    };

    pub const FULL: Self = Self {
        x: SizeUnit::Percent,
        y: SizeUnit::Percent,
        raw: Vec2::ONE,
    };

    /// Construct size.
    pub const fn new(x: Size, y: Size) -> Self{
        Self {
            x: x.unit,
            y: y.unit,
            raw: Vec2::new(x.value, y.value)
        }
    }

    /// Construct size.
    pub const fn splat(x: Size) -> Self{
        Self {
            x: x.unit,
            y: x.unit,
            raw: Vec2::new(x.value, x.value)
        }
    }


    /// Size based on fixed number of pixels.
    pub const fn pixels(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Pixels,
            y: SizeUnit::Pixels,
            raw: Vec2::new(x, y),
        }
    }

    /// Size based on the parent relative size.
    pub const fn em(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Em,
            y: SizeUnit::Em,
            raw: Vec2::new(x, y),
        }
    }

    /// Size based on the root size.
    pub const fn rem(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Rem,
            y: SizeUnit::Rem,
            raw: Vec2::new(x, y),
        }
    }

    /// Size based on a percentage for the parent size.
    pub const fn percent(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Percent,
            y: SizeUnit::Percent,
            raw: Vec2::new(x, y),
        }
    }

    /// Compute size in pixels given parent info.
    #[inline]
    pub fn as_pixels(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        Vec2::new(
            self.x.as_pixels(self.raw.x, parent.x, em, rem),
            self.y.as_pixels(self.raw.y, parent.y, em, rem),
        )
    }

    /// Units of x and y.
    pub fn units(&self) -> (SizeUnit, SizeUnit) {
        (self.x, self.y)
    }

    /// Obtains this struct's value if units are pixels.
    pub fn get_pixels(&self) -> Option<Vec2> {
        match (self.x, self.y) {
            (SizeUnit::Pixels, SizeUnit::Pixels) => Some(self.raw),
            _ => None,
        }
    }

    /// Obtains this struct's underlying value.
    ///
    /// The unit and meaning of this value depends on the use case.
    pub fn raw(&self) -> Vec2 {
        self.raw
    }

    /// Get mutable access to the underlying value.
    #[doc(hidden)]
    pub fn raw_mut(&mut self) -> &mut Vec2 {
        &mut self.raw
    }

    /// Updates this struct's underlying value.
    ///
    /// The unit and meaning of this value depends on the use case.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        f(&mut self.raw)
    }
}

impl From<Vec2> for Size2 {
    fn from(value: Vec2) -> Self {
        Self {
            x: SizeUnit::Pixels,
            y: SizeUnit::Pixels,
            raw: value
        }
    }
}

impl FontSize {
    #[doc(hidden)]
    /// Get mutable access to the underlying value.
    pub fn raw_mut(&mut self) -> &mut f32 {
        match self {
            FontSize::None => panic!("Does not own a raw value."),
            FontSize::Pixels(f) => f,
            FontSize::Ems(f) => f,
            FontSize::Rems(f) => f,
        }
    }
}

#[cfg(feature="serde")]
const _:() = {
    use serde::{Serialize, Deserialize};
    impl Serialize for Size2 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            ((self.x, self.raw.x), (self.y, self.raw.y)).serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Size2 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            let ((ux, x), (uy, y)) = <_>::deserialize(deserializer)?;
            Ok(Self {
                x: ux,
                y: uy,
                raw: Vec2::new(x, y)
            })
        }
    }
};
