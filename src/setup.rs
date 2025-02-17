use std::cmp;

use crate::resources::{DefaultFontHandle, TileSize};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");
    commands.insert_resource(DefaultFontHandle(font));

    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.cursor_options.visible = false;

    let window_width = window.width() as i32;
    let window_height = window.height() as i32;
    info!(window_width);
    info!(window_height);

    let tile_size = cmp::min(
        window_width / crate::ARENA_WIDTH,
        window_height / crate::ARENA_HEIGHT,
    );
    info!(tile_size);
    commands.insert_resource(TileSize(tile_size));
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
