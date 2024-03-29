

use bevy::ecs::component::Component;
use bevy::ecs::world::Mut;
use bevy::math::Vec2;
use bevy::text::Text;
use bevy_defer::{AsyncComponent, AsyncResult};
use crate::layout::{Layout, LayoutObject};
use crate::widgets::TextFragment;
use crate::widgets::inputbox::InputBox;
use crate::{Hitbox, HitboxShape, Anchor, SizeUnit, Size};
use crate::{Size2, FontSize, layout::Alignment, layout::LayoutDir};

use super::DslFrom;
use crate::util::convert::{DslConvert, DslInto, SealToken};

impl Hitbox {
    pub fn rect(value: impl DslInto<OneOrTwo<Vec2>>) -> Self {
        Hitbox {
            shape: HitboxShape::Rect,
            scale: value.dinto().0,
        }
    }

    pub fn ellipse(value: impl DslInto<OneOrTwo<Vec2>>) -> Self {
        Hitbox {
            shape: HitboxShape::Ellipse,
            scale: value.dinto().0,
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy)]
pub enum Aspect {
    #[default]
    None,
    /// Preserves the aspect from the associated sprite.
    Preserve,
    /// Experimental, does nothing.
    Owned(f32),
}

impl DslFrom<i32> for Aspect {
    fn dfrom(value: i32) -> Self {
        Aspect::Owned(value as f32)
    }
}

impl DslFrom<f32> for Aspect {
    fn dfrom(value: f32) -> Self {
        Aspect::Owned(value)
    }
}

impl<T> DslFrom<T> for Option<LayoutObject> where T: Layout {
    fn dfrom(value: T) -> Self {
        Some(LayoutObject::new(value))
    }
}

/// Unified constants for various enums used by `bevy_rectray`.
///
/// Note `Left` can be used as `CenterLeft`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacialConst {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl DslFrom<SpacialConst> for Anchor {
    fn dfrom(value: SpacialConst) -> Self {
        match value {
            SpacialConst::TopLeft => Anchor::TOP_LEFT,
            SpacialConst::TopCenter => Anchor::TOP_CENTER,
            SpacialConst::TopRight => Anchor::TOP_RIGHT,
            SpacialConst::CenterLeft => Anchor::CENTER_LEFT,
            SpacialConst::Center => Anchor::CENTER,
            SpacialConst::CenterRight => Anchor::CENTER_RIGHT,
            SpacialConst::BottomLeft => Anchor::BOTTOM_LEFT,
            SpacialConst::BottomCenter => Anchor::BOTTOM_CENTER,
            SpacialConst::BottomRight => Anchor::BOTTOM_RIGHT,
            SpacialConst::Top => Anchor::TOP_CENTER,
            SpacialConst::Bottom => Anchor::BOTTOM_CENTER,
            SpacialConst::Left => Anchor::CENTER_LEFT,
            SpacialConst::Right => Anchor::CENTER_RIGHT,
            c => panic!("{:?} is not an Anchor.", c),
        }
    }
}

impl DslFrom<SpacialConst> for Option<Anchor> {
    fn dfrom(value: SpacialConst) -> Self {
        Some(match value {
            SpacialConst::TopLeft => Anchor::TOP_LEFT,
            SpacialConst::TopCenter => Anchor::TOP_CENTER,
            SpacialConst::TopRight => Anchor::TOP_RIGHT,
            SpacialConst::CenterLeft => Anchor::CENTER_LEFT,
            SpacialConst::Center => Anchor::CENTER,
            SpacialConst::CenterRight => Anchor::CENTER_RIGHT,
            SpacialConst::BottomLeft => Anchor::BOTTOM_LEFT,
            SpacialConst::BottomCenter => Anchor::BOTTOM_CENTER,
            SpacialConst::BottomRight => Anchor::BOTTOM_RIGHT,
            SpacialConst::Top => Anchor::TOP_CENTER,
            SpacialConst::Bottom => Anchor::BOTTOM_CENTER,
            SpacialConst::Left => Anchor::CENTER_LEFT,
            SpacialConst::Right => Anchor::CENTER_RIGHT,
            c => panic!("{:?} is not an Anchor.", c),
        })
    }
}

type BevyAnchor = bevy::sprite::Anchor;

impl DslInto<BevyAnchor> for SpacialConst {
    fn dinto(self) -> BevyAnchor {
        match self {
            SpacialConst::TopLeft => BevyAnchor::TopLeft,
            SpacialConst::TopCenter => BevyAnchor::TopCenter,
            SpacialConst::TopRight => BevyAnchor::TopRight,
            SpacialConst::CenterLeft => BevyAnchor::CenterLeft,
            SpacialConst::Center => BevyAnchor::Center,
            SpacialConst::CenterRight => BevyAnchor::CenterRight,
            SpacialConst::BottomLeft => BevyAnchor::BottomLeft,
            SpacialConst::BottomCenter => BevyAnchor::BottomCenter,
            SpacialConst::BottomRight => BevyAnchor::BottomRight,
            SpacialConst::Top => BevyAnchor::TopCenter,
            SpacialConst::Bottom => BevyAnchor::BottomCenter,
            SpacialConst::Left => BevyAnchor::CenterLeft,
            SpacialConst::Right => BevyAnchor::CenterRight,
            c => panic!("{:?} is not an Anchor.", c),
        }
    }
}

impl DslInto<Alignment> for SpacialConst {
    fn dinto(self) -> Alignment {
        match self {
            SpacialConst::Center => Alignment::Center,
            SpacialConst::Top => Alignment::Top,
            SpacialConst::Bottom => Alignment::Bottom,
            SpacialConst::Left => Alignment::Left,
            SpacialConst::Right => Alignment::Right,
            c => panic!("{:?} is not an Alignment.", c),
        }
    }
}

impl DslInto<Option<Alignment>> for SpacialConst {
    fn dinto(self) -> Option<Alignment> {
        Some(match self {
            SpacialConst::Center => Alignment::Center,
            SpacialConst::Top => Alignment::Top,
            SpacialConst::Bottom => Alignment::Bottom,
            SpacialConst::Left => Alignment::Left,
            SpacialConst::Right => Alignment::Right,
            c => panic!("{:?} is not an Alignment.", c),
        })
    }
}

impl DslInto<LayoutDir> for SpacialConst {
    fn dinto(self) -> LayoutDir {
        match self {
            SpacialConst::LeftToRight => LayoutDir::LeftToRight,
            SpacialConst::RightToLeft => LayoutDir::RightToLeft,
            SpacialConst::TopToBottom => LayoutDir::TopToBottom,
            SpacialConst::BottomToTop => LayoutDir::BottomToTop,
            c => panic!("{:?} is not an FlexDir.", c),
        }
    }
}

impl DslInto<Option<LayoutDir>> for SpacialConst {
    fn dinto(self) -> Option<LayoutDir> {
        Some(match self {
            SpacialConst::LeftToRight => LayoutDir::LeftToRight,
            SpacialConst::RightToLeft => LayoutDir::RightToLeft,
            SpacialConst::TopToBottom => LayoutDir::TopToBottom,
            SpacialConst::BottomToTop => LayoutDir::BottomToTop,
            c => panic!("{:?} is not an FlexDir.", c),
        })
    }
}

/// Color construction macro, see [`colorthis`].
///
/// Input is `RgbaLinear`, but immediately cast into `Rgba`(sRGB).
///
/// ```
/// # use bevy_rectray::color;
/// color!(red400);
/// ```
#[macro_export]
macro_rules! color {
    ($color: tt) => {
        {
            #[allow(clippy::excessive_precision)]
            $crate::dsl::rgbaf!(
                $crate::bevy::prelude::Color::RgbaLinear,
                $color => {red, green, blue, alpha}
            ).as_rgba()
        }
    };
}

/// Create an array of colors.
#[macro_export]
macro_rules! colors {
    [$($color: tt),* $(,)?] => {
        [$($crate::color!($color)),*]
    };
}

/// Construct a list of colors used with interpolation.
#[macro_export]
macro_rules! gradient {
    [$(($color: tt, $frac: expr)),* $(,)?] => {
        [$(($crate::color!($color), $frac)),*]
    };
    [$first: tt, $second: tt $(,)?] => {
        [($crate::color!($first), 0.0), ($crate::color!($second), 1.0)]
    };
}


/// Convert degrees to radians
pub fn degrees(f: impl DslInto<f32>) -> f32{
    f32::to_radians(f.dinto())
}

/// Convert `Vec2` to radians
pub fn angle(f: impl DslInto<Vec2>) -> f32{
    let v = f.dinto();
    f32::atan2(v.y, v.x)
}

/// One dimensional size by `px`.
pub fn px(f: impl DslInto<f32>) -> Size {
    Size::new(SizeUnit::Pixels, f.dinto())
}

/// One dimensional size by `em`.
pub fn em(f: impl DslInto<f32>) -> Size {
    Size::new(SizeUnit::Em, f.dinto())
}

/// One dimensional size by `rem`.
pub fn rem(f: impl DslInto<f32>) -> Size {
    Size::new(SizeUnit::Rem, f.dinto())
}

/// One dimensional size by `%`.
///
/// Use values like `40`, not `0.4`.
pub fn percent(f: impl DslInto<f32>) -> Size {
    Size::new(SizeUnit::Percent, f.dinto() / 100.0)
}

impl DslFrom<Size> for FontSize {
    fn dfrom(value: Size) -> Self {
        match value.unit {
            SizeUnit::Pixels => FontSize::Pixels(value.value),
            SizeUnit::Em => FontSize::Ems(value.value),
            SizeUnit::Rem => FontSize::Rems(value.value),
            u => panic!("Unsupported SizeUnit {:?} as FontSize.", u)
        }
    }
}

/// Accepts 1 or 2 numbers for a `Vec2` or a `Size2`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OneOrTwo<T>(pub T);

impl<A, T> DslConvert<OneOrTwo<T>, '2'> for A where A: DslInto<T>{
    fn parse(self) -> OneOrTwo<T> {
        OneOrTwo(self.dinto())
    }

    fn sealed(_: SealToken) {}
}

macro_rules! impl_one_or_two {
    ($ty: ty, $x: ident, $y: ident, $expr: expr) => {

impl DslInto<OneOrTwo<$ty>> for i32 {
    fn dinto(self) -> OneOrTwo<$ty> {
        let $x = self;
        let $y = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for f32 {
    fn dinto(self) -> OneOrTwo<$ty> {
        let $x = self;
        let $y = self;
        OneOrTwo($expr)
    }
}

    };
}

impl_one_or_two!(Vec2, x, y, Vec2::new(x as f32, y as f32));
impl_one_or_two!(Size2, x, y, Size2::pixels(x as f32, y as f32));

impl DslInto<OneOrTwo<Size2>> for Size {
    fn dinto(self) -> OneOrTwo<Size2> {
        OneOrTwo(Size2::new(self, self))
    }
}

/// A `OneOrTwo<Vec2>` with default value `1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale(pub Vec2);
impl Default for Scale{
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

impl<A> DslConvert<Scale, 's'> for A where A: DslInto<Vec2>{
    fn parse(self) -> Scale {
        Scale(self.dinto())
    }
    fn sealed(_: SealToken) {}
}

impl DslInto<Scale> for i32 {
    fn dinto(self) -> Scale {
        Scale(Vec2::splat(self as f32))
    }
}

impl DslInto<Scale> for f32 {
    fn dinto(self) -> Scale {
        Scale(Vec2::splat(self))
    }
}

/// A `Anchor` with default value `INHERIT`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParentAnchor(pub Anchor);
impl Default for ParentAnchor{
    fn default() -> Self {
        Self(Anchor::INHERIT)
    }
}

impl From<Anchor> for ParentAnchor {
    fn from(value: Anchor) -> Self {
        ParentAnchor(value)
    }
}

impl<A> DslConvert<ParentAnchor, 'A'> for A where A: DslInto<Anchor>{
    fn parse(self) -> ParentAnchor {
        ParentAnchor(self.dinto())
    }
    fn sealed(_: SealToken) {}
}


/// Construct a [`Size`](crate::Size) through CSS like syntax.
#[macro_export]
macro_rules! size {
    (infer) => {
        $crate::Size::new($crate::SizeUnit::Infer, 0.0)
    };
    ($x: tt) => {
        $crate::Size::new($crate::SizeUnit::Pixels, $x as f32)
    };
    (-$x: tt) => {
        $crate::Size::new($crate::SizeUnit::Pixels, -($x as f32))
    };
    ($x: tt px) => {
        $crate::Size::new($crate::SizeUnit::Pixels, $x as f32)
    };
    (-$x: tt px) => {
        $crate::Size::new($crate::SizeUnit::Pixels, -($x as f32))
    };
    ($x: tt em) => {
        $crate::Size::new($crate::SizeUnit::Em, $x as f32)
    };
    (-$x: tt em) => {
        $crate::Size::new($crate::SizeUnit::Em, -($x as f32))
    };
    ($x: tt rem) => {
        $crate::Size::new($crate::SizeUnit::Rem, $x as f32)
    };
    (-$x: tt rem) => {
        $crate::Size::new($crate::SizeUnit::Rem, -($x as f32))
    };
    ($x: tt %) => {
        $crate::Size::new($crate::SizeUnit::Percent, $x as f32 / 100.0)
    };
    (-$x: tt %) => {
        $crate::Size::new($crate::SizeUnit::Percent, -($x as f32) / 100.0)
    };
    (1 + $x: tt px) => {
        $crate::Size::new($crate::SizeUnit::MarginPx, $x as f32)
    };
    (1 - $x: tt px) => {
        $crate::Size::new($crate::SizeUnit::MarginPx, -($x as f32))
    };
    (1 + $x: tt em) => {
        $crate::Size::new($crate::SizeUnit::MarginEm, $x as f32)
    };
    (1 - $x: tt em) => {
        $crate::Size::new($crate::SizeUnit::MarginEm, -($x as f32))
    };
    (1 + $x: tt rem) => {
        $crate::Size::new($crate::SizeUnit::MarginRem, $x as f32)
    };
    (1 - $x: tt rem) => {
        $crate::Size::new($crate::SizeUnit::MarginRem, -($x as f32))
    };
}


/// Construct a [`Size2`](crate::Size2) through CSS like syntax.
///
/// # Examples
/// ```
/// // We perform auto float conversion.
/// size2!(40, 50.5);
/// // Supply a unit like this
/// size2!([1, 1] rem);
/// // Supply multiple unit types like this.
/// size2!(40%, 1 em);
/// // Aside from the negative sign
/// // expressions need to be in wrapped parenthesis or braces.
/// size2!(-3 px, (PI * 2.0) rem);
/// size2!([-3 px, {
///     let pi = 3.0;
///     pi * 2.0
/// } rem]);
/// // `1 - 2px` means `100% - 2px`, or 2px smaller than parent dimension.
/// size2!(1 - 2 px, 1 + 4 em);
/// // or expressed as
/// size2!(1 - [4.5, 6.6] px);
/// ```
///
/// # Note
///
/// * `1px` is not valid rust syntax, always use `1 px`.
#[macro_export]
macro_rules! size2 {
    (full) => {
        $crate::Size2::FULL
    };
    (0) => {
        $crate::Size2::ZERO
    };
    ([$($tt:tt)*]) => {
        $crate::size2!(@accumulate [] [$($tt)*])
    };
    (@accumulate [$($tt1:tt)*] []) => {
        compile_error!("Expected 2 expressions, found 1.")
    };
    (@accumulate [$($tt1:tt)*] [, $($tt2:tt)*]) => {
        $crate::Size2::new($crate::size!($($tt1)*), $crate::size!($($tt2)*))
    };
    (@accumulate [$($tt1:tt)*] [$tt:tt $($tt2:tt)*]) => {
        $crate::size2!(@accumulate [$($tt1)* $tt] [$($tt2)*])
    };
    ([$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!($x $unit, $y $unit)
    };
    (1 - [$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!(1 - $x $unit, 1 - $y $unit)
    };
    (1 + [$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!(1 + $x $unit, 1 + $y $unit)
    };
    ($($tt:tt)*) => {
        $crate::size2!(@accumulate [] [$($tt)*])
    };
}

/// Format trait for a widget.
pub trait WidgetWrite {
    fn write(self, s: String);
}

impl WidgetWrite for &mut Text {
    fn write(self, s: String) {
        if let Some(section) = self.sections.first_mut() {
            section.value = s;
        }
    }
}

impl WidgetWrite for &mut InputBox {
    fn write(self, s: String) {
        self.set(s)
    }
}

impl WidgetWrite for &mut TextFragment {
    fn write(self, s: String) {
        self.text = s
    }
}

impl WidgetWrite for Mut<'_, Text> {
    fn write(mut self, s: String) {
        if let Some(section) = self.sections.first_mut() {
            section.value = s;
        }
    }
}

impl WidgetWrite for Mut<'_, InputBox> {
    fn write(mut self, s: String) {
        self.set(s)
    }
}

impl WidgetWrite for Mut<'_, TextFragment> {
    fn write(mut self, s: String) {
        self.text = s
    }
}

#[allow(async_fn_in_trait)]
pub trait WidgetWriteAsync {
    async fn write(self, s: impl Into<String>) -> AsyncResult<()>;
}

impl<C: Component> WidgetWriteAsync for AsyncComponent<'_, C> where for<'t> &'t mut C: WidgetWrite {
    async fn write(self, s: impl Into<String>) -> AsyncResult<()> {
        let s = s.into();
        self.set(move |x| x.write(s)).await
    }
}

/// Write to a text widget component using `format!` syntax.
///
/// The component must implement [`WidgetWrite`].
#[macro_export]
macro_rules! format_widget {
    ($widget: expr, $s: literal $(,$rest: expr),* $(,)?) => {
        $crate::dsl::WidgetWrite::write($widget, format!($s, $($rest),*))
    };
}