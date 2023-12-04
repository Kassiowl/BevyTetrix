
use bevy::prelude::*;
pub fn render_tetromino_i(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/I.png"),
        transform: Transform::from_xyz(0., 360., 0.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/I.png"),
        transform: Transform::from_xyz(0., -360., 0.),
        ..default()
    });
}
pub fn render_tetromino_j(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/I.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/I.png"),
        ..default()
    });
}
pub fn render_tetromino_l(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/I.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/I.png"),
        ..default()
    });
}

pub fn render_tetromino_o(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/I.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/I.png"),
        ..default()
    });
}
pub fn render_tetromino_s(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/S.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/S.png"),
        ..default()
    });
}
pub fn render_tetromino_t(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/T.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/T.png"),
        ..default()
    });
}
pub fn render_tetromino_z(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Shape Blocks/Z.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Ghost/Z.png"),
        ..default()
    });
}
