use bevy::{
    ecs::system::{Commands, Res, ResMut},
    math::Vec2,
    utils::default,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self},
};

use crate::{components::OnGameScreen, resources::TileSize};

use super::{DiplopodHead, DiplopodPosition, DiplopodSegment, DiplopodSegments};

fn diplopod_shape(tile_size: &Res<TileSize>) -> shapes::Rectangle {
    return shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };
}

pub fn init(
    mut commands: Commands,
    mut segments: ResMut<DiplopodSegments>,
    tile_size: Res<TileSize>,
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
