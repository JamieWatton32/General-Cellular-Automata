
use rand::{random, Rng};
use sdl2::sys::CurrentTime;


use crate::{api::Api,logic::{Cell, IS_EMPTY}, PARTICLE_SIZE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Material {
    Empty,
    Sand,
}

impl Material {
    pub fn update_material(&self, cell:Cell, api:Api) {
        match self {
            Material::Empty => {game_life(cell,api)}
            Material::Sand => {game_life(cell,api)},
        }
    }
}

pub fn update_sand(cell: Cell, mut api:Api) {
    let dx = rand::thread_rng().gen_range(-PARTICLE_SIZE..=PARTICLE_SIZE);
    
    let below = api.get_neighbour(0, PARTICLE_SIZE);
    let diagonal = api.get_neighbour(dx, PARTICLE_SIZE);
    
    match below.material{
        Material::Empty =>{
            api.set_cell(0, 0, IS_EMPTY);
            api.set_cell(0, PARTICLE_SIZE, cell);
        },
        Material::Sand =>{
            match diagonal.material{
                Material::Empty =>{
                    api.set_cell(0, 0, IS_EMPTY);
                    api.set_cell(dx, PARTICLE_SIZE, cell);
                },
                Material::Sand =>{
                    api.set_cell(0, 0, cell);
                }  
            }
        }  
    }     
} 

pub static IS_ALIVE: Cell = Cell{
    material: Material::Sand,
};
pub fn game_life(cell:Cell, mut api: Api){
    let up = api.get_neighbour(0, -PARTICLE_SIZE).material;
    let down = api.get_neighbour(0, PARTICLE_SIZE).material;
    let left = api.get_neighbour(-PARTICLE_SIZE, 0).material;
    let right = api.get_neighbour(PARTICLE_SIZE, 0).material;
    let left_diagonal_up = api.get_neighbour(-PARTICLE_SIZE, -PARTICLE_SIZE).material;
    let right_diagonal_up = api.get_neighbour(PARTICLE_SIZE, -PARTICLE_SIZE).material;
    let left_diagonal_down = api.get_neighbour(-PARTICLE_SIZE, PARTICLE_SIZE).material;
    let right_diagonal_down = api.get_neighbour(PARTICLE_SIZE, PARTICLE_SIZE).material;

    let live_neighbours = vec![up, down, left, right,
    left_diagonal_up, right_diagonal_up,
    left_diagonal_down, right_diagonal_down]
    .iter().filter(|&state| *state == Material::Sand).count();

    if live_neighbours < 2 || live_neighbours > 3{
        api.set_cell(0, 0, cell);
    } else if live_neighbours > 3{
        api.set_cell(0, 0, IS_EMPTY);
    } else if live_neighbours == 3 && cell.material == Material::Empty{
        api.set_cell(0, 0, IS_ALIVE);
    }
    

}



