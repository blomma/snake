use crate::{GameState, OnGameScreen, components::*, events::ShowMessage, resources::Paused};
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (fade_text.run_if(on_timer(Duration::from_millis(200))),)
                .run_if(in_state(GameState::Game))
                .run_if(not(resource_exists::<Paused>)),
        );
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
    let Some(show_message) = show_message_reader.read().next() else {
        return;
    };

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
