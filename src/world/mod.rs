
use bevy::app::Plugin;


mod grid;


pub struct WorldPlugin;

impl Plugin for WorldPlugin  {
    fn build(&self, app: &mut bevy::app::App) {
        app
          
            .insert_resource(grid::WorldMap::new());
    }
}