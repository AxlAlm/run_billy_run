use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
// use bevy_rapier2d::prelude::*;

// #[derive(Clone, Default, Bundle, LdtkIntCell)]
// pub struct ColliderBundle {
//     pub collider: Collider,
//     pub rigid_body: RigidBody,
//     pub velocity: Velocity,
//     pub rotation_constraints: LockedAxes,
//     pub friction: Friction,
//     pub restitution: Restitution,
//     pub mass_properties: ColliderMassProperties,
//     pub force: ExternalForce,
// }

// impl From<EntityInstance> for ColliderBundle {
//     fn from(entity_instance: EntityInstance) -> ColliderBundle {
//         match entity_instance.identifier.as_ref() {
//             "BillyStart" => ColliderBundle::default(),
//             _ => ColliderBundle::default(),
//         }
//     }
// }

// impl From<IntGridCell> for ColliderBundle {
//     fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
//         if int_grid_cell.value == 2 {
//             ColliderBundle {
//                 collider: Collider::cuboid(8., 8.),
//                 rotation_constraints: LockedAxes::ROTATION_LOCKED,
//                 ..Default::default()
//             }
//         } else {
//             ColliderBundle::default()
//         }
//     }
// }

// #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
// pub struct Player;

// #[derive(Default, Bundle, LdtkEntity)]
// pub struct PlayerBundle {
//     #[from_entity_instance]
//     #[bundle]
//     pub collider_bundle: ColliderBundle,
//     pub player: Player,

//     #[from_entity_instance]
//     entity_instance: EntityInstance,
// }

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Obstacle;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct ObstacleBundle {
    obstacle: Obstacle,
}
