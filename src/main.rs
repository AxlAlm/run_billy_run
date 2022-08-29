use bevy::prelude::*;

use billy::BillyPlugin;

mod billy;
mod components;

const BILLY_MOVEMENT_SPEED: f32 = 4.0;
const BILLY_SPRITE: &str = "Basic_green_dot.png";
const BILLY_SIZE: (f32, f32) = (100.0, 100.0);
const BILLY_SCALE: f32 = 0.1;

const TILE_SIZE: f32 = 0.1;

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 600.0;

// const MAP_HEIGHT: f32 = 1000.0;
// const MAP_WIDTH: f32 = 600.0;

pub struct WinSize {
    width: f32,
    height: f32,
}

pub struct GameTextures {
    billy: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Run Billy Run!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BillyPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_window)
        .add_startup_system(setup_game_textures)
        .run();
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

    commands.insert_resource(win_size)
}

fn setup_game_textures(mut commands: Commands, assert_server: Res<AssetServer>) {
    // camera
    let game_textures = GameTextures {
        billy: assert_server.load(BILLY_SPRITE),
    };
    commands.insert_resource(game_textures)
}
