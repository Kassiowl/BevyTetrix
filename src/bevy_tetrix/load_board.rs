use bevy::prelude::*;
pub fn load_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tetromino/Board/Board.png"),
        ..default()
    });
}