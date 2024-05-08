use crate::materials::{self, Material};


//A cell is a object that contains a material and a rgb color. 
#[derive(Clone, Debug, Copy)]
pub struct Cell{
    pub material:Material,
    pub red:u8,
    pub green:u8,
    pub blue:u8,
} 

impl Cell{
    pub fn new(material:Material,red: u8, green: u8, blue: u8) -> Cell{
        Cell{
            material,
            red,
            green,
            blue,
        }
    }
    // pub fn update(&self, sim: Simulator) {
    //     self.material.update(*self)
    // }
}

pub static IS_EMPTY: Cell = Cell{
    material: Material::Empty,
    red:0,
    green:0,
    blue:0,
};

//Simulator is the connection between the cells and the grid. It comp[utes the changes on the grid between the cells
//it will be the primary interface used to implement the logic. 
#[derive(Debug)]
pub struct Simulator<'a>{
    pub(crate) x: i32,
    pub(crate) y:i32,
    pub(crate) grid: &'a mut Grid
}



impl <'a>Simulator<'a>{
   
}

    
//Grid is the thing holds each cell, it contains the data for the simulation.
#[derive(Debug)]
pub struct Grid{
    pub width: i32,
    pub height:i32,
    pub cells: Vec<Cell>,
}

//public methods

impl Grid {
    pub fn new(width: i32, height: i32) -> Grid{
        let cells:Vec<Cell> = (0..width*height).map(|_i| IS_EMPTY).collect(); // this will create a vector of cells that is 
        Grid{
            width:width,
            height:height,
            cells:cells
        }
    }

    pub fn height(&self) -> i32 {
        self.height
    }
    
    pub fn width(&self) ->i32 {
        self.width
    }
}




