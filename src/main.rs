use bevy::prelude::*;
mod player;
mod world;
mod cam;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((Camera2d,Transform::from_xyz(0.0, 0.0, 0.0)));
}