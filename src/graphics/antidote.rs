use std::cmp;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::EventReader,
        query::With,
        system::{Query, ResMut},
    },
    math::Vec2,
    window::WindowResized,
};
use bevy_prototype_lyon::{
    draw::Stroke,
    entity::Path,
    path::{PathBuilder, ShapePath},
};

use crate::{
    components::AntiDote,
    resources::{TileSize, UpperLeft},
};

pub struct AntiDotePlugin;

impl Plugin for AntiDotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_window_resized);
    }
}

fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    mut paths: Query<(&mut Path, &mut Stroke), With<AntiDote>>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    let Some(resized) = reader.read().next() else {
        return;
    };

    tile_size.0 = cmp::min(
        resized.width as i32 / crate::ARENA_WIDTH,
        resized.height as i32 / crate::ARENA_HEIGHT,
    );
    upper_left.x = (resized.width as i32 - (crate::ARENA_WIDTH - 1) * tile_size.0) / 2;
    upper_left.y = (resized.height as i32 - (crate::ARENA_HEIGHT - 1) * tile_size.0) / 2;

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
    path_builder.line_to(tile_size.0 as f32 * Vec2::X);
    path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
    path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
    let cross = path_builder.build();

    for (mut path, mut stroke) in paths.iter_mut() {
        *path = ShapePath::build_as(&cross);
        *stroke = Stroke::new(crate::ANTIDOTE_COLOR, tile_size.0 as f32 * 0.9);
    }
}
