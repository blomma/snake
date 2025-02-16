mod setup;
pub mod spawn;

use bevy::{
    app::{App, Plugin},
    color::Color,
    ecs::{component::Component, event::Event},
    state::state::OnEnter,
};

use crate::components::GameState;

pub const AMOUNT_OF_FOOD: u32 = 16;
pub const FOOD_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

#[derive(Component)]
pub struct Food;

#[derive(Event)]
pub struct SpawnFood;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup::init);
    }
}
