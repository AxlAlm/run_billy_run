use bevy::prelude::*;
use bevy::{asset::AssetServerSettings, render::texture::ImageSettings};

use billy::BillyPlugin;
use map::MapPlugin;

mod billy;
mod components;
mod map;

const TILE_SIZE: f32 = 0.05;
// const WINDOW_HEIGHT: f32 = 600.0;
// const WINDOW_WIDTH: f32 = 600.0;

const BILLY_MOVEMENT_SPEED: f32 = 2.0;
// const BILLY_SPRITE: &str = "Basic_green_dot.png";
// // const BILLY_SIZE: (f32, f32) = (100.0, 100.0);
const BILLY_SCALE: f32 = TILE_SIZE;

// const MAP_HEIGHT: f32 = 1000.0;
// const MAP_WIDTH: f32 = 600.0;

pub struct WinSize {
    width: f32,
    height: f32,
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_window(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    let window = windows.get_primary_mut().unwrap();
    let win_size = WinSize {
        width: window.width(),
        height: window.height(),
    };

    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(3.0);

    commands.insert_resource(win_size)
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(WindowDescriptor {
            width: 512.0,
            height: 512.0,
            title: String::from("Run Billy Run"),
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_window)
        .add_plugin(MapPlugin)
        .add_plugin(BillyPlugin)
        .run();
}
