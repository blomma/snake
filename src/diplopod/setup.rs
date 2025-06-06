use super::{DIPLOPOD_COLOR, DiplopodHead, DiplopodSegment, DiplopodSegments};
use crate::{
    components::{OnGameScreen, Position},
    resources::TileSize,
};
use bevy::prelude::*;

pub fn init(
    mut commands: Commands,
    mut segments: ResMut<DiplopodSegments>,
    tile_size: Res<TileSize>,
) {
    segments.0 = vec![
        commands
            .spawn((
                Sprite::from_color(DIPLOPOD_COLOR, Vec2::splat(tile_size.0 as f32)),
                Transform::default(),
            ))
            .insert(DiplopodHead {
                direction: Vec2::ZERO,
            })
            .insert(DiplopodSegment)
            .insert(Position {
                x: crate::ARENA_WIDTH / 2,
                y: crate::ARENA_HEIGHT / 2,
            })
            .insert(OnGameScreen)
            .id(),
    ];
}
