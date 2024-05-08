use crate::logic::{self, Cell, Simulator, IS_EMPTY};

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

pub fn update_sand(cell: Cell, mut sim: Simulator){
    //simulate gravity for sand
    println!("cell:{:?}. sim:{:?}. ",cell,sim);
    let below_cell = sim.fetch_cell(0, 1);
    if below_cell.material == Material::Empty{
        sim.set_cell(0, 0, IS_EMPTY);
        sim.set_cell(0, 1, cell);
    }
}