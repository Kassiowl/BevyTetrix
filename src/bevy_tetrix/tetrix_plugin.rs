use std::process::Command;

use bevy::{DefaultPlugins, app::{Plugin, App, Update, Startup}, ecs::system::{Commands, Res}, asset::AssetServer, core_pipeline::core_2d::Camera2dBundle, time::Time};
use crate::game_rules::{tetrix_rules::tetrix_rules::generate_random_tetromino, entities::game::Game};
use crate::bevy_tetrix::load_board::load_board;
use crate::bevy_tetrix::camera::spawn_2d_camera;
use bevy::prelude::*;

use super::render_tetromino::{render_tetromino, TetrominoType};
pub struct TetrixPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver,
}
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimulationState{
    #[default]
    Paused,
    Running,
}



struct game_entity
{
    game: Game
}

#[derive(Event)]
struct SpawnTetrominio();


fn spawn_tetromino_setup(mut commands: Commands, asset_server: Res<AssetServer>)
{

    render_tetromino(&mut commands, &asset_server, TetrominoType::I);
    render_tetromino(&mut commands, &asset_server, TetrominoType::J);
    render_tetromino(&mut commands, &asset_server, TetrominoType::L);
    render_tetromino(&mut commands, &asset_server, TetrominoType::O);
    render_tetromino(&mut commands, &asset_server, TetrominoType::S);
    render_tetromino(&mut commands, &asset_server, TetrominoType::T);
    render_tetromino(&mut commands, &asset_server, TetrominoType::Z);

}

fn spawn_tetrominio(mut commands: Commands, asset_server: Res<AssetServer>, 
    mut ev_reader_spawn_tetrominio: EventReader<SpawnTetrominio>
)    {
    
    for ev in ev_reader_spawn_tetrominio.read() {
        
        let tetromino = generate_random_tetromino();
     
        match tetromino.tetromino_type
        {
            // 1 => render_tetromino_i(commands, asset_server),
            // 2 => render_tetromino_j(commands, asset_server),
            // 3 => render_tetromino_l(commands, asset_server),
            // 4 => render_tetromino_o(commands, asset_server),
            // 5 => render_tetromino_s(commands, asset_server),
            // 6 => render_tetromino_t(commands, asset_server),
            // 7 => render_tetromino_z(commands, asset_server),
            _ => println!("Error"),
        }
    }
    

}



impl Plugin for TetrixPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>();
        app.add_state::<AppState>();
        app.add_systems(Startup, (load_board, spawn_tetromino_setup, spawn_2d_camera));
        app.add_plugins(DefaultPlugins);
    }
}
