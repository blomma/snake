use crate::components::Position;
use bevy::prelude::*;

#[derive(Event)]
pub struct ShowMessage {
    pub text: String,
    pub position: Position,
}
