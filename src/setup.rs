use std::cmp;

use crate::resources::{DefaultFontHandle, TileSize, UpperLeft};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");
    commands.insert_resource(DefaultFontHandle(font));

    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;

        let tile_size = cmp::min(
            window.width() as i32 / crate::ARENA_WIDTH,
            window.height() as i32 / crate::ARENA_HEIGHT,
        );
        commands.insert_resource(TileSize(tile_size));

        let upper_left_x = (window.width() as i32 - (crate::ARENA_WIDTH - 1) * tile_size) / 2;
        let upper_left_y = (window.height() as i32 - (crate::ARENA_HEIGHT - 1) * tile_size) / 2;

        let upper_left = UpperLeft {
            x: upper_left_x,
            y: upper_left_y,
        };

        commands.insert_resource(upper_left);
    }
}

pub fn set_default_font(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    default_font_handle: Res<DefaultFontHandle>,
) {
    if let Some(font) = fonts.remove(&default_font_handle.0) {
        fonts.insert(&TextFont::default().font, font);
        commands.remove_resource::<DefaultFontHandle>();
    }
}
