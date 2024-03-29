use bevy::core::Name;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::Sprite;
use bevy::ecs::entity::Entity;
use bevy::math::{Vec2, Rect};
use bevy::text::{Text, TextSection, TextStyle, BreakLineOn, Text2dBounds, TextLayoutInfo, Font};
use bevy::render::texture::{Image, BevyDefault};
use bevy::render::render_resource::{Extent3d, TextureDimension};

use crate::{DimensionType, Transform2D, Dimension, Coloring};
use crate::{frame_extension, Clipping, bundles::{RectrayBundle, BuildTransformBundle}, Hitbox, build_frame, layout::Container};

use crate::util::{Widget, RCommands, convert::IntoAsset};
use super::Aspect;

frame_extension!(pub struct FrameBuilder {});
frame_extension!(
    pub struct SpriteBuilder {
        /// Handle of the image asset.
        pub sprite: IntoAsset<Image>,
        /// Size of the image.
        pub size: Option<Vec2>,
        /// Atlas rectangle of the image.
        pub rect: Option<Rect>,
        /// Flips the image.
        pub flip: [bool; 2],
    }
);

frame_extension!(
    pub struct RectangleBuilder {
        /// Size of the image.
        pub size: Option<Vec2>,
    }
);


frame_extension!(
    pub struct TextBuilder {
        /// The text string.
        pub text: String,
        /// Handle of the font asset.
        pub font: IntoAsset<Font>,
        /// Bounds of the text, should not be set most of the time.
        ///
        /// If not specified this is `UNBOUNDED`.
        pub bounds: Option<Vec2>,
        /// Sets if the text wraps.
        pub wrap: bool,
        /// Break line on, maybe use wrap instead.
        pub break_line_on: Option<BreakLineOn>,
    }
);

impl Widget for FrameBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        if self.layout.is_some() && self.dimension == DimensionType::Copied {
            self.dimension = DimensionType::Dynamic;
        }

        let mut base = commands.spawn_bundle(
            RectrayBundle {
                transform: Transform2D {
                    center: self.center,
                    anchor: self.anchor,
                    parent_anchor: self.parent_anchor.0,
                    offset: self.offset,
                    rotation: self.rotation,
                    scale: self.scale.0,
                    z: self.z
                },
                dimension: Dimension {
                    dimension: self.dimension,
                    font_size: self.font_size,
                    preserve_aspect: !matches!(self.aspect, Aspect::None)
                },
                opacity: self.opacity,
                clipping: Clipping::new(self.clipping.unwrap_or(false)),
                ..Default::default()
            }
        );
        if !self.name.is_empty() {
            base.insert(Name::new(self.name));
        }
        if !self.event.is_empty() {
            base.insert(self.event);
        }
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        } else if !self.event.is_empty() {
            base.insert(Hitbox::FULL);
        }
        if let Some(layer) = self.layer {
            base.insert(layer);
        }
        if let Some(layout) = self.layout {
            base.insert(Container {
                layout,
                margin: self.margin.0,
                padding: self.padding.0,
                range: self.children_range,
                maximum: usize::MAX,
            });
        }
        let base = base.id();
        (base, base)
    }
}

impl Widget for SpriteBuilder {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let sprite = commands.load_or_default(self.sprite);
        let mut frame = build_frame!(commands, self);
        let color = self.color.unwrap_or(bevy::prelude::Color::WHITE);
        frame.insert((
            Sprite {
                custom_size: self.size,
                color,
                rect: self.rect,
                flip_x: self.flip[0],
                flip_y: self.flip[1],
                ..Default::default()
            },
            sprite,
            Coloring::new(color),
            BuildTransformBundle::default(),
        ));
        (frame.id(), frame.id())
    }
}


impl Widget for RectangleBuilder {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let texture = Image::new(Extent3d {
            width: 1,
            height: 1,
            ..Default::default()
        }, TextureDimension::D2, vec![255, 255, 255, 255], BevyDefault::bevy_default(), RenderAssetUsages::RENDER_WORLD);
        let texture = commands.add_asset(texture);
        let color = self.color.unwrap_or(bevy::prelude::Color::WHITE);
        let frame = build_frame!(commands, self)
            .insert((
            Sprite {
                custom_size: self.size,
                color,
                ..Default::default()
            },
            Coloring::new(color),
            texture,
            BuildTransformBundle::default(),
        )).id();
        (frame, frame)
    }
}

impl Widget for TextBuilder {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let font = commands.load_or_default(self.font);
        let mut frame = build_frame!(commands, self);
        let color = self.color.unwrap_or(bevy::prelude::Color::WHITE);
        frame.insert((
            Text {
                sections: vec![TextSection::new(
                    self.text,
                    TextStyle {
                        font, color,
                        ..Default::default()
                    }
                )],
                linebreak_behavior: if let Some(b) = self.break_line_on {
                    b
                } else if self.wrap {
                    BreakLineOn::WordBoundary
                } else {
                    BreakLineOn::NoWrap
                },
                ..Default::default()
            },
            match self.bounds {
                Some(size) => Text2dBounds { size },
                None => Text2dBounds::UNBOUNDED,
            },
            TextLayoutInfo::default(),
            Into::<bevy::sprite::Anchor>::into(self.anchor),
            Coloring::new(color),
            BuildTransformBundle::default(),
        ));
        (frame.id(), frame.id())
    }
}

/// Construct an empty sprite. The underlying struct is [`FrameBuilder`].
#[macro_export]
macro_rules! frame {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {$($tt)*})};
}

/// Construct an image based sprite. The underlying struct is [`SpriteBuilder`].
#[macro_export]
macro_rules! sprite {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::SpriteBuilder] {$($tt)*})};
}

/// Construct a textbox. The underlying struct is [`TextBuilder`].
#[macro_export]
macro_rules! text {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::TextBuilder] {$($tt)*})};
}

/// Create a rectangle sprite with uniform color. The underlying struct is [`RectangleBuilder`].
#[macro_export]
macro_rules! rectangle {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::RectangleBuilder] {$($tt)*})};
}
