use bevy::prelude::*;

use crate::components::Billy;
use crate::{GameTextures, WinSize, BILLY_MOVEMENT_SPEED, BILLY_SCALE};

pub struct BillyPlugin;

impl Plugin for BillyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_billy)
            .add_system(billy_movement);
        //.add_system(position_translation);
    }
}

fn spawn_billy(mut commands: Commands, game_textures: Res<GameTextures>, win_size: Res<WinSize>) {
    let bottom = -win_size.height / 2.;
    let start_y_pos = bottom + (win_size.height * 0.7);
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.billy.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, start_y_pos, 10.0), //
                scale: Vec3::new(0.1, 0.1, 0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Billy);
    //.insert(Position { x: 0.0, y: 0.0 });
}

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

fn billy_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut position: Query<&mut Transform, With<Billy>>,
) {
    for mut transform in position.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= BILLY_MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += BILLY_MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= BILLY_MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += BILLY_MOVEMENT_SPEED;
        }
    }
}
