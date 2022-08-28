use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
