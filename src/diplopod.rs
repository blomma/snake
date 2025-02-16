mod setup;

use bevy::{
    app::{App, Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{Vec2, Vec3},
    state::{condition::in_state, state::OnEnter},
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self, Rectangle},
};

use crate::{
    components::{OnGameScreen, Position},
    food::{Food, SpawnFood},
    gameover::GameOver,
    resources::{FreePositions, LastTailPosition, TileSize, UpperLeft},
    wall::Wall,
    GameState, Phase,
};

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

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DiplopodPosition {
    pub x: i32,
    pub y: i32,
}

impl DiplopodPosition {
    pub fn to_position(self) -> Position {
        Position {
            x: self.x / crate::CONSUMABLE_SCALE_FACTOR,
            y: self.y / crate::CONSUMABLE_SCALE_FACTOR,
        }
    }
}

pub struct DiplopodPlugin;

impl Plugin for DiplopodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup::init)
            .add_systems(
                Update,
                (diplopod_position_translation,)
                    .after(Phase::Movement)
                    .run_if(in_state(GameState::Game)),
            );
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

pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_food_writer: EventWriter<SpawnFood>,
    mut game_over_writer: EventWriter<GameOver>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&DiplopodPosition, With<DiplopodHead>>,
    wall_positions: Query<(Entity, &Position), With<Wall>>,
    mut free_positions: ResMut<FreePositions>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if *food_pos == head_pos.to_position() {
                commands.entity(ent).despawn();
                free_positions.positions.push(*food_pos);
                free_positions.shuffle();

                growth_writer.send(Growth(1));
                spawn_food_writer.send(SpawnFood);
            }
        }

        for (_ent, wall_pos) in wall_positions.iter() {
            if *wall_pos == head_pos.to_position() {
                game_over_writer.send(GameOver);
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
) {
    if let Some(growth) = growth_reader.read().next() {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(tile_size.0 as f32),
            origin: shapes::RectangleOrigin::Center,
            radii: None,
        };

        for _ in 0..growth.0 {
            segments.0.push(spawn_segment(
                &mut commands,
                crate::DIPLOPOD_COLOR,
                last_tail_position.0.unwrap(),
                &shape,
            ));
        }
    }
}

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut DiplopodPosition>,
    segments: ResMut<DiplopodSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOver>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<DiplopodPosition>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_pos.x += head.direction.x as i32;
        head_pos.y += head.direction.y as i32;

        if segment_positions.contains(&head_pos)
            && (head.direction.x != 0.0 || head.direction.y != 0.0)
        {
            game_over_writer.send(GameOver);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });

        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

fn spawn_segment(
    commands: &mut Commands,
    color: Color,
    position: DiplopodPosition,
    shape: &Rectangle,
) -> Entity {
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(shape),
                ..default()
            },
            Fill::color(color),
            Stroke::color(color),
        ))
        .insert(DiplopodSegment)
        .insert(position)
        .insert(OnGameScreen)
        .id()
}
