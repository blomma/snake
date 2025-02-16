use bevy::{
    app::{App, Plugin},
    color::Color,
    ecs::component::Component,
    state::state::OnEnter,
};

use crate::components::GameState;

mod setup;

#[derive(Component)]
pub struct Wall;

pub const WALL_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup::init);
    }
}
