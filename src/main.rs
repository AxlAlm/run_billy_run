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
    window.update_scale_factor_from_backend(4.0);
    // window.set_resolution(180.0, 180.0);
    window.set_maximized(true);
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("billy_map.ldtk"),
        transform: Transform::from_xyz(-100.0, -50.0, 0.0),
        ..Default::default()
    });
}

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
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_window)
        .add_startup_system(setup_map)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<components::PlayerBundle>("entity_identifier")
        .run();
}
