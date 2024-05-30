use rand::{random, Rng};
use rand::prelude::*;
use rand_chacha::{ChaCha20Rng, ChaCha8Rng};

use crate::{
    api::Api,
    logic::{Cell, IS_EMPTY},
    PARTICLE_SIZE,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Material {
    Empty,
    Sand,
    Water,
    Wall,
}
//ADD COUNTER
impl Material {
    pub fn update_material(&self, cell: Cell, api: Api) {
      
        match self {
            Material::Empty => {},
            Material::Wall => {},
            Material::Sand => update_sand(cell, api),
            Material::Water => update_water(cell, api),
            
        }
    }
}
pub fn update_sand(cell: Cell, mut api: Api) {
    let dx = rand::thread_rng().gen_range(-1..=1);

    let below = api.get_neighbour(0, 1);
    let diagonal = api.get_neighbour(dx, 1);

    if below.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, 2*1, cell);// move down by 2 for speed
    
    } else if diagonal.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(dx, 1, cell);
    } else if below.material == Material::Water {
        api.set_cell(0, 0, below);
        api.set_cell(0, 1, cell);
    } else {
        api.set_cell(0, 0, cell);
    }
}

fn update_water(cell: Cell, mut api: Api) {
    let dx = rand::thread_rng().gen_range(-6..=6);

    let below = api.get_neighbour(0, 1);
    let diagonal = api.get_neighbour(dx, 1);

    if below.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, 2, cell); // move down by 2 for speed
    } else if below.material == Material::Water && diagonal.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(dx, 1, cell);
    } else if below.material == Material::Sand && diagonal.material == Material::Sand {
        api.set_cell(0, 0, cell);
    } else if diagonal.material == Material::Sand{
        api.set_cell(dx, 0, cell);
    }else{
        api.set_cell(0, 0, cell);
    }
}
