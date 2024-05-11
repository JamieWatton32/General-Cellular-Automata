
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
    let dx = 0;// no horizontal
    let dy = 1;// just vertical

    let below = api.get_neighbour(dx, dy); // retrieving cell data from pos (cell.x+dx, cell.y+dy)
    println!("below: {:?} x:{},y:{}",below,api.x,api.y);
    if below.material == Material::Empty{
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, 1, cell);
    } else{
        println!("below: {:?} x:{},y:{}",below,api.x,api.y);
        api.set_cell(0,0, cell);
    }
   
}


