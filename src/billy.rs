use bevy::prelude::*;

use crate::components::Billy;
use crate::{GameTextures, WinSize, BILLY_MOVEMENT_SPEED, BILLY_SCALE};

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
) {
    let texture_handle = asset_server.load("shBase_whiteMale.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let bottom = -win_size.height / 2.;
    let start_y_pos = bottom + (win_size.height * 0.7);

    let mut sprite_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0.0, start_y_pos, 1.0), //
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..Default::default()
        },
        //transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    };
    sprite_bundle.sprite.index = 1;
    commands
        .spawn_bundle(sprite_bundle)
        .insert(Billy)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
    //.insert(AnimationTimer(Timer::from_seconds(0.1, true))
    //);
}

// fn spawn_billy(mut commands: Commands, game_textures: Res<GameTextures>, win_size: Res<WinSize>) {
//     let bottom = -win_size.height / 2.;
//     let start_y_pos = bottom + (win_size.height * 0.7);
//     commands
//         .spawn_bundle(SpriteBundle {
//             texture: game_textures.billy.clone(),
//             transform: Transform {
//                 translation: Vec3::new(0.0, start_y_pos, 0.0), //
//                 scale: Vec3::new(BILLY_SCALE, BILLY_SCALE, 0.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(Billy);
//     //.insert(Position { x: 0.0, y: 0.0 });
// }

// fn position_translation(win_size: Res<WinSize>, mut q: Query<(&Position, &mut Transform)>) {
//     fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
//         let tile_size = bound_window / bound_game;
//         pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
//     }

//     for (pos, mut transform) in q.iter_mut() {
//         transform.translation = Vec3::new(
//             convert(pos.x, win_size.width, MAP_WIDTH),
//             convert(pos.y, win_size.height, MAP_HEIGHT),
//             0.0,
//         );
//     }
// }

fn update_animation(
    animatio_index_1: usize,
    animatio_index_2: usize,
    sprite_index: usize,
) -> usize {
    // timer.tick(time.delta());

    // if !timer.just_finished() {
    //     return sprite_index;
    // }

    if sprite_index == animatio_index_1 {
        return animatio_index_2;
    } else {
        return animatio_index_1;
    }
}

fn billy_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    // mut position: Query<&mut Transform, With<Billy>>,
    mut query: Query<(
        &mut Transform,
        &mut TextureAtlasSprite,
        With<Billy>,
        &mut AnimationTimer,
    )>,
) {
    for (mut transform, mut sprite, _, mut timer) in &mut query {
        timer.tick(time.delta());

        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= BILLY_MOVEMENT_SPEED;

            if timer.just_finished() {
                sprite.index = update_animation(3, 5, sprite.index);
            }
        }

        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += BILLY_MOVEMENT_SPEED;

            if timer.just_finished() {
                sprite.index = update_animation(6, 8, sprite.index);
            }
        }

        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= BILLY_MOVEMENT_SPEED;

            if timer.just_finished() {
                sprite.index = update_animation(0, 2, sprite.index);
            }
        }

        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += BILLY_MOVEMENT_SPEED;

            if timer.just_finished() {
                sprite.index = update_animation(9, 11, sprite.index);
            }
        }
    }
}
