use sdl2::render::Canvas;
use sdl2::sys::Window;


use crate::api::Api;
use crate::logic::{Cell, IS_EMPTY};
use crate::PARTICLE_SIZE;
const UPDATE_FLAG_MASK: u8 = 0b10000000;
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let cells:Vec<Cell> = (0..width*height).map(|_| IS_EMPTY).collect();
        Grid {
            width,
            height,
            cells,
        }
    }
   
    
    pub fn update_grid(&mut self) {
        for x in 0..self.width-PARTICLE_SIZE {
            for y in 0..self.height-PARTICLE_SIZE {
                let cell = self.get_cell_state(x, y);
                Grid::update_cell(cell, Api { x, y, grid: self });
                
            }
        }
    }
      
    

    pub fn get_cell_state(&self, x: i32, y: i32) -> Cell {
       let idx = self.get_current_index(x, y);
       self.cells[idx]

    }

    pub fn get_current_index(&self,x:i32,y:i32) -> usize{
        (x * self.width + y) as usize
    } 

    fn update_cell(cell: Cell, api: Api) {
        cell.update(api);
    }
}
