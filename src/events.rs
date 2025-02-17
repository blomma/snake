use bevy::ecs::event::Event;

use crate::components::Position;

#[derive(Event)]
pub struct ShowMessage {
    pub text: String,
    pub position: Position,
}
