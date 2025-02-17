use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        schedule::{common_conditions::on_event, IntoSystemConfigs},
        system::{Query, ResMut},
    },
    state::{condition::in_state, state::NextState},
};

use crate::{
    components::{GameState, Phase},
    diplopod::DiplopodSegment,
    resources::{FreePositions, Highscore, Lastscore},
};

#[derive(Event)]
pub struct GameOver;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            game_over
                .after(Phase::Movement)
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<GameOver>),
        );
    }
}

fn game_over(
    mut reader: EventReader<GameOver>,
    segments: Query<Entity, With<DiplopodSegment>>,
    mut free_positions: ResMut<FreePositions>,
    mut game_state: ResMut<NextState<GameState>>,
    mut lastscore: ResMut<Lastscore>,
    mut highscore: ResMut<Highscore>,
) {
    if reader.read().next().is_none() {
        return;
    };

    lastscore.0 = 0;
    for _ in segments.iter() {
        lastscore.0 += 1;
    }

    if lastscore.0 > highscore.0 {
        highscore.0 = lastscore.0;
    }

    free_positions.reset();

    game_state.set(GameState::Highscore);
}
