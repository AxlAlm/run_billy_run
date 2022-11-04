use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.update_scale_factor_from_backend(3.0);
    //window.set_resolution(180.0, 180.0);
    window.set_maximized(true);
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("billy_map.ldtk"),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_billy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    players: Query<(Entity, &Transform), Added<components::Player>>,
) {
    for (player, transform) in players.iter() {
        println!("PLAYER: {:?}", player);
        println!("TRANSFORM: {:?}", transform);

        let texture_handle = asset_server.load("shBase_whiteMale.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: transform.translation.clone(), //
                scale: transform.scale.clone(),
                ..Default::default()
            },
            ..default()
        };
        sprite_bundle.sprite.index = 1;
        commands
            .entity(player)
            .insert_bundle(sprite_bundle)
            .insert(Velocity::zero())
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(1.0, 1.0))
            .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
        println!("SPAWNING")
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    obstacles: Query<(&GridCoords, &Transform), Added<components::Obstacle>>,
) {
    for (grid_coors, transform) in obstacles.iter() {
        println!("OBSTACLES: {:?}", grid_coors);
        println!("TRANSFORM: {:?}", transform);

        commands
            .spawn()
            // .insert(components::Obstacle)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(1.0, 1.0));
    }
}

// pub fn movement(
//     input: Res<Input<KeyCode>>,
//     mut query: Query<&mut Velocity, With<components::Player>>,
// ) {
//     for mut velocity in query.iter_mut() {
//         println!("VELOCITY: {:?}", velocity);

//         velocity.linvel.x = 1.0;
//         velocity.angvel = 1.0;

//         // let right = if input.pressed(KeyCode::D) { 1. } else { 1. };
//         // let left = if input.pressed(KeyCode::A) { 1. } else { 1. };

//         // velocity.linvel.x = (right - left) * 200.;
//     }
// }

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

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_info: Query<
        (&mut Velocity, &mut AnimationTimer, &mut TextureAtlasSprite),
        With<components::Player>,
    >,
) {
    for (mut rb_vels, mut timer, mut sprite) in player_info.iter_mut() {
        timer.tick(time.delta());
        let timer_ref = &timer;

        println!("VELOCITY: {:?}", rb_vels);
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }
        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * 100.0;

        if rb_vels.linvel.x > 0.0 {
            sprite.index = get_new_animation_index(timer_ref, 6, 8, sprite.index);
        } else if rb_vels.linvel.x < 0.0 {
            sprite.index = get_new_animation_index(timer_ref, 3, 5, sprite.index);
        } else if rb_vels.linvel.y > 0.0 {
            sprite.index = get_new_animation_index(timer_ref, 9, 11, sprite.index);
        } else if rb_vels.linvel.y < 0.0 {
            sprite.index = get_new_animation_index(timer_ref, 0, 2, sprite.index);
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            set_clear_color: SetClearColor::FromLevelBackground,
            // level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            //     load_level_neighbors: true,
            // },
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_window)
        .add_startup_system(setup_map)
        .add_system(spawn_billy)
        .add_system(spawn_obstacles)
        .add_system(player_movement)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<components::PlayerBundle>("BillyStart")
        .register_ldtk_int_cell::<components::ObstacleBundle>(1)
        .run();
}
