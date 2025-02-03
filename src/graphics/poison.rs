use std::cmp;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::EventReader,
        query::With,
        system::{Query, ResMut},
    },
    math::Vec2,
    window::{PrimaryWindow, Window, WindowResized},
};
use bevy_prototype_lyon::{entity::Path, path::ShapePath, shapes};

use crate::{
    components::Poison,
    resources::{TileSize, UpperLeft},
};

pub struct PoisonPlugin;

impl Plugin for PoisonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, on_window_created)
            .add_systems(Update, on_window_resized);
    }
}

fn on_window_created(
    windows: Query<&Window, With<PrimaryWindow>>,
    paths: Query<&mut Path, With<Poison>>,
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
    paths: Query<&mut Path, With<Poison>>,
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
    mut paths: Query<&mut Path, With<Poison>>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    tile_size.0 = cmp::min(width / crate::ARENA_WIDTH, height / crate::ARENA_HEIGHT);
    upper_left.x = (width - (crate::ARENA_WIDTH - 1) * tile_size.0) / 2;
    upper_left.y = (height - (crate::ARENA_HEIGHT - 1) * tile_size.0) / 2;

    let shape = shapes::Circle {
        radius: tile_size.0 as f32 * crate::RADIUS_FACTOR,
        center: Vec2::new(0., 0.),
    };

    for mut path in paths.iter_mut() {
        *path = ShapePath::build_as(&shape);
    }
}
