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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerStart {}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Goal {}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GoalBundle {
    player_start: PlayerStart,
}
