use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rectray::{util::RCommands, RectrayPlugin};

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
        .add_plugins(RectrayPlugin)
        // classic macos stuff
        .run();
}


pub fn init(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        system: |fps: Fps, text: Ac<Text>| {
            let fps = fps.get().await;
            text.set(move |text| format_widget!(text, "FPS: {:.2}", fps)).await?;
        }
    });

    let (send1, recv1) = signal();

    text! (commands {
        offset: [-400, 200],
        color: color!(gold),
        text: "Scroll this! =>",
        signal: receiver::<PositionFac>(recv1),
        system: |x: Receiver<PositionFac>, text: Ac<Text>| {
            let s = x.recv().await;
            text.write(format!("This has value {:.2}! =>", s)).await?;
        },
    });

    sprite! (commands {
        dimension: [200, 60],
        offset: [-200, 200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::X,
            signal: sender::<PositionFac>(send1),
            dimension: [60, 60],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [0, 200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::BOTH,
            dimension: [60, 150],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });
    sprite! (commands {
        dimension: [60, 100],
        offset: [200, 200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::X,
            dimension: [200, 60],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });
    sprite! (commands {
        dimension: [200, 200],
        offset: [-200, 0],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::BOTH,
            dimension: [60, 60],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });
    sprite! (commands {
        dimension: [200, 200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: frame! {
            extra: Scrolling::BOTH,
            dimension: [150, 150],
            child: sprite! {
                dimension: [60, 60],
                anchor: Left,
                sprite: commands.load("square.png"),
                color: color!(red),
            },
            child: sprite! {
                dimension: [60, 60],
                anchor: Bottom,
                sprite: commands.load("square.png"),
                color: color!(red),
            },
            child: sprite! {
                dimension: [60, 60],
                anchor: TopRight,
                sprite: commands.load("square.png"),
                color: color!(red),
            }
        }
    });
    sprite! (commands {
        dimension: [100, 100],
        offset: [200, 0],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        extra: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::BOTH,
            dimension: [200, 200],
            z: -1,
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });

    let (send2, recv2) = signal();

    text! (commands {
        offset: [-400, -200],
        color: color!(gold),
        text: "Scroll this! =>",
        signal: receiver::<PositionFac>(recv2),
        system: |x: Receiver<PositionFac>, text: Ac<Text>| {
            let s = x.recv().await;
            text.write(format!("This has value {:.2}! =>", s)).await?;
        },
    });

    sprite! (commands {
        dimension: [60, 200],
        offset: [-200, -200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::Y,
            signal: sender::<PositionFac>(send2),
            dimension: [60, 60],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });

    sprite! (commands {
        dimension: [100, 100],
        offset: [0, -200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::BOTH,
            dimension: [150, 60],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });

    sprite! (commands {
        dimension: [100, 60],
        offset: [200, -200],
        hitbox: Hitbox::rect(1),
        sprite: commands.load("square.png"),
        event: EventFlags::MouseWheel,
        extra: ScrollParent,
        child: sprite! {
            extra: Scrolling::Y,
            dimension: [60, 200],
            sprite: commands.load("square.png"),
            color: color!(red),
        }
    });
}
