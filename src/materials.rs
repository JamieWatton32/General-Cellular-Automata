use std::io::IsTerminal;

use rand::{random, Rng};
use sdl2::sys::CurrentTime;

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
}
//ADD COUNTER
impl Material {
    pub fn update_material(&self, cell: Cell, api: Api) {
        match self {
            Material::Empty => {}
            Material::Sand => update_sand(cell, api),
            Material::Water => update_water(cell, api),
        }
    }
}

pub fn update_sand(cell: Cell, mut api: Api) {
    let dx = rand::thread_rng().gen_range(-PARTICLE_SIZE..=PARTICLE_SIZE);

    let below = api.get_neighbour(0, PARTICLE_SIZE);
    let diagonal = api.get_neighbour(dx, PARTICLE_SIZE);

    if below.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, PARTICLE_SIZE, cell);
    } else if diagonal.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(dx, PARTICLE_SIZE, cell);
    } else if below.material == Material::Water {
        api.set_cell(0, 0, below);
        api.set_cell(0, PARTICLE_SIZE, cell);
    } else {
        api.set_cell(0, 0, cell);
    }
}

fn update_water(cell: Cell, mut api: Api) {
    let dx = rand::thread_rng().gen_range(-PARTICLE_SIZE..=PARTICLE_SIZE);

    let below = api.get_neighbour(0, PARTICLE_SIZE);
    let diagonal = api.get_neighbour(dx, PARTICLE_SIZE);

    if below.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, PARTICLE_SIZE, cell);

    } else if below.material == Material::Water && diagonal.material == Material::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(dx+dx, 0, cell);
    } else if below.material == Material::Sand && diagonal.material == Material::Sand {
        api.set_cell(0, 0, cell);
    } else if diagonal.material == Material::Sand{
        api.set_cell(dx+dx, 0, cell);
    
    }else{
        api.set_cell(0, 0, cell);
    }
}
