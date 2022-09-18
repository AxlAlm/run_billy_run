use bevy::prelude::*;
use bevy_ecs_tilemap::*;

use crate::ldtk_map::*;

use bevy::{asset::AssetServerSettings, render::texture::ImageSettings};

use billy::BillyPlugin;
//use lava::LavaPlugin;

mod billy;
mod components;
mod ldtk_map;

const TILE_SIZE: f32 = 0.05;
// const WINDOW_HEIGHT: f32 = 600.0;
// const WINDOW_WIDTH: f32 = 600.0;

const BILLY_MOVEMENT_SPEED: f32 = 2.0;
// const BILLY_SPRITE: &str = "Basic_green_dot.png";
// // const BILLY_SIZE: (f32, f32) = (100.0, 100.0);
const BILLY_SCALE: f32 = TILE_SIZE;

// println!(TILE_SIZE);
// const LAVA_SPRITE: &str = "lava2.png";

// const MAP_HEIGHT: f32 = 1000.0;
// const MAP_WIDTH: f32 = 600.0;

pub struct WinSize {
    width: f32,
    height: f32,
}

// pub struct GameTextures {
//     billy: Handle<Image>,
//     lava: Handle<Image>,
// }

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

// fn setup_game_textures(mut commands: Commands, assert_server: Res<AssetServer>) {
//     // camera
//     let game_textures = GameTextures {
//         billy: assert_server.load(BILLY_SPRITE),
//         lava: assert_server.load(LAVA_SPRITE),
//     };
//     commands.insert_resource(game_textures)
// }

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<LdtkMap> = asset_server.load("billy_map.ldtk");
    commands.spawn().insert_bundle(LdtkMapBundle {
        ldtk_map: handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(WindowDescriptor {
            width: 512.0,
            height: 512.0,
            title: String::from("LDTK Example"),
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(BillyPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_window)
        .add_startup_system(setup_map)
        .run();
}
