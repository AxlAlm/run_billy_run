use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

use crate::components::{Obstacle, ObstacleBundle};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_map)
            // .add_startup_system(setup_map)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<ObstacleBundle>(1)
            //.add_startup_system_to_stage(StartupStage::PostStartup, setup_obstacles);
            .add_system(setup_obstacles);
    }
}

pub struct ObstacleCoords {
    pub coords: HashSet<(i32, i32)>,
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("billy_map.ldtk"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn setup_obstacles(mut commands: Commands, obst_query: Query<&GridCoords, Added<Obstacle>>) {
    let mut obst = HashSet::new();
    for grid_coord in obst_query.iter() {
        obst.insert((grid_coord.x, grid_coord.y));
    }
    println!("{:?}", obst);

    commands.insert_resource(ObstacleCoords { coords: obst })
}
