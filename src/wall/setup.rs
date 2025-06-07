use super::{WALL_COLOR, Wall};
use crate::{
    components::{OnGameScreen, Position},
    resources::{FreePositions, TileSize},
};
use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let Some(tile_size) = world.get_resource::<TileSize>() else {
            panic!("Tilesize not available");
        };

        let rectangle = Rectangle::new(tile_size.0 as f32, tile_size.0 as f32);
        let mesh =
            world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| meshes.add(rectangle));

        let color = world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
            materials.add(WALL_COLOR)
        });

        let mut positions: Vec<Position> = Vec::new();

        for x in 0..crate::ARENA_WIDTH {
            let pos = Position { x, y: 0 };
            world.spawn((
                Wall,
                pos,
                Mesh2d(mesh.clone()),
                MeshMaterial2d(color.clone()),
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
                Mesh2d(mesh.clone()),
                MeshMaterial2d(color.clone()),
                OnGameScreen,
            ));
            positions.push(pos);
        }

        for y in 1..crate::ARENA_HEIGHT - 1 {
            let pos = Position { x: 0, y };
            world.spawn((
                Wall,
                pos,
                Mesh2d(mesh.clone()),
                MeshMaterial2d(color.clone()),
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
                Mesh2d(mesh.clone()),
                MeshMaterial2d(color.clone()),
                OnGameScreen,
            ));
            positions.push(pos);
        }

        let Some(mut free_positions) = world.get_resource_mut::<FreePositions>() else {
            panic!("FreePositions not available");
        };
        free_positions.remove_all(&positions);
    });
}
