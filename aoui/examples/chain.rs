//! Showcases a simple skeletal system by chaining rectangles.

use bevy_aoui::{*, bundles::*, util::AouiCommands};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{Slider, self}};
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_egui::EguiPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, egui_window)
        .add_plugins(AouiPlugin)
        .run();
}

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct AnchorMarker;

pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    let texture = commands.load::<Image>("square.png");
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

    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut last = commands.spawn_bundle((AouiSpriteBundle {
        transform: Transform2D::UNIT.with_anchor(Anchor::CENTER).with_z(0.1),
        sprite: Sprite {
            color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }, Root)).id();
    for _ in 0..120 {
        let curr = commands.spawn_bundle(AouiSpriteBundle {
            transform: Transform2D::UNIT
                .with_anchor(Anchor::CENTER_LEFT)
                .with_center(Anchor::CENTER_LEFT)
                // We use parent anchor for skeletal animation.
                .with_parent_anchor(Anchor::CENTER_RIGHT),
            sprite: Sprite {
                color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }).id();
        let marker = commands.spawn_bundle((AouiSpriteBundle {
            transform: Transform2D::UNIT
                .with_offset(Vec2::new(1.0, 0.0))
                .with_anchor(Anchor::CENTER_RIGHT)
                .with_z(1.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }, AnchorMarker)).id();
        commands.entity(last).push_children(&[curr, marker]);
        last = curr;
    }
}


pub fn egui_window(mut ctx: EguiContexts,
    mut root: Query<&mut Transform2D, With<Root>>,
    mut query: Query<&mut Transform2D, (Without<AnchorMarker>, Without<Root>)>,
) {
    let mut rotation = root.single().rotation;
    let mut root_scaling = root.single().scale.x;
    let mut skewed_scaling = query.iter().next().unwrap().scale.x;
    egui::Window::new("Console").show(ctx.ctx_mut(), |ui| {
        ui.label("Chain");
        ui.add(Slider::new(&mut rotation, -1.0..=1.0).text("segment rotation"));
        ui.add(Slider::new(&mut root_scaling, 0.0..=20.0).text("root scaling"));
        ui.add(Slider::new(&mut skewed_scaling, 0.0..=4.0).text("segment scaling"));
    });
    root.single_mut().rotation = rotation;
    root.single_mut().scale.x = root_scaling;
    for mut sp in query.iter_mut() {
        sp.rotation = rotation;
        sp.scale.x = skewed_scaling;
    }
}
