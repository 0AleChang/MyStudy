use std::io::{self, Write};

use bevy::prelude::*;

use super::{my_player::Player, PlayerInputEvent};



pub fn move_player(
    mut player: Query<(&mut Transform, &Player), With<Player>>,
    mut receiver: EventReader<PlayerInputEvent>,
) {
    let Ok((mut transform, player_info)) = player.single_mut() else {
        dbg!("Erro, não único");
        return;
    };
    let Some(message) = receiver.read().next() else {
        return;
    };
    if !(player_info.id == message.id_) {
        return;
    }
    let key_board_input = message.key;
    let speed = 2.0; // Velocidade de movimento por frame

    if key_board_input == KeyCode::KeyA {
        transform.translation.x -= speed;
    }
    if key_board_input == KeyCode::KeyW {
        transform.translation.y += speed;
    }
    if key_board_input == KeyCode::KeyS {
        transform.translation.y -= speed;
    }
    if key_board_input == KeyCode::KeyD {
        transform.translation.x += speed;
    }
}

pub fn move_cam(
    mut player: Query<(&mut Transform, &Camera)>,
    mut receiver: EventReader<PlayerInputEvent>,
) {
    let Ok((mut transform, player_info)) = player.single_mut() else {
        dbg!("Erro, não único");
        return;
    };
    let Some(message) = receiver.read().next() else {
        return;
    };
  
    let key_board_input = message.key;
    let speed = 2.0; // Velocidade de movimento por frame

    if key_board_input == KeyCode::ArrowLeft {
        transform.translation.x -= speed;
    }
    if key_board_input == KeyCode::ArrowUp {
        transform.translation.y += speed;
    }
    if key_board_input == KeyCode::ArrowDown {
        transform.translation.y -= speed;
    }
    if key_board_input == KeyCode::ArrowRight {
        transform.translation.x += speed;
    }
}