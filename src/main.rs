// use crate::map::ObstacleCoords;
use bevy::prelude::*;
use bevy::{asset::AssetServerSettings, render::texture::ImageSettings};
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

use crate::components::{
    Billy, GoalBundle, Obstacle, ObstacleBundle, PlayerStart, PlayerStartBundle,
};

mod components;

const WINDOW_HEIGHT: f32 = 150.0;
const WINDOW_WIDTH: f32 = 150.0;

const BILLY_MOVEMENT_SPEED: f32 = 2.0;

pub struct WinSize {
    width: f32,
    height: f32,
}

// pub struct MapPlugin;

// impl Plugin for MapPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(LdtkPlugin)
//             .add_startup_system(setup_map)
//             .insert_resource(LdtkSettings {
//                 int_grid_rendering: IntGridRendering::Invisible,
//                 set_clear_color: SetClearColor::FromLevelBackground,
//                 ..Default::default()
//             })
//             .insert_resource(LevelSelection::Index(0))
//             .register_ldtk_int_cell::<ObstacleBundle>(1)
//             .register_ldtk_int_cell::<PlayerStartBundle>(3)
//             .register_ldtk_int_cell::<GoalBundle>(2);

//         //.add_system(setup_obstacles);
//     }
// }

pub struct ObstacleCoords {
    pub coords: HashSet<(i32, i32)>,
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("billy_map.ldtk"),
        transform: Transform::from_xyz(-100.0, -50.0, 0.0),
        ..Default::default()
    });
}

// fn setup_obstacles(mut commands: Commands, obst_query: Query<&GridCoords, Added<Obstacle>>) {
//     let mut obst = HashSet::new();
//     for grid_coord in obst_query.iter() {
//         obst.insert((grid_coord.x, grid_coord.y));
//     }
//     println!("{:?}", obst);

//     commands.insert_resource(ObstacleCoords { coords: obst })
// }

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_window(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let win_size = WinSize {
        width: window.width(),
        height: window.height(),
    };
    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(4.0);

    commands.insert_resource(win_size)
}

fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(ObstacleCoords {
        coords: HashSet::new(),
    });

    // let texture_handle = asset_server.load("shBase_whiteMale.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 4);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // commands.insert_resource(texture_atlas_handle)
}

// pub struct BillyPlugin;

// impl Plugin for BillyPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_startup_stage_after(StartupStage::PostStartup, setup)
//             .add_system(billy_movement);
//     }
// }

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_billy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_start_query: Query<&GridCoords, With<PlayerStart>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.pressed(KeyCode::P) {
        return;
    }

    print!("{:?}", "SPAWNING BILLY");
    let texture_handle = asset_server.load("shBase_whiteMale.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for player_start in player_start_query.iter() {
        print!("{:?}", "OFKEOFKEKOEFKOEFKOE");
        println!("{:?}", player_start);
        println!("{:?}", (player_start.x, player_start.y));
    }

    let mut sprite_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0.0, -100.0, 1.0), //
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..Default::default()
        },
        ..default()
    };
    sprite_bundle.sprite.index = 1;
    commands
        .spawn_bundle(sprite_bundle)
        .insert(Billy)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

fn get_new_animation_index(
    timer: &Mut<AnimationTimer>,
    animatio_index_1: usize,
    animatio_index_2: usize,
    sprite_index: usize,
) -> usize {
    if !timer.just_finished() {
        return sprite_index;
    }

    if sprite_index == animatio_index_1 {
        return animatio_index_2;
    } else {
        return animatio_index_1;
    }
}

fn billy_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    obst_query: Query<&GridCoords, With<Obstacle>>,
    player_start_query: Query<&GridCoords, With<PlayerStart>>,
    obstacles: Res<ObstacleCoords>,
    mut query: Query<(
        &mut Transform,
        &mut TextureAtlasSprite,
        With<Billy>,
        &mut AnimationTimer,
    )>,
) {
    for (mut transform, mut sprite, _, mut timer) in &mut query {
        timer.tick(time.delta());
        let timer_ref = &timer;

        // for player_start in player_start_query.iter() {
        //     println!("{:?}", (player_start.x, player_start.y));
        //     println!("{:?}", player_start);
        // }

        // let mut obst = HashSet::new();
        // for grid_coord in obst_query.iter() {
        //     obst.insert((grid_coord.x, grid_coord.y));
        // }
        //println!("{:?}", obst);

        if keyboard_input.pressed(KeyCode::Left) {
            let x = transform.translation.x - BILLY_MOVEMENT_SPEED;
            let y = transform.translation.y;

            let coords = (x.floor() as i32, y.ceil() as i32);
            // print!("{:?}", (x, y));
            // print!("{:?}", coords);
            // print!("{:?}", obstacles.coords);
            // print!("{:?}", obstacles.coords.contains(&coords));
            if !obstacles.coords.contains(&coords) {
                transform.translation.x = x;
                sprite.index = get_new_animation_index(timer_ref, 3, 5, sprite.index);
            }

            // transform.translation.x = x;
            // sprite.index = get_new_animation_index(timer_ref, 3, 5, sprite.index);
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += BILLY_MOVEMENT_SPEED;
            sprite.index = get_new_animation_index(timer_ref, 6, 8, sprite.index);
        } else if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= BILLY_MOVEMENT_SPEED;
            sprite.index = get_new_animation_index(timer_ref, 0, 2, sprite.index);
        } else if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += BILLY_MOVEMENT_SPEED;
            sprite.index = get_new_animation_index(timer_ref, 9, 11, sprite.index);
        }
    }
}

// fn billy_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     time: Res<Time>,
//     obstacles: Res<ObstacleCoords>,
//     mut query: Query<(
//         &mut Transform,
//         &mut TextureAtlasSprite,
//         With<Billy>,
//         &mut AnimationTimer,
//     )>,
// ) {
//     for (mut transform, mut sprite, _, mut timer) in &mut query {
//         timer.tick(time.delta());
//         let timer_ref = &timer;

//         let mut x: f32 = transform.translation.x;
//         let mut y: f32 = transform.translation.y;
//         let mut sprite_index: usize = sprite.index;

//         if keyboard_input.pressed(KeyCode::Left) {
//             x = -BILLY_MOVEMENT_SPEED;
//             sprite_index = get_new_animation_index(timer_ref, 3, 5, sprite.index);
//         } else if keyboard_input.pressed(KeyCode::Right) {
//             x = BILLY_MOVEMENT_SPEED;
//             sprite_index = get_new_animation_index(timer_ref, 6, 8, sprite.index);
//         } else if keyboard_input.pressed(KeyCode::Down) {
//             y = -BILLY_MOVEMENT_SPEED;
//             sprite_index = get_new_animation_index(timer_ref, 0, 2, sprite.index);
//         } else if keyboard_input.pressed(KeyCode::Up) {
//             y = BILLY_MOVEMENT_SPEED;
//             sprite_index = get_new_animation_index(timer_ref, 9, 11, sprite.index);
//         }
//         keyboard_input.pressed

//         // println!("{:?}", (x, y, sprite_index));
//         // transform.translation.x += x;
//         // transform.translation.y += y;
//         // sprite.index = sprite_index;
//     }
// }

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            width: 10.0,
            height: 10.0,
            title: String::from("Run Billy Run"),
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_window)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_assets)
        .add_startup_system(setup_map)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<ObstacleBundle>(1)
        .register_ldtk_int_cell::<PlayerStartBundle>(3)
        .register_ldtk_int_cell::<GoalBundle>(2)
        .add_system(spawn_billy)
        .add_system(billy_movement.after(spawn_billy))
        .run();
}
