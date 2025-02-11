use std::cmp;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    state::{condition::in_state, state::OnEnter},
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window, WindowResized},
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    path::ShapePath,
    prelude::GeometryBuilder,
    shapes,
};

use crate::{
    components::{DiplopodHead, DiplopodPosition, DiplopodSegment, OnGameScreen},
    resources::{DiplopodSegments, TileSize, UpperLeft},
    GameState, Phase,
};

pub struct DiplopodSegmentsPlugin;

impl Plugin for DiplopodSegmentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), init_diplopod)
            .add_systems(Update, on_window_resized)
            .add_systems(
                Update,
                (diplopod_position_translation,)
                    .after(Phase::Movement)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn diplopod_shape(tile_size: &Res<TileSize>) -> shapes::Rectangle {
    return shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };
}

fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    mut paths: Query<&mut Path, With<DiplopodSegment>>,
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

        let shape = diplopod_shape(&tile_size.into());
        for mut path in paths.iter_mut() {
            *path = ShapePath::build_as(&shape);
        }
    }
}

fn diplopod_position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&DiplopodPosition, &mut Transform)>,
    tile_size: Res<TileSize>,
    upper_left: Res<UpperLeft>,
) {
    if let Ok(window) = windows.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                (pos.x * tile_size.0 + upper_left.x - window.width() as i32 / 2) as f32,
                (pos.y * tile_size.0 + upper_left.y - window.height() as i32 / 2) as f32,
                0.0,
            )
        }
    }
}

fn init_diplopod(
    mut commands: Commands,
    mut segments: ResMut<DiplopodSegments>,
    tile_size: Res<TileSize>,
) {
    spawn_diplopod(&mut commands, &mut segments, &tile_size);
}

fn spawn_diplopod(
    commands: &mut Commands,
    segments: &mut ResMut<DiplopodSegments>,
    tile_size: &Res<TileSize>,
) {
    let shape = diplopod_shape(&tile_size);
    segments.0 = vec![commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(crate::DIPLOPOD_COLOR),
            Stroke::color(crate::DIPLOPOD_COLOR),
        ))
        .insert(DiplopodHead {
            direction: Vec2::ZERO,
        })
        .insert(DiplopodSegment)
        .insert(DiplopodPosition {
            x: crate::ARENA_WIDTH / 2,
            y: crate::ARENA_HEIGHT / 2,
        })
        .insert(OnGameScreen)
        .id()];
}
