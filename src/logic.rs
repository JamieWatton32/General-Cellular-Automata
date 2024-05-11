use crate::{grid::Grid, materials::Material};




//A cell is a object that contains a material and a rgb color. 
#[derive(Clone, Debug, Copy,PartialEq)]
pub struct Cell{
    pub material:Material, // material of the cell
    pub x: i32,
    pub y: i32,
} 

impl Cell{
    pub fn new(material: Material,x:i32,y:i32)-> Cell{
        Cell {
            material,
            x,
            y
        }
    }

    pub fn update_cells(&self, grid: &Grid) {
        self.material.update_material(self, grid);
    }
    
}


