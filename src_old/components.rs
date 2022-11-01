use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

use heron::prelude::*;

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

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        match entity_instance.identifier.as_ref() {
            "BillyStart" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(6., 14., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
            },
            "BillyEnd" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(5., 5., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                ..Default::default()
            },
        }
    }
}
