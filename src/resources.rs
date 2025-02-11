use bevy::prelude::*;
use rand::rng;
use rand::seq::SliceRandom;

use crate::components::{DiplopodPosition, Position};

#[derive(Default, Resource)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Clone, Resource, Default)]
pub struct FreePositions {
    pub positions: Vec<Position>,
    width: i32,
    height: i32,
}

impl FreePositions {
    pub fn new(width: i32, height: i32) -> Self {
        let positions = Self::new_positions(width, height);
        Self {
            positions,
            width,
            height,
        }
    }

    fn new_positions(width: i32, height: i32) -> Vec<Position> {
        let mut positions = Vec::new();

        for x in 0..width {
            for y in 0..height {
                positions.push(Position { x, y });
            }
        }

        positions.shuffle(&mut rng());

        positions
    }

    pub fn shuffle(&mut self) {
        self.positions.shuffle(&mut rng());
    }

    pub fn remove(&mut self, position: &Position) {
        self.positions.retain(|&p| p != *position);
    }

    pub fn remove_all(&mut self, positions: &Vec<Position>) {
        for position in positions {
            self.remove(position);
        }
    }

    pub fn reset(&mut self) {
        self.positions = Self::new_positions(self.width, self.height);
    }
}

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<DiplopodPosition>);

#[derive(Default, Resource)]
pub struct LastSpecialSpawn(pub u32);

#[derive(Default, Resource)]
pub struct ImmunityTime(pub u8);

#[derive(Default, Debug, Resource)]
pub struct TileSize(pub i32);

#[derive(Default, Debug, Resource)]
pub struct UpperLeft {
    pub x: i32,
    pub y: i32,
}

#[derive(Resource)]
pub struct DefaultFontHandle(pub Handle<Font>);

#[derive(Default, Resource)]
pub struct Highscore(pub u16);

#[derive(Default, Resource)]
pub struct Lastscore(pub u16);

#[derive(Default, Resource)]
pub struct Paused;
