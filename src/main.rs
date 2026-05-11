use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.9)))
        .add_plugins(DefaultPlugins)
        .run();
}
