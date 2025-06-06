mod setup;

use crate::{
    ARENA_HEIGHT, ARENA_WIDTH, GameState, Phase,
    components::{OnGameScreen, Position},
    food::{Food, SpawnFood},
    gameover::GameOver,
    resources::{FreePositions, LastTailPosition, TileSize},
    wall::Wall,
};
use bevy::{color::palettes::css::ORANGE, prelude::*, window::PrimaryWindow};

pub const DIPLOPOD_COLOR: Color = Color::Srgba(ORANGE);

#[derive(Event)]
pub struct Growth(pub u8);

#[derive(Default, Resource)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Component)]
pub struct DiplopodHead {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct DiplopodSegment;

pub struct DiplopodPlugin;

impl Plugin for DiplopodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup::init)
            .add_systems(
                Update,
                (position_translation,)
                    .after(Phase::Movement)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform), With<DiplopodSegment>>,
    tile_size: Res<TileSize>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32, tile_size: f32) -> f32 {
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let Ok(window) = windows.single() else {
        return;
    };

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width(),
                ARENA_WIDTH as f32,
                tile_size.0 as f32,
            ),
            convert(
                pos.y as f32,
                window.height(),
                ARENA_HEIGHT as f32,
                tile_size.0 as f32,
            ),
            1.0,
        );
    }
}

pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_food_writer: EventWriter<SpawnFood>,
    mut game_over_writer: EventWriter<GameOver>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<DiplopodHead>>,
    wall_positions: Query<(Entity, &Position), With<Wall>>,
    mut free_positions: ResMut<FreePositions>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                free_positions.positions.push(*food_pos);
                free_positions.shuffle();

                growth_writer.write(Growth(1));
                spawn_food_writer.write(SpawnFood);
            }
        }

        for (_ent, wall_pos) in wall_positions.iter() {
            if wall_pos == head_pos {
                game_over_writer.write(GameOver);
            }
        }
    }
}

pub fn growth(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<DiplopodSegments>,
    mut growth_reader: EventReader<Growth>,
    tile_size: Res<TileSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(growth) = growth_reader.read().next() else {
        return;
    };

    let size = (tile_size.0 - 4) as f32;
    for _ in 0..growth.0 {
        segments.0.push(
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(size, size))),
                    MeshMaterial2d(materials.add(DIPLOPOD_COLOR)),
                ))
                .insert(DiplopodSegment)
                .insert(last_tail_position.0.unwrap())
                .insert(OnGameScreen)
                .id(),
        );
    }
}

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut Position, With<DiplopodSegment>>,
    segments: ResMut<DiplopodSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOver>,
) {
    let Some((head_entity, head)) = heads.iter_mut().next() else {
        return;
    };

    let segment_positions = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    let mut head_pos = positions.get_mut(head_entity).unwrap();
    head_pos.x += head.direction.x as i32;
    head_pos.y += head.direction.y as i32;

    if segment_positions.contains(&head_pos) && (head.direction.x != 0.0 || head.direction.y != 0.0)
    {
        game_over_writer.write(GameOver);
    }

    segment_positions
        .iter()
        .zip(segments.0.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });

    last_tail_position.0 = Some(*segment_positions.last().unwrap());
}
