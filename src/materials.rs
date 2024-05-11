
use std::io::Empty;

use crate::{grid::Grid, logic::Cell, WINDOW_HEIGHT};


#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Material {
    Empty,
    Sand,
}

impl Material {
    pub fn update_material(&self, cell: &Cell, grid: &Grid) {
        match self {
            Material::Empty => {}
            Material::Sand => {update_sand(cell.clone(),grid.clone())},
        }
    }
}

pub fn update_sand(mut cell: Cell,grid:Grid) {
    if cell.x > WINDOW_HEIGHT -1 
    || cell.y > WINDOW_HEIGHT - 1{
        panic!("out of bounds!");
    }
    println!("{:?} cell in sand ",cell);

    let dx = 0;
    let dy = 1;
    if grid.get_state(dx, dy,cell).material == Material::Empty{
        cell.material = Material::Sand;
        cell.x += dx as i32;
        cell.y += dy as i32;
    }
}

