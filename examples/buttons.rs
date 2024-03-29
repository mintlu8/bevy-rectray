//! Showcases the features of a button widget.
#![recursion_limit = "256"]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_defer::{world, AsyncExtension};
use bevy_defer::{spawn, Object, signals::Signal};
use bevy_rectray::widgets::signals::Fac;
use bevy_rectray::RectrayPlugin;
use bevy_rectray::util::RCommands;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .add_systems(Update, recv)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(RectrayPlugin)
        .spawn_task(async {
            let world = world();
            loop {
                let message = world.poll::<Fac<String>>("Clicked").await;
                println!("{}", message);
            }
        })
        .run();
}

#[derive(Debug, Component)]
pub struct Listen(Signal<Object>);

pub fn recv(query: Query<&Listen>) {
    for item in query.iter() {
        if item.0.try_read().is_some() {
            println!("Signal is received!");
        }
    }
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
    let (send2, recv2) = signal();

    vstack!(commands {
        offset: [0, 100],
        child: check_button! {
            dimension: size2!(14 em, 2 em),
            checked: true,
            on_change: send1,
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "check.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: text!{
                anchor: Left,
                offset: size2!(2.5 em, 0),
                text: "This is a checkbox",
            },
        },
        child: check_button! {
            dimension: size2!(14 em, 2 em),
            on_change: send2,
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "check.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: text!{
                anchor: Left,
                offset: size2!(2.5 em, 0),
                text: "This is also a checkbox",
            },
        }
    });

    text! (commands {
        offset: [300, 120],
        color: color!(gold),
        text: "<= true!",
        signal: receiver::<ToggleChange>(recv1),
        system: |toggle: Receiver<ToggleChange>, text: Aeq<&mut Text>| {
            let checked = toggle.recv().await;
            let _ = text.run(move |mut text| format_widget!(text.as_mut(), "<= {}!", checked)).await;
        }
    });

    text! (commands {
        offset: [300, 80],
        color: color!(gold),
        text: "<= false!",
        signal: receiver::<ToggleChange>(recv2),
        system: |x: Receiver<ToggleChange>, text: Aeq<&mut Text>| {
            let checked = x.recv().await;
            let _ = text.run(move |mut text| format_widget!(text.as_mut(), "<= {}!", checked)).await;
        }
    });

    let ctx = radio_button_group::<[_; 4]>("Fire");
    let sig = ctx[0].recv();
    let elements = ["Fire", "Water", "Earth", "Wind"];

    text! (commands {
        offset: [300, -100],
        color: color!(gold),
        text: "<= Click the radio button and this will change!",
        signal: receiver::<FormatTextStatic>(sig),
        system: |x: Receiver<FormatTextStatic>, text: Ac<Text>| {
            let checked = x.recv().await;
            text.set(move |text| format_widget!(text, "<= {}!", checked)).await?;
        }
    });

    vstack!(commands {
        offset: [0, -150],
        child: #radio_button! {
            dimension: size2!(14 em, 2 em),
            context: #ctx,
            value: #elements,
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "radio.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: text!{
                anchor: Left,
                offset: size2!(2.5 em, 0),
                text: #elements,
            },
        },
    });

    let (send, recv, recv2) = signal::<Object, _>();

    commands.spawn_bundle(Listen(Signal::from(recv2)));

    text! (commands {
        offset: [300, 0],
        color: color!(gold),
        text: "<= Click this button.",
        signal: receiver::<Invocation>(recv),
        system: |sig: Receiver<Invocation>, text: Aeq<&mut Text>| {
            sig.recv().await;
            let _ = text.run(|text| format_widget!(text, "You clicked it!")).await;
            spawn(async { 
                println!("Hello from spawned task!");
            }).await;
        }
    });

    button! (commands {
        dimension: size2!(12 em, 2 em),
        font_size: em(2),
        cursor: CursorIcon::Pointer,
        on_click: send,
        child: rectangle!{
            dimension: size2!(100%, 100%),
            color: color!(blue500),
            extra: DisplayIf(EventFlags::Idle)
        },
        child: text!{
            text: "Click Me!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Idle),
            z: 0.1
        },
        child: rectangle!{
            dimension: size2!(100%, 100%),
            color: color!(blue800),
            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed)
        },
        child: text!{
            text: "Hovering!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Hover),
            z: 0.1
        },
        child: text!{
            text: "Clicked!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::LeftPressed),
            z: 0.1
        },
        system: |sender: Sender<ButtonClick>, world: AsyncWorldMut| {
            sender.recv().await;
            println!("Clicked");
            world.send::<Fac<String>>("Clicked", "!!!!".to_owned()).await;
        }
    });
}
