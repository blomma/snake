use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: snake::TITLE.into(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            snake::GamePlugin,
        ))
        .run();
}
