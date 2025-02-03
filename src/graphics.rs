pub mod antidote;
pub mod diplopod_segments;
pub mod food;
pub mod poison;
pub mod superfood;
pub mod wall;

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;

use bevy_prototype_lyon::prelude::*;

use crate::components::*;
use crate::events::ShowMessage;
use crate::resources::ImmunityTime;
use crate::resources::Paused;
use crate::GameState;
use crate::OnGameScreen;
use crate::Phase;
use crate::TileSize;
use crate::UpperLeft;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (position_translation,)
                .after(Phase::Movement)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            FixedUpdate,
            (change_color)
                .run_if(in_state(GameState::Game))
                .run_if(not(resource_exists::<Paused>)),
        )
        .add_systems(
            Update,
            (fade_text.run_if(on_timer(Duration::from_millis(200))),)
                .run_if(in_state(GameState::Game))
                .run_if(not(resource_exists::<Paused>)),
        );
    }
}

fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
    tile_size: Res<TileSize>,
    upper_left: Res<UpperLeft>,
) {
    if let Ok(window) = windows.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                (pos.x * tile_size.0 * crate::CONSUMABLE_SCALE_FACTOR + upper_left.x
                    - window.width() as i32 / 2
                    + tile_size.0 / 2) as f32,
                (pos.y * tile_size.0 * crate::CONSUMABLE_SCALE_FACTOR + upper_left.y
                    - window.height() as i32 / 2
                    + tile_size.0 / 2) as f32,
                1.0,
            )
        }
    }
}

fn change_color(
    mut query: Query<(&mut Fill, &mut Stroke), With<DiplopodSegment>>,
    immunity_time: Res<ImmunityTime>,
) {
    if immunity_time.0 > 2 {
        for (mut fill, mut stroke) in query.iter_mut() {
            fill.color = crate::DIPLOPOD_IMMUNE_COLOR;
            stroke.color = crate::DIPLOPOD_IMMUNE_COLOR;
        }
    } else if immunity_time.0 > 0 {
        for (mut fill, mut stroke) in query.iter_mut() {
            if fill.color == crate::DIPLOPOD_IMMUNE_COLOR {
                fill.color = crate::DIPLOPOD_COLOR;
                stroke.color = crate::DIPLOPOD_COLOR;
            } else {
                fill.color = crate::DIPLOPOD_IMMUNE_COLOR;
                stroke.color = crate::DIPLOPOD_IMMUNE_COLOR;
            }
        }
    } else {
        for (mut fill, mut stroke) in query.iter_mut() {
            fill.color = crate::DIPLOPOD_COLOR;
            stroke.color = crate::DIPLOPOD_COLOR;
        }
    }
}

fn fade_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadingText)>,
    mut writer: Text2dWriter,
) {
    for (entity, mut fading_text) in query.iter_mut() {
        writer.color(entity, 0).set_alpha(fading_text.0);
        fading_text.0 -= 0.1;

        if fading_text.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn show_message(mut commands: Commands, mut show_message_reader: EventReader<ShowMessage>) {
    if let Some(show_message) = show_message_reader.read().next() {
        commands
            .spawn((
                Text2d::new(&show_message.text),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor::WHITE,
                TextLayout::new_with_justify(JustifyText::Center),
                // ensure that the text is drawn above the diplopod
                Transform::from_translation(Vec3::Z),
            ))
            .insert(show_message.position)
            .insert(OnGameScreen)
            .insert(FadingText(1.0));
    }
}
