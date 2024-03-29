use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;

use bevy::text::Font;
use bevy::window::CursorIcon;
use bevy_defer::Object;
use bevy_defer::signals::{TypedSignal, Signals};
use crate::util::ComposeExtension;
use crate::widgets::TextFragment;
use crate::widgets::button::{Payload, Button, CheckButton, RadioButton, RadioButtonCancel, ButtonClick, ToggleChange};
use crate::widgets::util::{SetCursor, PropagateFocus};
use crate::{build_frame, Anchor, rectangle, Size, size};
use crate::events::EventFlags;
use crate::frame_extension;
use crate::widgets::inputbox::{InputOverflow, InputBoxText, TextSubmit, TextChange};
use crate::widgets::inputbox::{InputBox, InputBoxCursorBar, InputBoxCursorArea};

use crate::util::{Widget, RCommands, convert::IntoAsset};

frame_extension!(
    pub struct InputBoxBuilder {
        pub text: String,
        pub font: IntoAsset<Font>,
        pub width: Option<Size>,
        pub text_area: Option<Entity>,
        pub cursor_bar: Option<Entity>,
        pub cursor_area: Option<Entity>,
        pub on_change: Option<TypedSignal<String>>,
        pub on_submit: Option<TypedSignal<String>>,
        pub overflow: InputOverflow,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
    }
);

impl Widget for InputBoxBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::DoubleClick|EventFlags::LeftDrag|EventFlags::ClickOutside;
        let font = commands.load_or_default(self.font);

        let mut entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            InputBox::new(&self.text, self.overflow)
                .with_width(self.width.unwrap_or(size!(100%))),
            //TextColor(self.color.expect("color is required.")),
            font.clone(),
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftDrag,
                icon: self.cursor_icon.unwrap_or(CursorIcon::Text),
            },
        ));
        entity.compose2(
            self.on_change.map(Signals::from_sender::<TextChange>),
            self.on_submit.map(Signals::from_sender::<TextSubmit>)
        );
        let entity = entity.id();
        let text_area = self.text_area.unwrap_or(
            rectangle!(commands {
                color: self.color.expect("color is required."),
                anchor: Anchor::CENTER_LEFT,
                extra: InputBoxText,
                extra: TextFragment {
                    text: self.text,
                    font,
                    size: 0.0
                }
            })
        );
        let bar = commands.entity(self.cursor_bar.expect("cursor_bar is required."))
            .insert(InputBoxCursorBar)
            .id();
        let area = commands.entity(self.cursor_area.expect("cursor_area is required."))
            .insert(InputBoxCursorArea)
            .id();
        commands.entity(text_area).add_child(bar);
        commands.entity(text_area).add_child(area);
        commands.entity(entity).add_child(text_area);
        (entity, entity)
    }
}
/// Construct a `input_box`. The underlying struct is [`InputBoxBuilder`].
#[macro_export]
macro_rules! inputbox {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::InputBoxBuilder] {$($tt)*})};
}

frame_extension!(
    pub struct ButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// Sends a signal whenever the button is clicked.
        pub on_click: Option<TypedSignal<Object>>,
        /// If set, `submit` sends its contents.
        pub payload: Option<Payload>,
    }
);

impl Widget for ButtonBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftClick;
        let mut entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            Button,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Pointer),
            },
        ));
        if let Some(payload) = self.payload  {
            entity.insert(payload);
        }
        if let Some(click) = self.on_click {
            entity.compose(Signals::from_sender::<ButtonClick>(click));
        }
        let entity = entity.id();
        (entity, entity)
    }
}

frame_extension!(
    pub struct CheckButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// If set, `submit` sends its contents.
        pub payload: Option<Payload>,
        /// Sends a signal whenever the button is clicked and its value is `true`.
        ///
        /// Like button, this sends either `()` or `Payload`.
        pub on_checked: Option<TypedSignal<Object>>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub on_change: Option<TypedSignal<bool>>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,
    }
);

impl Widget for CheckButtonBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftClick;
        let mut  entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            CheckButton::from(self.checked),
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Pointer),
            },
        ));
        if let Some(payload) = self.payload  {
            entity.insert(payload);
        }
        entity.compose2(
            self.on_change.map(Signals::from_sender::<ToggleChange>),
            self.on_checked.map(Signals::from_sender::<ButtonClick>),
        );
        let entity = entity.id();
        (entity, entity)
    }
}

frame_extension!(
    pub struct RadioButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// The context for the radio button's value.
        pub context: Option<RadioButton>,
        /// If true, behave like a `CheckButton` and set context to `None` if already checked.
        pub cancellable: bool,
        /// Discriminant for this button's value, must be comparable.
        pub value: Option<Payload>,
        /// Sends a signal whenever the button is clicked.
        pub on_click: Option<TypedSignal<Object>>,
    }
);

impl Widget for RadioButtonBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftClick;
        let mut entity = build_frame!(commands, self);

        entity.insert((
            PropagateFocus,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Pointer),
            },
            self.context.expect("Expected RadioButton context."),
            self.value.expect("Expected RadioButton value."),
        ));
        if self.cancellable {
            entity.insert(RadioButtonCancel);
        }
        if let Some(click) = self.on_click {
            entity.compose(Signals::from_sender::<ButtonClick>(click));
        }
        let entity = entity.id();
        (entity, entity)
    }
}

/// Construct a button. The underlying struct is [`ButtonBuilder`].
///
/// # Features
///
/// `button` is a widget primitive with no default look. You need to nest
/// `sprite` or `text` as children to make `button` function properly.
///
/// These are what `button` does compared to `frame`:
///
/// * Add event listeners for `Hover` and `Click`
/// * Change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its descendants.
/// * Allow usage of `EvButtonClick` event. Which uses the button's [`Payload`].
///
/// You can use signals to handle clicks
/// and use [`DisplayIf`](crate::widgets::util::DisplayIf)
/// or [`Interpolate`](crate::anim::Interpolate) for simple UI interaction.
///
/// # Common Pitfall
///
/// Do not nest `button`, `check_button` or `radio_button` inside a button.
/// Button propagates its state to all its descendants and can inject unwanted state.
/// Introduce a common parent instead.
#[macro_export]
macro_rules! button {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ButtonBuilder] {$($tt)*})};
}


/// Construct a `check_button`. The underlying struct is [`CheckButtonBuilder`].
///
/// # Features
///
/// `check_button` is a widget primitive with no default look. You need to nest
/// `sprite` or `text` as children to make `check_button` function properly.
///
/// These are what `check_button` does compared to `frame`:
///
/// * Add event listeners for `Hover` and `Click`
/// * Change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its descendants.
/// * Hold a boolean context value for if the button is checked or not.
/// * Generate `CheckButtonState` based on the context.
/// * Allow usage of `EvButtonClick` event. Which uses the button's [`Payload`].
///
/// You can use signals to handle clicks
/// and use [`DisplayIf`](crate::widgets::util::DisplayIf)
/// or [`Interpolate`](crate::anim::Interpolate) for simple UI interaction.
///
/// # Common Pitfall
///
/// Do not nest `button`, `check_button` or `radio_button` inside a button.
/// Button propagates its state to all its descendants and can inject unwanted state.
/// Introduce a common parent instead.
#[macro_export]
macro_rules! check_button {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::CheckButtonBuilder] {$($tt)*})};
}


/// Construct a `radio_button`. The underlying struct is [`RadioButtonBuilder`].
///
/// This is in fact very versatile and can be used for any exclusive UI elements
/// like a dropdown select or an accordion.
///
/// # Features
///
/// `radio_button` is a widget primitive with no default look. You need to nest
/// `sprite` or `text` as children to make `radio_button` function properly.
///
/// These are what `radio_button` does compared to `frame`:
///
/// * Add event listeners for `Hover` and `Click`
/// * Change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its descendants.
/// * Hold a [`Payload`] value as a discriminant.
/// * Generate `CheckButtonState` based on the context and payload.
/// * Send payload value through `EvButtonClick`.
///
/// You can use signals to handle clicks
/// and use [`DisplayIf`](crate::widgets::util::DisplayIf)
/// or [`Interpolate`](crate::anim::Interpolate) for simple UI interaction.
///
/// # Common Pitfall
///
/// Do not nest `button`, `check_button` or `radio_button` inside a button.
/// Button propagates its state to all its descendants and can inject unwanted state.
/// Introduce a common parent instead.
#[macro_export]
macro_rules! radio_button {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::RadioButtonBuilder] {$($tt)*})};
}
