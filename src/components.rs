use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Billy;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Obstacle;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ObstacleBundle {
    obstacle: Obstacle,
}
