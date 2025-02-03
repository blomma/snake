use std::cmp;

use crate::resources::{DefaultFontHandle, TileSize, UpperLeft};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    commands.spawn(Camera2d);

    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");
    commands.insert_resource(DefaultFontHandle(font));

    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;

        tile_size.0 = cmp::min(
            window.width() as i32 / crate::ARENA_WIDTH,
            window.height() as i32 / crate::ARENA_HEIGHT,
        );

        upper_left.x = (window.width() as i32 - (crate::ARENA_WIDTH - 1) * tile_size.0) / 2;
        upper_left.y = (window.height() as i32 - (crate::ARENA_HEIGHT - 1) * tile_size.0) / 2;
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
