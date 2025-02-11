use std::cmp;

use bevy::{
    app::{App, Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, ResMut},
        world::World,
    },
    math::Vec2,
    state::state::OnEnter,
    utils::default,
    window::WindowResized,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    path::ShapePath,
    prelude::GeometryBuilder,
    shapes::{self},
};

use crate::{
    components::{GameState, OnGameScreen, Position},
    resources::{FreePositions, TileSize, UpperLeft},
};

#[derive(Component)]
pub struct Wall;

pub const WALL_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), init_wall)
            .add_systems(Update, on_window_resized);
    }
}

fn wall_shape(tile_size: &TileSize) -> shapes::Rectangle {
    return shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32 * 2.0),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };
}

fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    mut paths: Query<&mut Path, With<Wall>>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    if let Some(resized) = reader.read().next() {
        tile_size.0 = cmp::min(
            resized.width as i32 / crate::ARENA_WIDTH,
            resized.height as i32 / crate::ARENA_HEIGHT,
        );
        upper_left.x = (resized.width as i32 - (crate::ARENA_WIDTH - 1) * tile_size.0) / 2;
        upper_left.y = (resized.height as i32 - (crate::ARENA_HEIGHT - 1) * tile_size.0) / 2;

        let shape = wall_shape(&tile_size);
        for mut path in paths.iter_mut() {
            *path = ShapePath::build_as(&shape);
        }
    }
}

fn init_wall(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let Some(tile_size) = world.get_resource::<TileSize>() else {
            return;
        };

        let shape = wall_shape(tile_size);
        let mut positions: Vec<Position> = Vec::new();

        for x in 0..crate::CONSUMABLE_WIDTH + 1 {
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

        for y in 1..crate::CONSUMABLE_HEIGHT {
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
