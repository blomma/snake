use std::cmp;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::{common_conditions::resource_exists, IntoSystemConfigs},
        system::{Query, ResMut},
    },
    math::Vec2,
    window::{WindowCreated, WindowResized},
};
use bevy_prototype_lyon::{entity::Path, path::ShapePath, shapes};

use crate::{
    components::Poison,
    resources::{self, TileSize, UpperLeft},
};

pub struct PoisonPlugin;

impl Plugin for PoisonPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(
            //     Update,
            //     on_window_created
            //         .run_if(resource_exists::<resources::TileSize>)
            //         .run_if(resource_exists::<resources::UpperLeft>),
            // )
            .add_systems(Update, on_window_resized);
    }
}

fn on_window_created(
    mut reader: EventReader<WindowCreated>,
    paths: Query<&mut Path, With<Poison>>,
    tile_size: ResMut<TileSize>,
) {
    if reader.read().next().is_some() {
        println!("PoisonPlugin::on_window_created");
        update(paths, tile_size);
    }
}

fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    paths: Query<&mut Path, With<Poison>>,
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

        update(paths, tile_size);
    }
}

fn update(mut paths: Query<&mut Path, With<Poison>>, tile_size: ResMut<TileSize>) {
    let shape = shapes::Circle {
        radius: tile_size.0 as f32 * crate::RADIUS_FACTOR,
        center: Vec2::new(0., 0.),
    };

    for mut path in paths.iter_mut() {
        *path = ShapePath::build_as(&shape);
    }
}
