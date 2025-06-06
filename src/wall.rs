use crate::{
    ARENA_HEIGHT, ARENA_WIDTH,
    components::{GameState, Phase, Position},
    resources::TileSize,
};
use bevy::{prelude::*, window::PrimaryWindow};

mod setup;

#[derive(Component)]
pub struct Wall;

pub const WALL_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup::init)
            .add_systems(
                Update,
                (position_translation,)
                    .after(Phase::Movement)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform), With<Wall>>,
    tile_size: Res<TileSize>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32, tile_size: f32) -> f32 {
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let Ok(window) = windows.single() else {
        return;
    };

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width(),
                ARENA_WIDTH as f32,
                tile_size.0 as f32,
            ),
            convert(
                pos.y as f32,
                window.height(),
                ARENA_HEIGHT as f32,
                tile_size.0 as f32,
            ),
            2.0,
        );
    }
}
