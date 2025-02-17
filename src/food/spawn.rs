use bevy::{
    ecs::{
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec2,
    utils::default,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes,
};

use crate::{
    components::{OnGameScreen, Position},
    diplopod::{DiplopodSegment, DiplopodSegments},
    resources::{FreePositions, TileSize},
};

use super::{Food, SpawnFood, FOOD_COLOR};

fn food_shape(tile_size: &TileSize) -> shapes::Rectangle {
    shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    }
}

pub fn spawn_food(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_food_reader: EventReader<SpawnFood>,
    mut diplopod_positions: Query<&mut Position, With<DiplopodSegment>>,
    mut free_positions: ResMut<FreePositions>,
    tile_size: Res<TileSize>,
) {
    if spawn_food_reader.read().next().is_none() {
        return;
    }

    let shape = food_shape(&tile_size);
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
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(FOOD_COLOR),
                Stroke::color(FOOD_COLOR),
            ))
            .insert(Food)
            .insert(OnGameScreen)
            .insert(pos);

        free_positions.remove(&pos);
    }
}
