use super::{AMOUNT_OF_FOOD, FOOD_COLOR, Food};
use crate::{
    components::{OnGameScreen, Position},
    resources::{FreePositions, TileSize},
};
use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let Some(free_positions) = world.get_resource::<FreePositions>() else {
            panic!("FreePositions not available");
        };

        let segment_positions = vec![Position {
            x: crate::ARENA_WIDTH / 2,
            y: crate::ARENA_HEIGHT / 2,
        }];

        let mut position_candidates = free_positions.clone();
        position_candidates.remove_all(&segment_positions);

        let Some(tile_size) = world.get_resource::<TileSize>() else {
            panic!("TileSize not available");
        };

        let rectangle = Rectangle::new(tile_size.0 as f32, tile_size.0 as f32);
        let mesh =
            world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| meshes.add(rectangle));

        let color = world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
            materials.add(FOOD_COLOR)
        });

        let mut positions: Vec<Position> = Vec::new();

        for _ in 0..AMOUNT_OF_FOOD {
            match position_candidates.positions.pop() {
                None => break,
                Some(pos) => {
                    world
                        .spawn((Mesh2d(mesh.clone()), MeshMaterial2d(color.clone())))
                        .insert(Food)
                        .insert(OnGameScreen)
                        .insert(pos);
                    positions.push(pos);
                }
            }
        }

        let Some(mut free_positions) = world.get_resource_mut::<FreePositions>() else {
            panic!("FreePositions not available");
        };
        free_positions.remove_all(&positions);
    });
}
