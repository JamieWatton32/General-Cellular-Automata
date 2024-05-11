
use crate::{api::Api, logic::{Cell, IS_EMPTY}};


#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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
    let dx = 0;// no horizontal
    let dy = 1;// just vertical

    let below = api.get_neighbour(dx, dy); // retrieving cell data from pos (cell.x+dx, cell.y+dy)
    

    if below == IS_EMPTY{
        api.get_neighbour(dx,dy);
        api.set_cell(dx, dy, cell)
    }
   
}

