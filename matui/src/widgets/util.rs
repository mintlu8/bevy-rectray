use std::ops::{Deref, DerefMut};

use bevy::{render::color::Color, ecs::{component::Component, entity::Entity}};
use bevy_aoui::{dsl::{AouiCommands, DslFromOptionEx}, material_sprite, size2, layout::LayoutControl};

use crate::shapes::RoundedShadowMaterial;

/// Create a palette struct, every field must be a color.
///
/// ```
/// # /*
/// palette!(FramePalette {
///     foreground: red,
///     background: green,
/// })
/// # */
/// ```
/// Translates to:
/// ```
/// # /*
/// FramePalette {
///     foreground: color!(red),
///     background: color!(green),
///     ..Default::default()
/// }
/// # */
/// ```
#[macro_export]
macro_rules! palette {
    ($ty: ident {$($field: ident: $color: tt),* $(,)?}) => {
        $ty {
            $($field: $crate::aoui::color!($color),)*
            ..Default::default()
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ShadowInfo {
    /// Size of the shadow.
    pub size: f32,
    /// Color of the shadow, default is `black`.
    pub color: Color,
    /// Darkens the shadow.
    ///
    /// Value should be in `0..=1` but should realistically
    /// be around `0.0..=0.4` as `1` is completely dark.
    ///
    /// This effectively computes a more compact shadow
    /// of a slightly larger sprite to produce a darker shadow.
    pub darken: f32,
}

impl ShadowInfo {

    pub fn build_capsule(&self, commands: &mut AouiCommands) -> Entity {
        material_sprite!(commands {
            dimension: size2![1 + {self.size * 2.0} px, 1 + {self.size * 2.0} px],
            z: -0.005,
            material: RoundedShadowMaterial::capsule(self.color, self.size - self.size * self.darken),
            extra: LayoutControl::IgnoreLayout,
        })
    }

    pub fn build_rect(&self, commands: &mut AouiCommands, corner: f32) -> Entity {
        material_sprite!(commands {
            dimension: size2![1 + {self.size * 2.0} px, 1 + {self.size * 2.0} px],
            z: -0.005,
            material: RoundedShadowMaterial::new(self.color, corner, self.size - self.size * self.darken),
            extra: LayoutControl::IgnoreLayout,
        })
    }
}
impl Default for ShadowInfo {
    fn default() -> Self {
        Self {
            size: 0.0,
            color: Color::BLACK,
            darken: 0.0,
        }
    }
}


impl DslFromOptionEx<i32> for ShadowInfo {
    fn dfrom_option(value: i32) -> Self {
        ShadowInfo {
            size: value as f32,
            ..Default::default()
        }
    }
}

impl DslFromOptionEx<f32> for ShadowInfo {
    fn dfrom_option(value: f32) -> Self {
        ShadowInfo {
            size: value,
            ..Default::default()
        }
    }
}

impl DslFromOptionEx<(Color, i32)> for ShadowInfo {
    fn dfrom_option((color, size): (Color, i32)) -> Self {
        ShadowInfo {
            size: size as f32,
            color,
            ..Default::default()
        }
    }
}

impl DslFromOptionEx<(Color, f32)> for ShadowInfo {
    fn dfrom_option((color, size): (Color, f32)) -> Self {
        ShadowInfo {
            size,
            color,
            ..Default::default()
        }
    }
}

impl DslFromOptionEx<(i32, Color)> for ShadowInfo {
    fn dfrom_option((size, color): (i32, Color)) -> Self {
        ShadowInfo {
            size: size as f32,
            color,
            ..Default::default()
        }
    }
}

impl DslFromOptionEx<(f32, Color)> for ShadowInfo {
    fn dfrom_option((size, color): (f32, Color)) -> Self {
        ShadowInfo {
            size,
            color,
            ..Default::default()
        }
    }
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct StrokeColors<T>(pub T);

impl<T> Deref for StrokeColors<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StrokeColors<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct WidgetPalette {
    pub background: Color,
    pub foreground: Color,
    pub stroke: Color,
}