use bevy::ecs::event::Event;

use crate::diplopod::DiplopodPosition;

#[derive(Event)]
pub struct ShowMessage {
    pub text: String,
    pub position: DiplopodPosition,
}
