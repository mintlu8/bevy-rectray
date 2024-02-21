use bevy::ecs::entity::Entity;
use bevy::ecs::query::Has;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::text::{Font, Text};
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_rectray::dsl::OptionEx;
use bevy_defer::TypedSignal;
use bevy_rectray::util::{signal, ComposeExtension};
use bevy_rectray::widgets::button::ButtonClick;
use bevy_rectray::widgets::signals::ClearWidget;
use bevy_rectray::widgets::TextFragment;
use bevy_rectray::{Opacity, material_sprite, size2, color, inputbox, Anchor, text, Size2, rectangle, transition, frame};
use bevy_rectray::widgets::inputbox::{InputOverflow, InputBoxState, InputBoxCursorArea, InputBoxCursorBar, InputBoxText};
use bevy_rectray::{size, frame_extension, build_frame};
use bevy_rectray::anim::{Easing, Interpolate, Offset, Scale, VisibilityToggle};
use bevy_rectray::events::{EventFlags, CursorFocus};
use bevy_rectray::util::{Widget, RCommands, DslInto, convert::IntoAsset};
use crate::{StrokeColoring, build_shape};
use crate::shaders::RoundedRectangleMaterial;
use crate::style::Palette;
use super::ShadowInfo;
use super::util::StrokeColors;

#[derive(Debug, Clone, Copy, Component)]
pub struct PlaceHolderText {
    pub idle_color: Color,
    pub active_color: Color,
    pub points_to: Entity,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct DisplayIfHasText{
    pub points_to: Entity,
}

pub fn text_placeholder(
    mut input_box: Query<(
        &PlaceHolderText,
        &mut Interpolate<Color>,
        &mut Interpolate<Offset>,
        &mut Interpolate<Scale>,
        Has<InputBoxState>,
    )>,
    text_query: Query<(Option<&TextFragment>, Option<&Text>)>
) {
    for (placeholder, mut color, mut offset, mut scale, has) in input_box.iter_mut() {
        let has_text = has || match text_query.get(placeholder.points_to) {
            Ok((frag, text)) => 
                frag.map(|x| !x.text.is_empty()).unwrap_or(false) || 
                text.map(|x| x.sections.iter().any(|x| !x.value.is_empty())).unwrap_or(false),
            Err(_) => false,
        };
        if has_text {
            color.interpolate_to(placeholder.active_color);
            offset.interpolate_to(Vec2::new(0.8, 0.7));
            scale.interpolate_to(Vec2::new(0.8, 0.8));
        } else {
            color.interpolate_to(placeholder.idle_color);
            offset.interpolate_to(Vec2::new(0.8, 0.0));
            scale.interpolate_to(Vec2::new(1.0, 1.0));
        }
    }
}

pub fn display_if_has_text(
    mut display_if: Query<(&DisplayIfHasText, VisibilityToggle)>,
    text_query: Query<(Option<&TextFragment>, Option<&Text>)>
) {
    for (display, mut visibility) in display_if.iter_mut() {
        let has_text = match text_query.get(display.points_to) {
            Ok((frag, text)) => 
                frag.map(|x| !x.text.is_empty()).unwrap_or(false) || 
                text.map(|x| x.sections.iter().any(|x| !x.value.is_empty())).unwrap_or(false),
            Err(_) => false,
        };
        visibility.set_visible(has_text)
    }
}
/// A simple state machine that changes depending on status.
#[derive(Debug, Component, Clone, Copy)]
pub struct CursorStateColors {
    pub idle: Color,
    pub hover: Color,
    pub pressed: Color,
    pub disabled: Color,
}

impl Default for CursorStateColors {
    fn default() -> Self {
        Self {
            idle: Color::NONE,
            hover: Color::NONE,
            pressed: Color::NONE,
            disabled: Color::NONE
        }
    }
}

pub fn cursor_color_change(mut query: Query<(&CursorStateColors, &Opacity, Option<&CursorFocus>, &mut Interpolate<Color>)>) {
    query.iter_mut().for_each(|(colors, opacity, focus, mut color)| {
        if opacity.is_disabled() {
            color.interpolate_to(colors.disabled);
            return;
        }
        match focus {
            Some(focus) if focus.is(EventFlags::Hover)=> color.interpolate_to(colors.hover),
            Some(focus) if focus.intersects(EventFlags::LeftPressed|EventFlags::LeftDrag)
                => color.interpolate_to(colors.pressed),
            _ => color.interpolate_to(colors.idle),
        }
    })
}


pub fn cursor_stroke_change(mut query: Query<(&StrokeColors<CursorStateColors>, &Opacity, Option<&CursorFocus>, &mut Interpolate<StrokeColoring>)>) {
    query.iter_mut().for_each(|(colors, opacity, focus, mut color)| {
        if opacity.is_disabled() {
            color.interpolate_to(colors.disabled);
            return;
        }
        match focus {
            Some(focus) if focus.is(EventFlags::Hover)=> color.interpolate_to(colors.hover),
            Some(focus) if focus.is(EventFlags::LeftPressed)=> color.interpolate_to(colors.pressed),
            _ => color.interpolate_to(colors.idle),
        }
    })
}

/// A simple state machine that changes depending on status.
#[derive(Debug, Component, Clone, Copy)]
pub struct InputStateColors {
    pub idle: Color,
    pub focused: Color,
    pub disabled: Color,
}

frame_extension!(
    pub struct MInputBuilder {
        pub placeholder: String,
        pub text: String,
        /// Width of text, in em.
        pub width: f32,
        pub font: IntoAsset<Font>,
        pub texture: IntoAsset<Image>,
        pub stroke: f32,
        pub capsule: bool,
        pub radius: f32,
        pub shadow: OptionEx<ShadowInfo>,
        pub on_change: TypedSignal<String>,
        pub on_submit: TypedSignal<String>,
        pub overflow: InputOverflow,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
        pub palette: Palette,
        pub focus_palette: Option<Palette>,
        pub disabled_palette: Option<Palette>,
        pub cancel: Option<Entity>,
        pub bottom_bar: Option<f32>,
    }
);

impl Widget for MInputBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftDrag;

        self.dimension = size2!({self.width} em, 2.8 em).dinto();
        let style = self.palette;
        let focus_style = self.focus_palette.unwrap_or(style);
        let disabled_style = self.disabled_palette.unwrap_or(style);

        let entity = build_frame!(commands, self).id();
        let text_area;
        let input_box = inputbox!(commands {
            color: style.foreground(),
            text: &self.text,
            overflow: self.overflow,
            dimension: Size2::FULL,
            font: self.font.clone(),
            width: size!(1 - 1.6 em),
            z: 0.01,
            extra: InputStateColors {
                idle: style.background(),
                focused: focus_style.background(),
                disabled: disabled_style.background(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                style.background(),
                0.15
            ),
            cursor_bar: frame! {
                z: 0.005,
                dimension: size2!(0.15 em, 1.2 em),
                extra: RoundedRectangleMaterial::capsule(style.foreground())
                    .into_bundle(commands),
                extra: InputBoxCursorBar,
            },
            cursor_area: frame! {
                z: -0.005,
                dimension: size2!(0, 1.2 em),
                extra: RoundedRectangleMaterial::new(color!(green300), 2.0)
                    .into_bundle(commands),
                extra: InputBoxCursorArea,
            },
            text_area: rectangle! {
                entity: text_area,
                z: 0.01,
                offset: size2!(0.8 em, {if self.placeholder.is_empty() {
                    0.0
                } else {
                    -0.4
                }} em),
                color: style.foreground(),
                anchor: Anchor::CENTER_LEFT,
                extra: InputBoxText,
                extra: TextFragment::new(self.text)
                    .with_font(commands.load_or_default(self.font.clone()))
            }
        });

        if let Some(cancel) = self.cancel {
            let (cancel_send, cancel_recv) = signal();
            commands.entity(cancel).insert((
                DisplayIfHasText { points_to: text_area},
            )).add_sender::<ButtonClick>(cancel_send);
            commands.entity(entity).add_child(cancel);
            commands.entity(input_box).add_receiver::<ClearWidget>(cancel_recv);
        }

        build_shape!(commands, self, input_box);
        let has_placeholder = !self.placeholder.is_empty();
        if has_placeholder {
            let placeholder = text!(commands {
                anchor: Anchor::CENTER_LEFT,
                center: Anchor::CENTER_LEFT,
                offset: size2!(0.8 em, 0 em),
                font: self.font.clone(),
                text: self.placeholder,
                extra: PlaceHolderText {
                    idle_color: style.foreground(),
                    active_color: focus_style.foreground(),
                    points_to: text_area,
                },
                extra: transition!(
                    Color 0.15 Linear default {self.palette.foreground()};
                    Offset 0.15 Linear default {Vec2::ZERO};
                    Scale 0.15 Linear default {Vec2::ONE};
                )
            });
            commands.entity(input_box).add_child(placeholder);
        }
        if let Some(bottom_bar) = self.bottom_bar {
            let bottom_bar = material_sprite!(commands {
                parent_anchor: Anchor::BOTTOM_CENTER,
                dimension: size2!(100%, bottom_bar em),
                material: RoundedRectangleMaterial::capsule(color!(black)),
            });
            commands.entity(input_box).add_child(bottom_bar);
        }

        commands.entity(entity).add_child(input_box);
        (entity, input_box)
    }
}

#[macro_export]
macro_rules! minput {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MInputBuilder] {
            $($tt)*
        })
    };
}
