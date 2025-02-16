use bevy::{
    ecs::{system::Commands, world::World},
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
    diplopod::DiplopodPosition,
    resources::{FreePositions, TileSize},
};

use super::{Food, AMOUNT_OF_FOOD, FOOD_COLOR};

fn food_shape(tile_size: &TileSize) -> shapes::Circle {
    return shapes::Circle {
        radius: tile_size.0 as f32 * crate::RADIUS_FACTOR,
        center: Vec2::new(0., 0.),
    };
}

pub fn init(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let Some(free_positions) = world.get_resource::<FreePositions>() else {
            return;
        };

        let segment_positions = vec![DiplopodPosition {
            x: crate::ARENA_WIDTH / 2,
            y: crate::ARENA_HEIGHT / 2,
        }
        .to_position()];

        let mut position_candidates = free_positions.clone();
        position_candidates.remove_all(&segment_positions);

        let Some(tile_size) = world.get_resource::<TileSize>() else {
            return;
        };

        let mut positions: Vec<Position> = Vec::new();

        let shape = food_shape(tile_size);
        for _ in 0..AMOUNT_OF_FOOD {
            match position_candidates.positions.pop() {
                None => break,
                Some(pos) => {
                    world
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
                    positions.push(pos);
                }
            }
        }

        if let Some(mut free_positions) = world.get_resource_mut::<FreePositions>() {
            while let Some(position) = positions.pop() {
                free_positions.remove(&position);
            }
        };
    });
}
