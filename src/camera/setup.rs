use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::Commands};

pub fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}
