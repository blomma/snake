mod camera;
mod components;
mod diplopod;
mod events;
mod food;
mod gameover;
mod graphics;
mod highscore;
mod menu;
mod player_input;
mod resources;
mod setup;
mod wall;

use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;
use components::{GameState, OnGameScreen, Phase};
use diplopod::{eat, growth, movement, DiplopodSegments, Growth};
use events::*;
use food::{spawn::spawn_food, SpawnFood};
use gameover::GameOver;
use resources::*;

pub const TITLE: &str = "diplopod";

pub const CONSUMABLE_WIDTH: i32 = 22;
pub const CONSUMABLE_HEIGHT: i32 = 22;
pub const ARENA_WIDTH: i32 = CONSUMABLE_WIDTH + 1;
pub const ARENA_HEIGHT: i32 = CONSUMABLE_HEIGHT + 1;

pub const DIPLOPOD_COLOR: Color = Color::Srgba(ORANGE);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ShapePlugin,
            highscore::HighscorePlugin,
            gameover::GameOverPlugin,
            menu::MenuPlugin,
            graphics::GraphicsPlugin,
            food::FoodPlugin,
            wall::WallPlugin,
            diplopod::DiplopodPlugin,
            camera::CameraPlugin,
        ))
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            setup::set_default_font.run_if(resource_exists::<resources::DefaultFontHandle>),
        )
        .add_systems(
            Update,
            (
                (
                    player_input::keyboard,
                    player_input::gamepad,
                    player_input::pause,
                )
                    .in_set(Phase::Input)
                    .run_if(in_state(GameState::Game))
                    .run_if(not(resource_exists::<Paused>)),
                (player_input::unpause,)
                    .in_set(Phase::Input)
                    .run_if(in_state(GameState::Game))
                    .run_if(resource_exists::<Paused>),
            ),
        )
        .add_systems(
            FixedUpdate,
            ((
                movement.after(Phase::Input).in_set(Phase::Movement),
                eat,
                spawn_food.run_if(on_event::<SpawnFood>),
                graphics::show_message,
                growth.run_if(on_event::<Growth>),
            )
                .chain(),)
                .run_if(in_state(GameState::Game))
                .run_if(not(resource_exists::<Paused>)),
        )
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
        .init_state::<crate::GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DiplopodSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(FreePositions::new(CONSUMABLE_WIDTH, CONSUMABLE_HEIGHT))
        .insert_resource(Time::<Fixed>::from_seconds(0.075))
        .add_event::<GameOver>()
        .add_event::<Growth>()
        .add_event::<SpawnFood>()
        .add_event::<ShowMessage>();
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
