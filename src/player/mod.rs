use bevy::{app::{Plugin, Startup, Update}, ecs::component::Component};
use my_player::MovePlayer;

mod my_player;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app 
            .add_event::<MovePlayer>()
            .add_systems(Startup, my_player::spawn_player)
            .add_systems(Update, my_player::move_player);
        
    }
}