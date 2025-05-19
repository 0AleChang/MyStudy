use bevy::{ecs::entity::Entity, platform::collections::HashMap};

// Cartesian Coordinate.
pub type MyCord = (i32,i32);

const GRID_SIZE: u32 = 10;
const GRID_SIZE_U: usize = GRID_SIZE as usize;
struct Grid{
    data : HashMap<MyCord, Option<Entity>>
}

impl Grid {
    fn new() -> Grid{
        Grid { data: HashMap::new() }
    }
}

struct Chunk{
    map : [[MyCord;GRID_SIZE_U];GRID_SIZE_U],
}

impl Chunk {
    fn new()->Chunk{
        Chunk { map:[[(0,0);GRID_SIZE_U];GRID_SIZE_U]  }
    }
    fn init(start : MyCord){
        
    }
}