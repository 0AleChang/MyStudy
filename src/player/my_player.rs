use bevy::{
    asset::{AssetServer, Handle}, ecs::{
        bundle::Bundle, component::Component, event::{Event, EventWriter}, query::With, system::{command, Commands, Query, Res, ResMut}
    }, image::Image, input::{keyboard::KeyCode, ButtonInput}, math::{Vec2, Vec3}, sprite::Sprite, transform::components::Transform
};

use super::{PlayerInputEvent};

#[derive(Component)]
pub struct Player{
    pub id : u8,
    pub cord: (f32,f32)
}

pub fn player_bandle(start_spawn: (f32,f32),sprite_handle: Handle<Image>)-> impl Bundle{
    let result  = (Sprite{
        image:sprite_handle,
        custom_size: Some(Vec2 { x: 5.0, y: 5.0 }),
        ..Default::default()
    }
    ,Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3{x: 20.0,y : 20.0,z: 20.0}),
    Player{id : 0 ,cord : start_spawn}
    );
  return result;
}


pub fn client_input(mut command: Commands, asset : Res<AssetServer>){
    command.spawn(player_bandle((0.0,0.0), asset.load("asset03player.png")));
}

pub fn capture_key_player(
    mut _command: Commands,
    mut player: Query<&Player, With<Player>>,
    mut sender: EventWriter<PlayerInputEvent>,
    key_board_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_info) = player.single_mut() else {
        dbg!("Erro, não é único");
        return;
    };

    for key in [
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ] {
        if key_board_input.pressed(key) {
            sender.write(PlayerInputEvent {
                id_: player_info.id,
                key,
            });
        }
    }
}
 