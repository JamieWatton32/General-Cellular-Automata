use crate::{api::Api, materials::Material};




//A cell is a object that contains a material and a rgb color. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell{
    pub material:Material, // material of the cell
    pub count:u8,
} 

pub static IS_EMPTY: Cell = Cell{
    material:Material::Empty,
    count:0,
};


impl Cell{
    pub fn _new(material: Material)-> Cell{
        Cell {
            material,
            count:0,
        }
    }

    pub fn update(&self, api:Api){
        self.material.update_material(*self, api)
    }

   
    
}


