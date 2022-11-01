use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_ldtk::prelude::*;
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

fn spawn_billy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    players: Query<(Entity, &Transform), (Added<EntityInstance>, With<components::Player>)>,
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
        commands.spawn_bundle(sprite_bundle);
        println!("SPAWNING")
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
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<components::PlayerBundle>("BillyStart")
        .run();
}
