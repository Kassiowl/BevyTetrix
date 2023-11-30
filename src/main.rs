use bevy::app::App;
use bevy_tetrix::tetrix_plugin::TetrixPlugin;
mod game_rules;
mod bevy_tetrix;
fn main() {
    App::new()
    .add_plugins(TetrixPlugin)
    .run();
}

