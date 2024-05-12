
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
            Material::Empty => {}
            Material::Sand => {update_sand(cell,api)},
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
    
   



// pub static _IS_ALIVE: Cell = Cell{
//     material: Material::Sand,
//     active:true,
// };
// pub fn _game_life(cell:Cell, mut api: Api){
//     let up = api.get_neighbour(0, -PARTICLE_SIZE);
//     let down = api.get_neighbour(0, PARTICLE_SIZE);
//     let left = api.get_neighbour(-PARTICLE_SIZE, 0);
//     let right = api.get_neighbour(PARTICLE_SIZE, 0);
//     let left_diagonal_up = api.get_neighbour(-PARTICLE_SIZE, -PARTICLE_SIZE);
//     let right_diagonal_up = api.get_neighbour(PARTICLE_SIZE, -PARTICLE_SIZE);
//     let left_diagonal_down = api.get_neighbour(-PARTICLE_SIZE, PARTICLE_SIZE);
//     let right_diagonal_down = api.get_neighbour(PARTICLE_SIZE, PARTICLE_SIZE);

//     let live_neighbours = vec![up, down, left, right,
//     left_diagonal_up, right_diagonal_up,
//     left_diagonal_down, right_diagonal_down]
//     .iter().filter(|&cell| *cell == IS_ALIVE).count();

//     if live_neighbours < 2 || live_neighbours > 3{
//         api.set_cell(0, 0, cell);
//     } else if live_neighbours > 3{
//         api.set_cell(0, 0, IS_EMPTY);
//     } else if live_neighbours == 3 && cell.material == Material::Empty{
//         api.set_cell(0, 0, IS_ALIVE);
//     }    

// }



