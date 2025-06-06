use bevy::app::{App, Plugin, Startup};

mod setup;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::init);
    }
}
