use bevy::{
    ecs::{system::Commands, world::World},
    math::Vec2,
    utils::default,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self},
};

use crate::{
    components::{OnGameScreen, Position},
    resources::{FreePositions, TileSize},
};

use super::{Wall, WALL_COLOR};

fn wall_shape(tile_size: &TileSize) -> shapes::Rectangle {
    shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    }
}

pub fn init(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let Some(tile_size) = world.get_resource::<TileSize>() else {
            return;
        };

        let shape = wall_shape(tile_size);
        let mut positions: Vec<Position> = Vec::new();

        for x in 0..crate::ARENA_WIDTH {
            let pos = Position { x, y: 0 };
            world.spawn((
                Wall,
                pos,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(WALL_COLOR),
                Stroke::color(WALL_COLOR),
                OnGameScreen,
            ));
            positions.push(pos);

            let pos = Position {
                x,
                y: crate::CONSUMABLE_HEIGHT,
            };
            world.spawn((
                Wall,
                pos,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(WALL_COLOR),
                Stroke::color(WALL_COLOR),
                OnGameScreen,
            ));
            positions.push(pos);
        }

        for y in 1..crate::ARENA_HEIGHT - 1 {
            let pos = Position { x: 0, y };
            world.spawn((
                Wall,
                pos,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(WALL_COLOR),
                Stroke::color(WALL_COLOR),
                OnGameScreen,
            ));
            positions.push(pos);

            let pos = Position {
                x: crate::CONSUMABLE_WIDTH,
                y,
            };
            world.spawn((
                Wall,
                pos,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(WALL_COLOR),
                Stroke::color(WALL_COLOR),
                OnGameScreen,
            ));
            positions.push(pos);
        }

        if let Some(mut free_positions) = world.get_resource_mut::<FreePositions>() {
            while let Some(position) = positions.pop() {
                free_positions.remove(&position);
            }
        };
    });
}
