use bevy::{app::{Plugin, Startup, Update}, ecs::{component::Component, event::Event, schedule::IntoScheduleConfigs}, input::keyboard::KeyCode};

mod events_handle;
mod my_player;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app 
            .add_event::<PlayerInputEvent>()
            
            .add_systems(Startup, my_player::client_input)
            .add_systems(Update, my_player::capture_key_player)
            .add_systems(Update, (events_handle::move_player,events_handle::move_cam).chain());
        
    }
}


#[derive(Event)]
pub struct  PlayerInputEvent{
    id_ : u8,
    key : KeyCode,
}