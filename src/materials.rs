use crate::logic::{Cell, Simulator, IS_EMPTY};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Material{
    Empty = 0,
    Sand = 1,

}

impl Material{
    pub fn update(&self, cell: Cell, sim: Simulator){
        match self{
            Material::Empty => {},
            Material::Sand => {update_sand(cell, sim)},
        }
    }
}

pub fn update_sand(mut cell: Cell, mut sim: Simulator){
    (cell.red, cell.green, cell.blue) =  (250,240,230);
}