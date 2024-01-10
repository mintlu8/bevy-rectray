use bevy::{prelude::*, window::{Window, PrimaryWindow}};

use super::{*, cursor::CameraQuery, wheel::MouseWheelAction};


/// Remove [`CursorFocus`], [`CursorAction`], [`CursorClickOutside`] and [`Submit`];
pub fn remove_focus(mut commands: Commands,
    query1: Query<Entity, With<CursorFocus>>,
    query2: Query<Entity, With<CursorAction>>,
    query3: Query<Entity, With<CursorClickOutside>>,
    query4: Query<Entity, With<MouseWheelAction>>,
) {
    for entity in query1.iter() {
        commands.entity(entity).remove::<CursorFocus>();
    }
    for entity in query2.iter() {
        commands.entity(entity).remove::<CursorAction>();
    }
    for entity in query3.iter() {
        commands.entity(entity).remove::<CursorClickOutside>();
    }
    for entity in query4.iter() {
        commands.entity(entity).remove::<MouseWheelAction>();
    }
}

trait OptionDo<T> {
    fn exec(self, f: impl FnOnce());
    fn exec_with(self, f: impl FnOnce(T));
}

impl<T> OptionDo<T> for Option<T> {
    fn exec(self, f: impl FnOnce()) {
        if self.is_some() {
            f()
        }
    }
    fn exec_with(self, f: impl FnOnce(T)) {
        if let Some(val) = self {
            f(val)
        }
    }
}

trait End: Sized {
    fn end(self) {}
}

impl<T> End for T {}

/// We hand out component [`CursorFocus`] for persistant states,
/// [`CursorAction`] for active events.
/// and [`CursorClickOutside`] for cancelling.
/// These should be handled on this frame during [`Update`].
pub fn mouse_button_input(
    mut commands: Commands,
    mut state: ResMut<CursorState>,
    time: Res<Time>,
    double_click: Res<DoubleClickThreshold>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: CameraQuery,
    query: Query<(Entity, &EventFlags, CursorDetection, ActiveDetection)>,
) {
    let iter = |f: EventFlags|query.iter().filter_map(move |(entity, flag, cursor, detection)| {
        if detection.is_active() && flag.intersects(f) {
            Some((entity, flag, cursor))
        } else {
            None
        }
    });
    state.caught = false;
    state.focused = None;
    if state.blocked { return; }
    let Ok(window) = windows.get_single() else { return };
    let Some(mouse_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(cursor))
    else {return;};
    state.cursor_pos = mouse_pos;
    if state.dragging {
        state.caught = true;
        if let Some(mut entity) = state.drag_target(&mut commands) {
            state.focused = Some(entity.id());
            if !buttons.pressed(state.drag_button) {
                if state.drag_dbl_click && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                    entity.insert(CursorAction(EventFlags::DoubleClick));
                    entity.insert(CursorFocus(EventFlags::Hover));
                    state.clear_dbl_click();
                } else {
                    entity.insert(CursorAction(EventFlags::DragEnd));
                    entity.insert(CursorFocus(EventFlags::Hover));
                }
                state.dragging = false;
                state.drag_target = None;
                let dragged_id = entity.id();
                iter(EventFlags::Drop)
                    .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
                    .max_by(|(.., a), (.., b)| a.z().total_cmp(&b.z()))
                    .exec_with(|(entity, ..)| commands.entity(entity).insert(CursorAction(EventFlags::Drop)).end());
                iter(EventFlags::ClickOutside)
                    .filter(|(e, ..)| e != &dragged_id)
                    .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                    .for_each(|(entity, ..)| commands.entity(entity).insert(CursorClickOutside).end());
            } else {
                if state.drag_button != MouseButton::Left && buttons.just_pressed(MouseButton::Left) {
                    entity.insert(CursorAction(EventFlags::LeftDown));
                } else if state.drag_button != MouseButton::Right && buttons.just_pressed(MouseButton::Right) {
                    entity.insert(CursorAction(EventFlags::RightDown));
                } else if state.drag_button != MouseButton::Middle && buttons.just_pressed(MouseButton::Middle) {
                    entity.insert(CursorAction(EventFlags::MidDown));
                }
                entity.insert(CursorFocus(match state.drag_button {
                    MouseButton::Left => EventFlags::LeftDrag,
                    MouseButton::Right => EventFlags::RightDrag,
                    MouseButton::Middle => EventFlags::MidDrag,
                    MouseButton::Other(_) => EventFlags::LeftDrag,
                }));
            }
        } else if !buttons.pressed(state.drag_button) {
            state.dragging = false;
            state.drag_target = None;
            iter(EventFlags::ClickOutside)
            .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
            .for_each(|(entity, ..)| commands.entity(entity).insert(CursorClickOutside).end());
        }
    } else if buttons.pressed(MouseButton::Left) {
        if buttons.just_pressed(MouseButton::Left) {
            state.down_pos = mouse_pos;
            let [_, last] = state.last_lmb_down_time;
            state.last_lmb_down_time = [last, time.elapsed_seconds()];
        }
        if let Some((entity, flag)) = iter(EventFlags::LeftDrag|EventFlags::LeftClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, flags, _)| (entity, flags)
            ) {
            state.caught = true;
            if buttons.just_pressed(MouseButton::Left) {
                commands.entity(entity).insert(CursorAction(EventFlags::LeftDown));
                if flag.contains(EventFlags::LeftDrag) {
                    state.drag_target = Some(entity);
                    state.dragging = true;
                    state.drag_button = MouseButton::Left;
                    state.drag_dbl_click = flag.contains(EventFlags::DoubleClick);
                    commands.entity(entity).insert(CursorFocus(EventFlags::LeftDrag));
                    state.focused = Some(entity);
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::LeftPressed));
                    state.focused = Some(entity);
                }
            } else if flag.contains(EventFlags::LeftClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::LeftPressed));
                state.focused = Some(entity);
            }
        }
    } else if buttons.pressed(MouseButton::Right) {
        if buttons.just_pressed(MouseButton::Right) {
            state.down_pos = mouse_pos
        }
        if let Some((entity, flag)) = iter(EventFlags::RightDrag|EventFlags::RightClick)
            .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
            .max_by(|(.., a), (.., b)| a.compare(b))
            .map(|(entity, flags, _)| (entity, flags)
        ) {
            state.caught = true;
            if buttons.just_pressed(MouseButton::Right) {
                commands.entity(entity).insert(CursorAction(EventFlags::RightDown));
                if flag.contains(EventFlags::RightDrag) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Right;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::RightDrag));
                    state.focused = Some(entity);
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::RightPressed));
                    state.focused = Some(entity);
                }
            } else if flag.contains(EventFlags::RightClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::RightPressed));
                state.focused = Some(entity);
            }
        }
    } else if buttons.pressed(MouseButton::Middle) {
        if buttons.just_pressed(MouseButton::Middle) {
            state.down_pos = mouse_pos
        }
        if let Some((entity, flag)) = iter(EventFlags::MidDrag|EventFlags::MidClick)
            .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
            .max_by(|(.., a), (.., b)| a.compare(b))
            .map(|(entity, flags, _)| (entity, flags)
        ) {
            state.caught = true;
            if buttons.just_pressed(MouseButton::Middle) {
                state.down_pos = mouse_pos;
                commands.entity(entity).insert(CursorAction(EventFlags::MidDown));
                if flag.contains(EventFlags::MidDrag) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Middle;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::MidDrag));
                    state.focused = Some(entity);
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::MidPressed));
                    state.focused = Some(entity);
                }
            } else if flag.contains(EventFlags::MidClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::MidPressed));
                state.focused = Some(entity);
            }
        }
    } else {
        if buttons.just_released(MouseButton::Left) {
            let down = state.down_pos;
            iter(EventFlags::LeftClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos) && hitbox.contains(down))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, flags, _)|
                    if flags.contains(EventFlags::DoubleClick) && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                        commands.entity(entity).insert(CursorAction(EventFlags::DoubleClick));
                        state.clear_dbl_click();
                    } else {
                        commands.entity(entity).insert(CursorAction(EventFlags::LeftClick));
                    }
                )
                .exec(|| state.caught = true);
            iter(EventFlags::ClickOutside)
                .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                .for_each(|(entity, ..)| commands.entity(entity).insert(CursorClickOutside).end());
        } else if buttons.just_released(MouseButton::Right) {
            let down = state.down_pos;
            iter(EventFlags::RightClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos) && hitbox.contains(down))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, ..)| commands.entity(entity).insert(CursorAction(EventFlags::RightClick)).end())
                .exec(|| state.caught = true);
            iter(EventFlags::ClickOutside)
                .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                .for_each(|(entity, ..)| commands.entity(entity).insert(CursorClickOutside).end());
        } else if buttons.just_released(MouseButton::Middle) {
            let down = state.down_pos;
            iter(EventFlags::MidClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos) && hitbox.contains(down))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, ..)| commands.entity(entity).insert(CursorAction(EventFlags::MidClick)).end())
                .exec(|| state.caught = true);
            iter(EventFlags::ClickOutside)
                .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                .for_each(|(entity, ..)| commands.entity(entity).insert(CursorClickOutside).end());
        }
        if state.focused.is_none() {
            iter(EventFlags::Hover)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, ..)| {
                    commands.entity(entity).insert(CursorFocus(EventFlags::Hover)).end();
                    state.focused = Some(entity);
                })
                .exec(|| state.caught = true);
        }
    }
}
