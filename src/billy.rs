use crate::components::Billy;
use crate::{WinSize, BILLY_MOVEMENT_SPEED, BILLY_SCALE};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

//    mut wall_query: Query<(&mut Transform, With<Wall>)>,

fn collision_systemn() {}

fn billy_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
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
        // if else for non-diagonal movements
        // only if blocks if we want to allow diagonal movements
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= BILLY_MOVEMENT_SPEED;
            sprite.index = get_new_animation_index(timer_ref, 3, 5, sprite.index);
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
