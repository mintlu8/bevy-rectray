//! Showcases support for dragging and interpolation.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin, sprite::{Material2dPlugin, Material2d}, render::render_resource::AsBindGroup};
use bevy_aoui::{AouiPlugin, util::{WorldExtension, AouiCommands}};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .add_plugins(Material2dPlugin::<Circle>::default())
        .register_cursor_default(CursorIcon::Arrow)
        .run();
}

#[derive(Debug, Default, Clone, AsBindGroup, TypePath, Asset)]
pub struct Circle{
    #[uniform(0)]
    fill: Color,
    #[uniform(1)]
    stroke: Color,
}

impl Material2d for Circle {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "stroke_circle.wgsl".into()
    }
}

pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    material_sprite! (commands {
        dimension: [100, 100],
        hitbox: Hitbox::rect(1),
        z: 10,
        material: Circle {
            fill: Color::RED,
            stroke: Color::BLACK
        },
        event: EventFlags::Hover|EventFlags::LeftDrag,
        extra: DragBoth.with_snap_back(),
        extra: SetCursor {
            flags: EventFlags::Hover|EventFlags::LeftDrag,
            icon: CursorIcon::Hand,
        },
        extra: transition!(Offset 4.0 BounceOut default Vec2::ZERO),
    });

    let (send1, recv1) = commands.signal();

    rectangle!(commands {
        dimension: [400, 50],
        offset: [0, 100],
        child: rectangle! {
            dimension: [50, 50],
            anchor: Right,
            center: Center,
            color: color!(aqua),
            event: EventFlags::Hover|EventFlags::LeftDrag,
            extra: SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftDrag,
                icon: CursorIcon::Hand,
            },
            extra: DragX.with_handler(
                Handlers::new(send1).and_mutate(
                    |fac: f32, transform: &mut Transform2D, dim: &mut Dimension| {
                        transform.rotation = fac * 2.0 * PI;
                        dim.edit_raw(|v| v.y = 50.0 + (1.0 - fac) * 50.0)
                    }
                )
            ),
        }
    });

    text! (commands {
        offset: [300, 100],
        color: color!(gold),
        text: "<= Drag and this will change!",
        extra: recv1.recv(|x: f32, text: &mut Text| format_widget!(text, "<= has value {:.2}!", x))
    });

    let (send2, recv2) = commands.signal();
    let (send3, recv3) = commands.signal();

    rectangle!(commands {
        dimension: [400, 50],
        offset: [0, -100],
        child: rectangle! {
            dimension: [50, 50],
            anchor: Left,
            color: color!(aqua),
            extra: DragX
                .with_recv(recv2)
                .with_handler(send3),
        }
    });

    material_sprite! (commands {
        dimension: [100, 100],
        offset: [-300, -100],
        hitbox: Hitbox::rect(1),
        event: EventFlags::Hover|EventFlags::LeftDrag,
        material: Circle {
            fill: color!(aqua),
            stroke: color!(blue),
        },
        extra: SetCursor {
            flags: EventFlags::Hover|EventFlags::LeftDrag,
            icon: CursorIcon::Hand,
        },
        extra: Handlers::<EvMouseDrag>::new(send2),
    });

    text! (commands {
        offset: [300, -100],
        color: color!(gold),
        text: "<= Drag and this will change!",
        extra: recv3.recv(|x: f32, text: &mut Text| format_widget!(text, "<= has value {:.2}!", x))
    });
}
