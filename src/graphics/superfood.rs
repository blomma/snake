use std::cmp;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::{
            common_conditions::{not, resource_exists},
            IntoSystemConfigs,
        },
        system::{Query, Res, ResMut},
    },
    math::{Quat, Vec2},
    state::condition::in_state,
    time::Time,
    transform::components::Transform,
    window::{PrimaryWindow, Window, WindowResized},
};
use bevy_prototype_lyon::{
    entity::Path,
    path::{PathBuilder, ShapePath},
};

use crate::{
    components::SuperFood,
    resources::{Paused, TileSize, UpperLeft},
    GameState, Phase,
};

pub struct SuperFoodPlugin;

impl Plugin for SuperFoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, on_window_created).add_systems(
            Update,
            (
                on_window_resized,
                (rotate_superfood,)
                    .after(Phase::Movement)
                    .run_if(in_state(GameState::Game))
                    .run_if(not(resource_exists::<Paused>)),
            ),
        );
    }
}

fn on_window_created(
    windows: Query<&Window, With<PrimaryWindow>>,
    paths: Query<&mut Path, With<SuperFood>>,
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
    paths: Query<&mut Path, With<SuperFood>>,
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
    mut paths: Query<&mut Path, With<SuperFood>>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    tile_size.0 = cmp::min(width / crate::ARENA_WIDTH, height / crate::ARENA_HEIGHT);
    upper_left.x = (width - (crate::ARENA_WIDTH - 1) * tile_size.0) / 2;
    upper_left.y = (height - (crate::ARENA_HEIGHT - 1) * tile_size.0) / 2;

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
    path_builder.line_to(tile_size.0 as f32 * Vec2::X);
    path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
    path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
    let cross = path_builder.build();

    for mut path in paths.iter_mut() {
        *path = ShapePath::build_as(&cross);
    }
}

fn rotate_superfood(mut query: Query<&mut Transform, With<SuperFood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_secs();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}
