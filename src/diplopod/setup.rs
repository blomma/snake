use super::{DIPLOPOD_COLOR, DiplopodHead, DiplopodSegment, DiplopodSegments};
use crate::{
    components::{OnGameScreen, Position},
    resources::TileSize,
};
use bevy::prelude::*;

pub fn init(
    mut commands: Commands,
    mut segments: ResMut<DiplopodSegments>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tile_size: Res<TileSize>,
) {
    segments.0 = vec![
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(tile_size.0 as f32, tile_size.0 as f32))),
                MeshMaterial2d(materials.add(DIPLOPOD_COLOR)),
                Transform::from_translation(Vec3::ZERO),
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
