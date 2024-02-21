use bevy::ecs::entity::Entity;

use crate::{layout::*, build_frame};

/// Construct a dummy entity for linebreak in a layout.
#[macro_export]
macro_rules! linebreak {
    (($commands: expr $(, $tt:expr)*) $({})? $(,)?) => {
        $commands.spawn_bundle($crate::bundles::LinebreakBundle::default()).id()
    };
    (($commands: expr $(, $tt:expr)*), $size: expr $(,)?) => {
        {
            let OneOrTwo(size) = $crate::util::DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn_bundle($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
    (($commands: expr $(, $tt:expr)*) {$size: expr}) => {
        {
            let OneOrTwo(size) = $crate::util::DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn_bundle($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
    ($commands: tt $({})? $(,)?) => {
        $commands.spawn_bundle($crate::bundles::LinebreakBundle::default()).id()
    };
    ($commands: tt $size: expr $(,)?) => {
        {
            let OneOrTwo(size) = $crate::util::DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn_bundle($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
    ($commands: tt {$size: expr}) => {
        {
            let OneOrTwo(size) = $crate::util::DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn_bundle($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
}

use crate::frame_extension;

use crate::util::{Widget, RCommands};


frame_extension! {
    pub struct PaddingBuilder {}
}

impl Widget for PaddingBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.layout = Some(BoundsLayout::PADDING.into());
        let entity = build_frame!(commands, self).id();
        (entity, entity)
    }
}

/// Construct a `BoundsLayout`, commonly used for padding. The Underlying struct is [`PaddingBuilder`].
#[macro_export]
macro_rules! padding {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::PaddingBuilder] {
            $($tt)*
        })
    };
}

/// Construct a horizontal left to right compact layout.
/// The Underlying struct is [`FrameBuilder`](super::builders::FrameBuilder).
#[macro_export]
macro_rules! hstack {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::StackLayout::HSTACK,
            $($tt)*
        })
    };
}

/// Construct a vertical top to bottom compact layout.
/// The Underlying struct is [`FrameBuilder`](super::builders::FrameBuilder).
#[macro_export]
macro_rules! vstack {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::StackLayout::VSTACK,
            $($tt)*
        })
    };
}

/// Construct a horizotal left to right layout with fixed dimension.
/// The Underlying struct is [`FrameBuilder`](super::builders::FrameBuilder).
#[macro_export]
macro_rules! hbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::SpanLayout::HBOX,
            $($tt)*
        })
    };
}

/// Construct a vertical top to bottom layout with fixed dimension.
/// The Underlying struct is [`FrameBuilder`](super::builders::FrameBuilder).
#[macro_export]
macro_rules! vbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::SpanLayout::VBOX,
            $($tt)*
        })
    };
}

/// Construct a paragraph layout.
/// The Underlying struct is [`FrameBuilder`](super::builders::FrameBuilder).
#[macro_export]
macro_rules! paragraph {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::ParagraphLayout::PARAGRAPH,
            $($tt)*
        })
    };
}
