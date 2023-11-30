use bevy::{DefaultPlugins, app::{Plugin, App}};

pub struct TetrixPlugin;

impl Plugin for TetrixPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
