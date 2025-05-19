use bevy::{
    asset::{AssetServer, Handle}, ecs::{
        component::Component, event::{Event, EventWriter}, query::With, system::{Commands, Query, Res, ResMut}
    }, image::Image, input::{keyboard::KeyCode, ButtonInput}, math::{Vec2, Vec3}, sprite::Sprite, transform::components::Transform
};

#[derive(Component)]
pub struct Player;



pub fn spawn_player(
   mut commands: Commands,
   //mut meshes: ResMut<Assets<Mesh>>,
   //mut materials: ResMut<Assets<StandardMaterial>>,
   asset_server: Res<AssetServer>,
   
){
    let handle_image:Handle<Image> = asset_server.load("asset03player.png"); 
    let player = (Sprite{
        image:handle_image,
        custom_size: Some(Vec2::new(5.0, 5.0)),
        ..Default::default()
    },Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 { x: 20.0, y: 20.0, z: 20.0 }),Player{}

    );
    commands.spawn(player);
}
#[derive(Event)]
pub enum MovePlayer{
    KeyA,
    KeyD,
    KeyW,
    KeyS,
}


pub fn move_player(
    mut _command: Commands,
    mut player: Query<&mut Transform, With<Player>>,
    mut sender: EventWriter<MovePlayer>,
    key_board_input: Res<ButtonInput<KeyCode>>,
) {
    let  Ok(mut player_transform) = player.single_mut() else {
        dbg!("Erro, não unico");
        return;
    };
    let mut vec_tran : Vec3 = player_transform.translation;
    if key_board_input.pressed(KeyCode::KeyA) {
        vec_tran.x += -1.0;
    }
    if key_board_input.pressed(KeyCode::KeyW) {
        vec_tran.y += 1.0; 
    }
    if key_board_input.pressed(KeyCode::KeyS) {
        vec_tran.y += -1.0; 
    }
    if key_board_input.pressed(KeyCode::KeyD) {
        sender.write(MovePlayer::KeyD);
    }
    player_transform.translation = vec_tran;
}
