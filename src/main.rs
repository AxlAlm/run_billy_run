use bevy::prelude::*;

const BILLY_MOVEMENT_SPEED: f32 = 4.0;
const BILLY_SPRITE: &str = "Basic_green_dot.png";
const BILLY_SIZE: (f32, f32) = (100.0, 100.0);
const BILLY_SCALE: f32 = 0.1;

#[derive(Component)]
struct Billy;

pub struct WinSize {
    width: f32,
    height: f32,
}

pub struct GameTexture {
    billy: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Run Billy Run!".to_string(),
            width: 598.0,
            height: 600.0,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(setup_window)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_billy)
        .add_system(billy_movement)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_window(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    let window = windows.get_primary_mut().unwrap();
    let win_size = WinSize {
        width: window.width(),
        height: window.height(),
    };

    commands.insert_resource(win_size)
}

fn spawn_billy(mut commands: Commands, assert_server: Res<AssetServer>, win_size: Res<WinSize>) {
    let bottom = -win_size.height / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: assert_server.load(BILLY_SPRITE),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + (win_size.height * 0.7), 10.0), //
                scale: Vec3::new(BILLY_SCALE, BILLY_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Billy);
}

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
