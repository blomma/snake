use std::cmp;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    state::condition::in_state,
    transform::components::Transform,
    window::{PrimaryWindow, Window, WindowResized},
};
use bevy_prototype_lyon::{entity::Path, path::ShapePath, shapes};

use crate::{
    components::{DiplopodPosition, DiplopodSegment},
    resources::{TileSize, UpperLeft},
    GameState, Phase,
};

pub struct DiplopodSegmentsPlugin;

impl Plugin for DiplopodSegmentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, on_window_created)
            .add_systems(Update, on_window_resized)
            .add_systems(
                Update,
                (diplopod_position_translation,)
                    .after(Phase::Movement)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn on_window_created(
    windows: Query<&Window, With<PrimaryWindow>>,
    paths: Query<&mut Path, With<DiplopodSegment>>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if let Ok(window) = windows.get_single() {
        update(
            window.width() as i32,
            window.height() as i32,
            paths,
            tile_size,
            upper_left,
        );
    }
}

fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    paths: Query<&mut Path, With<DiplopodSegment>>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if let Some(resized) = reader.read().next() {
        update(
            resized.width as i32,
            resized.height as i32,
            paths,
            tile_size,
            upper_left,
        );
    }
}

fn update(
    width: i32,
    height: i32,
    mut paths: Query<&mut Path, With<DiplopodSegment>>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    tile_size.0 = cmp::min(width / crate::ARENA_WIDTH, height / crate::ARENA_HEIGHT);
    upper_left.x = (width - (crate::ARENA_WIDTH - 1) * tile_size.0) / 2;
    upper_left.y = (height - (crate::ARENA_HEIGHT - 1) * tile_size.0) / 2;

    let shape = shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    for mut path in paths.iter_mut() {
        *path = ShapePath::build_as(&shape);
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
