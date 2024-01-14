//! Demo for the `quote!` syntax.

use bevy::prelude::*;
use bevy_aoui::{AouiPlugin, util::AouiCommands};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .run();
}

pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());
    let directions = [PI, 0.0, PI, 0.0, PI, 0.0];
    let colors = colors!(blue100, blue200, blue300, blue400, blue500, blue600, blue700, blue800, blue900);
    let rotations = [-0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4];

    hstack! (commands {
        child: #vstack! {
            rotation: #directions,
            child: #rectangle! {
                dimension: [40, 20],
                color: #colors,
                rotation: #rotations,
                z: -1,
                child: text! {
                    text: format!("{:.2}", #rotations),
                    color: color!(black),
                }
            },
        },
    });
}