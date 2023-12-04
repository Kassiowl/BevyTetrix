use std::process::Command;

use bevy::{DefaultPlugins, app::{Plugin, App, Update, Startup}, ecs::system::{Commands, Res}, asset::AssetServer, core_pipeline::core_2d::Camera2dBundle};
use crate::game_rules::tetrix_rules::tetrix_rules::generate_random_tetromino;
use crate::bevy_tetrix::render_tetromino::render_tetromino_i;
use crate::bevy_tetrix::load_board::load_board;
use crate::bevy_tetrix::camera::spawn_2d_camera;

use super::render_tetromino::{render_tetromino_j, render_tetromino_z, render_tetromino_t, render_tetromino_s, render_tetromino_o, render_tetromino_l};
pub struct TetrixPlugin;

fn spawn_tetrimonio(mut commands: Commands, asset_server: Res<AssetServer>){
    let tetromino = generate_random_tetromino();

    match tetromino.tetromino_type
    {
        1 => render_tetromino_i(commands, asset_server),
        2 => render_tetromino_j(commands, asset_server),
        3 => render_tetromino_l(commands, asset_server),
        4 => render_tetromino_o(commands, asset_server),
        5 => render_tetromino_s(commands, asset_server),
        6 => render_tetromino_t(commands, asset_server),
        7 => render_tetromino_z(commands, asset_server),
        _ => println!("Error"),
    }


}

impl Plugin for TetrixPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_2d_camera, load_board));
        app.add_systems(Update, spawn_tetrimonio);
        app.add_plugins(DefaultPlugins);
    }
}
