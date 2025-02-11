use std::cmp;

use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::{
            common_conditions::{not, on_event, resource_exists},
            IntoSystemConfigs,
        },
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    math::Vec2,
    state::{condition::in_state, state::OnEnter},
    utils::default,
    window::WindowResized,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    path::ShapePath,
    prelude::GeometryBuilder,
    shapes,
};

use crate::{
    components::{DiplopodPosition, Food, GameState, OnGameScreen, Position},
    events::SpawnConsumables,
    resources::{DiplopodSegments, FreePositions, Paused, TileSize, UpperLeft},
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            ((spawn_food.run_if(on_event::<SpawnConsumables>),))
                .run_if(in_state(GameState::Game))
                .run_if(not(resource_exists::<Paused>)),
        )
        .add_systems(OnEnter(GameState::Game), init_food)
        .add_systems(Update, on_window_resized);
    }
}

// fn food_shape(tile_size: &TileSize) -> shapes::Circle {
//     return shapes::Circle {
//         radius: tile_size.0 as f32 * crate::RADIUS_FACTOR,
//         center: Vec2::new(0., 0.),
//     };
// }

fn food_square_shape(tile_size: &TileSize) -> shapes::Rectangle {
    return shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };
}

fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    mut paths: Query<&mut Path, With<Food>>,
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

        let shape = food_square_shape(&tile_size);
        for mut path in paths.iter_mut() {
            *path = ShapePath::build_as(&shape);
        }
    }
}

fn init_food(mut commands: Commands) {
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

        let shape = food_square_shape(tile_size);
        for _ in 0..crate::AMOUNT_OF_FOOD {
            match position_candidates.positions.pop() {
                None => break,
                Some(pos) => {
                    world
                        .spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                ..default()
                            },
                            Fill::color(crate::FOOD_COLOR),
                            Stroke::color(crate::FOOD_COLOR),
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

fn spawn_food(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_consumables_reader: EventReader<SpawnConsumables>,
    mut diplopod_positions: Query<&mut DiplopodPosition>,
    mut free_positions: ResMut<FreePositions>,
    tile_size: Res<TileSize>,
) {
    if let Some(spawn_event) = spawn_consumables_reader.read().next() {
        if !spawn_event.regular {
            return;
        }

        let shape = food_square_shape(&tile_size);
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *diplopod_positions.get_mut(*e).unwrap())
            .map(|p| p.to_position())
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
                    Fill::color(crate::FOOD_COLOR),
                    Stroke::color(crate::FOOD_COLOR),
                ))
                .insert(Food)
                .insert(OnGameScreen)
                .insert(pos);

            free_positions.remove(&pos);
        }
    }
}
