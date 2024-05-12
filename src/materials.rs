
use rand::{random, Rng};
use sdl2::libc::rand;

use crate::{api::Api,logic::{Cell, IS_EMPTY}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    let dx = rand::thread_rng().gen_range(-1..=1);
    let dy = 1;// just vertical

    let below = api.get_neighbour(0, dy); // retrieving cell data from pos (cell.x+dx, cell.y+dy)
    if below.material == Material::Empty{
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, dy, cell);
        return
        
    } else if api.get_neighbour(dx, 0).material == Material::Sand{
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(1, 0, cell);
    }
    else{
        api.set_cell(0, -1, cell);
    }
         
} 
   



