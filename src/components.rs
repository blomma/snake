use bevy::prelude::*;

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

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Poison;

#[derive(Component)]
pub struct SuperFood;

#[derive(Component)]
pub struct AntiDote;

#[derive(Component)]
pub struct FadingText(pub f32);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    Highscore,
}

#[derive(Component)]
pub struct OnGameScreen;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,
    Movement,
}
