use std::os::linux::raw::stat;

use bevy::{ecs::{component::Component, entity::Entity, resource::Resource}, platform::collections::HashMap};



const SIZE: usize = 4;
const SIZEI32: i32 = SIZE as i32;
type CHUNKTYPE = [[Tile; SIZE];SIZE];
type COORDINATESTYPE = (i32,i32);
type NEIGHBOR = [COORDINATESTYPE;8];


#[derive(Resource)]
pub struct WorldMap{
    map : HashMap<(i32,i32),Chunk> ,
}
//Data hold 
#[derive(Debug)]
struct Chunk{
    content : CHUNKTYPE,
    neighbor: NEIGHBOR,
    chunk_type: Biom,
}
#[derive(Debug, Component, Clone, Copy)]
struct Tile{
    pub coordinate: COORDINATESTYPE,
    pub neighbor : NEIGHBOR,
    pub entity_slot : Option<Entity>,
}
#[derive(Debug)]
enum Biom{
    Blank,
}


impl WorldMap {
    pub fn new()-> WorldMap{
        WorldMap { map: HashMap::new() }
    }
}
impl Chunk{
    fn new(coord : COORDINATESTYPE)->Chunk{
        let mut array_tile = [[Tile::new((0,0));SIZE];SIZE];
    
        for y in 0..SIZEI32 {
            for x in 0..SIZEI32{
                array_tile[x as usize][y as usize].coordinate = (x* coord.0 ,y * coord.1);
                array_tile[x as usize][y as usize].neighbor = neighbors(coord.0, coord.1, SIZEI32)
            }
        }


        Chunk { content: array_tile , chunk_type: Biom::Blank, neighbor: [(0,0);8] }
    }
}
impl Tile {
    fn new(coord : COORDINATESTYPE)-> Tile{
        Tile {coordinate: (0,0) , neighbor: [(0,0);8], entity_slot: None }
    }
}


fn neighbors(x: i32, y: i32, n: i32) -> [(i32, i32); 8] {
    [
        (x - n, y + n), // top-left         index : 0
        (x    , y + n), // top-center       index : 1
        (x + n, y + n), // top-right        index : 2
        (x - n, y    ), // center-left      index : 3
        (x + n, y    ), // center-right     index : 4
        (x - n, y - n), // bottom-left      index : 5
        (x    , y - n), // bottom-center    index : 6
        (x + n, y - n), // bottom-right     index : 7
    ]
}