use bevy_prototype_lyon::shapes::Rectangle;
use rand::{rng, Rng};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::*;
use crate::events::*;
use crate::graphics::wall::Wall;
use crate::resources::*;
use crate::OnGameScreen;

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

pub fn init_poison(
    mut commands: Commands,
    mut free_positions: ResMut<FreePositions>,
    tile_size: Res<TileSize>,
) {
    spawn_poison(&mut commands, &mut free_positions, &tile_size);
}

fn spawn_poison(
    commands: &mut Commands,
    free_positions: &mut ResMut<FreePositions>,
    tile_size: &Res<TileSize>,
) {
    let segment_positions = vec![DiplopodPosition {
        x: crate::ARENA_WIDTH / 2,
        y: crate::ARENA_HEIGHT / 2,
    }
    .to_position()];

    let mut position_candidates = free_positions.clone();
    position_candidates.remove_all(&segment_positions);

    spawn_random_poison(
        crate::AMOUNT_OF_POISON,
        commands,
        &mut position_candidates,
        free_positions,
        tile_size,
    );
}

fn spawn_random_poison(
    amount: u32,
    commands: &mut Commands,
    position_candidates: &mut FreePositions,
    free_positions: &mut ResMut<FreePositions>,
    tile_size: &Res<TileSize>,
) {
    let shape = shapes::Circle {
        radius: tile_size.0 as f32 * crate::RADIUS_FACTOR,
        center: Vec2::new(0., 0.),
    };

    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
                commands
                    .spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(crate::POISON_FILL_COLOR),
                        Stroke::new(crate::POISON_OUTLINE_COLOR, 7.),
                    ))
                    .insert(Poison)
                    .insert(OnGameScreen)
                    .insert(pos);
                free_positions.remove(&pos);
            }
        }
    }
}

fn spawn_random_superfood(
    commands: &mut Commands,
    position_candidates: &mut FreePositions,
    free_positions: &mut ResMut<FreePositions>,
    tile_size: &Res<TileSize>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
        path_builder.line_to(tile_size.0 as f32 * Vec2::X);
        path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
        path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
        let cross = path_builder.build();

        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&cross),
                    ..default()
                },
                Stroke::new(crate::SUPERFOOD_COLOR, 7.5),
            ))
            .insert(SuperFood)
            .insert(OnGameScreen)
            .insert(pos);
        free_positions.remove(&pos);
    }
}

fn spawn_random_antidote(
    commands: &mut Commands,
    position_candidates: &mut FreePositions,
    free_positions: &mut ResMut<FreePositions>,
    tile_size: &Res<TileSize>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
        path_builder.line_to(tile_size.0 as f32 * Vec2::X);
        path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
        path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
        let cross = path_builder.build();

        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&cross),
                    ..default()
                },
                Stroke::new(crate::ANTIDOTE_COLOR, tile_size.0 as f32 * 0.9),
            ))
            .insert(AntiDote)
            .insert(OnGameScreen)
            .insert(pos);
        free_positions.remove(&pos);
    }
}

pub fn spawn_consumables(
    mut commands: Commands,
    segments: Res<DiplopodSegments>,
    mut spawn_consumables_reader: EventReader<SpawnConsumables>,
    mut diplopod_positions: Query<&mut DiplopodPosition>,
    positions: Query<&Position>,
    superfood: Query<Entity, With<SuperFood>>,
    antidotes: Query<Entity, With<AntiDote>>,
    mut free_positions: ResMut<FreePositions>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    tile_size: Res<TileSize>,
) {
    if let Some(spawn_event) = spawn_consumables_reader.read().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *diplopod_positions.get_mut(*e).unwrap())
            .map(|p| p.to_position())
            .collect::<Vec<Position>>();

        let mut position_candidates = free_positions.clone();
        position_candidates.remove_all(&segment_positions);

        if spawn_event.regular {
            spawn_random_poison(
                1,
                &mut commands,
                &mut position_candidates,
                &mut free_positions,
                &tile_size,
            );
        }

        let new_size = segments.0.len() as u32 + spawn_event.new_segments as u32;
        if new_size - last_special_spawn.0 > crate::SPECIAL_SPAWN_INTERVAL {
            last_special_spawn.0 =
                (new_size / crate::SPECIAL_SPAWN_INTERVAL) * crate::SPECIAL_SPAWN_INTERVAL;

            for ent in superfood.iter() {
                let position = positions.get(ent).unwrap();
                free_positions.positions.push(*position);
                commands.entity(ent).despawn();
            }
            free_positions.shuffle();

            if last_special_spawn.0 % (crate::SPECIAL_SPAWN_INTERVAL * 2) == 0 {
                for ent in antidotes.iter() {
                    let position = positions.get(ent).unwrap();
                    free_positions.positions.push(*position);
                    commands.entity(ent).despawn();
                }
                free_positions.shuffle();

                spawn_random_antidote(
                    &mut commands,
                    &mut position_candidates,
                    &mut free_positions,
                    &tile_size,
                );
            }

            spawn_random_superfood(
                &mut commands,
                &mut position_candidates,
                &mut free_positions,
                &tile_size,
            );
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

#[allow(clippy::too_many_arguments)]
pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    mut show_message_writer: EventWriter<ShowMessage>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    superfood_positions: Query<(Entity, &Position), With<SuperFood>>,
    poison_positions: Query<(Entity, &Position), With<Poison>>,
    antidote_positions: Query<(Entity, &Position), With<AntiDote>>,
    head_positions: Query<&DiplopodPosition, With<DiplopodHead>>,
    wall_positions: Query<(Entity, &Position), With<Wall>>,
    mut free_positions: ResMut<FreePositions>,
    mut immunity_time: ResMut<ImmunityTime>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if *food_pos == head_pos.to_position() {
                commands.entity(ent).despawn();
                free_positions.positions.push(*food_pos);
                free_positions.shuffle();
                growth_writer.send(Growth(1));

                spawn_consumables_writer.send(SpawnConsumables {
                    regular: true,
                    new_segments: 1,
                });
            }
        }

        for (ent, superfood_pos) in superfood_positions.iter() {
            if *superfood_pos == head_pos.to_position() {
                commands.entity(ent).despawn();
                free_positions.positions.push(*superfood_pos);
                free_positions.shuffle();
                let new_segments = rng().random_range(2..10);
                growth_writer.send(Growth(new_segments));

                show_message_writer.send(ShowMessage {
                    text: new_segments.to_string(),
                    position: *head_pos,
                });

                spawn_consumables_writer.send(SpawnConsumables {
                    regular: false,
                    new_segments,
                });
            }
        }

        for (ent, antidote_pos) in antidote_positions.iter() {
            if *antidote_pos == head_pos.to_position() {
                commands.entity(ent).despawn();
                free_positions.positions.push(*antidote_pos);
                immunity_time.0 += 10;

                commands.spawn((OnGameScreen,));
            }
        }

        for (ent, poison_pos) in poison_positions.iter() {
            if *poison_pos == head_pos.to_position() {
                if immunity_time.0 > 0 {
                    commands.entity(ent).despawn();
                    free_positions.positions.push(*poison_pos);
                    free_positions.shuffle();
                    growth_writer.send(Growth(1));
                } else {
                    game_over_writer.send(GameOver);
                }
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
    immunity_time: Res<ImmunityTime>,
    tile_size: Res<TileSize>,
) {
    let shape = shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    if let Some(growth) = growth_reader.read().next() {
        for _ in 0..growth.0 {
            segments.0.push(spawn_segment(
                &mut commands,
                if immunity_time.0 > 0 {
                    crate::ANTIDOTE_COLOR
                } else {
                    crate::DIPLOPOD_COLOR
                },
                last_tail_position.0.unwrap(),
                &shape,
            ));
        }
    }
}

pub fn move_antidote(
    mut antidotes: Query<&mut Position, With<AntiDote>>,
    mut segment_positions: Query<&mut DiplopodPosition, With<DiplopodSegment>>,
) {
    for mut pos in antidotes.iter_mut() {
        let mut new_pos = *pos;
        match rng().random_range(0..4) {
            0 => new_pos.x -= 1,
            1 => new_pos.x += 1,
            2 => new_pos.y -= 1,
            3 => new_pos.y += 1,
            _ => (),
        }

        if new_pos.x < 1
            || new_pos.x >= crate::CONSUMABLE_WIDTH
            || new_pos.y < 1
            || new_pos.y >= crate::CONSUMABLE_HEIGHT
            || segment_positions
                .iter_mut()
                .map(|p| p.to_position())
                .any(|x| x == new_pos)
        {
            continue;
        }

        pos.x = new_pos.x;
        pos.y = new_pos.y;
    }
}

pub fn limit_immunity(mut immunity_time: ResMut<ImmunityTime>) {
    if immunity_time.0 > 0 {
        immunity_time.0 -= 1;
    }
}
