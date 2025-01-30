pub mod components;
pub mod control;
pub mod events;
pub mod graphics;
pub mod highscore;
pub mod menu;
pub mod player_input;
pub mod resources;
pub mod setup;

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;
use events::*;
use resources::*;

pub const TITLE: &str = "diplopod";

pub const CONSUMABLE_WIDTH: i32 = 39 + 1;
pub const CONSUMABLE_HEIGHT: i32 = 21 + 1;
pub const CONSUMABLE_SCALE_FACTOR: i32 = 2;
pub const ARENA_WIDTH: i32 = (CONSUMABLE_WIDTH + 1) * CONSUMABLE_SCALE_FACTOR;
pub const ARENA_HEIGHT: i32 = (CONSUMABLE_HEIGHT + 1) * CONSUMABLE_SCALE_FACTOR;
pub const AMOUNT_OF_FOOD: u32 = 16;
pub const AMOUNT_OF_POISON: u32 = 17;
pub const SPECIAL_SPAWN_INTERVAL: u32 = 16;

pub const DIPLOPOD_COLOR: Color = Color::Srgba(ORANGE);
pub const DIPLOPOD_IMMUNE_COLOR: Color = Color::WHITE;
pub const WALL_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const FOOD_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
pub const SUPERFOOD_COLOR: Color = Color::Srgba(BLUE);
pub const POISON_OUTLINE_COLOR: Color = Color::Srgba(RED);
pub const POISON_FILL_COLOR: Color = Color::BLACK;
pub const ANTIDOTE_COLOR: Color = Color::WHITE;

pub const RADIUS_FACTOR: f32 = 0.9;

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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShapePlugin, highscore::HighscorePlugin, menu::MenuPlugin))
            .add_systems(Startup, setup::setup)
            .add_systems(
                Update,
                setup::set_default_font.run_if(resource_exists::<resources::DefaultFontHandle>),
            )
            .add_systems(
                OnEnter(GameState::Game),
                (
                    control::init_diplopod,
                    control::init_wall,
                    control::init_food,
                    control::init_poison,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    (graphics::on_window_created, graphics::on_window_resized),
                    (
                        player_input::keyboard,
                        player_input::gamepad,
                        player_input::pause,
                        control::move_antidote.run_if(on_timer(Duration::from_millis(500))),
                    )
                        .in_set(Phase::Input)
                        .run_if(in_state(GameState::Game))
                        .run_if(not(resource_exists::<Paused>)),
                    (player_input::unpause,)
                        .in_set(Phase::Input)
                        .run_if(in_state(GameState::Game))
                        .run_if(resource_exists::<Paused>),
                    (
                        graphics::diplopod_position_translation,
                        graphics::position_translation,
                    )
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game)),
                    (graphics::rotate_superfood,)
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game))
                        .run_if(not(resource_exists::<Paused>)),
                    (
                        control::limit_immunity.run_if(on_timer(Duration::from_secs(1))),
                        graphics::fade_text.run_if(on_timer(Duration::from_millis(200))),
                    )
                        .run_if(in_state(GameState::Game))
                        .run_if(not(resource_exists::<Paused>)),
                    control::game_over
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game))
                        .run_if(on_event::<GameOver>),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    (
                        control::movement
                            .after(Phase::Input)
                            .in_set(Phase::Movement),
                        control::eat,
                        control::spawn_consumables.run_if(on_event::<SpawnConsumables>),
                        graphics::show_message,
                        control::growth.run_if(on_event::<Growth>),
                    )
                        .chain(),
                    (graphics::change_color, control::control_antidote_sound),
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(not(resource_exists::<Paused>)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .init_state::<crate::GameState>()
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(TileSize::default())
            .insert_resource(UpperLeft::default())
            .insert_resource(DiplopodSegments::default())
            .insert_resource(LastTailPosition::default())
            .insert_resource(LastSpecialSpawn::default())
            .insert_resource(ImmunityTime::default())
            .insert_resource(FreePositions::new(CONSUMABLE_WIDTH, CONSUMABLE_HEIGHT))
            .insert_resource(Time::<Fixed>::from_seconds(0.075))
            .add_event::<GameOver>()
            .add_event::<Growth>()
            .add_event::<SpawnConsumables>()
            .add_event::<ShowMessage>();
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
