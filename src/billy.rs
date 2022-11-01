use crate::components::Billy;
use crate::components::{Obstacle, ObstacleBundle, PlayerStart};
use crate::map::ObstacleCoords;
use crate::{WinSize, BILLY_MOVEMENT_SPEED};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

pub struct BillyPlugin;

impl Plugin for BillyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system(billy_movement);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    win_size: Res<WinSize>,
    player_start_query: Query<&GridCoords, With<PlayerStart>>,
) {
    let texture_handle = asset_server.load("shBase_whiteMale.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let bottom = -win_size.height / 2.;
    let start_y_pos = bottom + (win_size.height * 0.7);

    print!("{:?}", start_y_pos);
    println!("{:?}", start_y_pos);

    for player_start in player_start_query.iter() {
        println!("{:?}", player_start);
    }

    let mut sprite_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0.0, start_y_pos, 1.0), //
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
            print!("{:?}", (x, y));
            print!("{:?}", coords);
            print!("{:?}", obstacles.coords);
            print!("{:?}", obstacles.coords.contains(&coords));
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
