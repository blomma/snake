use super::{FOOD_COLOR, Food, SpawnFood};
use crate::{
    components::{OnGameScreen, Position},
    diplopod::{DiplopodSegment, DiplopodSegments},
    resources::{FreePositions, TileSize},
};
use bevy::prelude::*;

pub fn spawn_food(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_food_reader: EventReader<SpawnFood>,
    mut diplopod_positions: Query<&mut Position, With<DiplopodSegment>>,
    mut free_positions: ResMut<FreePositions>,
    tile_size: Res<TileSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if spawn_food_reader.read().next().is_none() {
        return;
    }

    let segment_positions = segments
        .0
        .iter()
        .map(|e| *diplopod_positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    let mut position_candidates = free_positions.clone();
    position_candidates.remove_all(&segment_positions);

    if let Some(pos) = position_candidates.positions.pop() {
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(tile_size.0 as f32, tile_size.0 as f32))),
                MeshMaterial2d(materials.add(FOOD_COLOR)),
            ))
            .insert(Food)
            .insert(OnGameScreen)
            .insert(pos);

        free_positions.remove(&pos);
    }
}
