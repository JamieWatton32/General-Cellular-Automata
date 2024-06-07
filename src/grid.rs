use crate::api::Api;
use crate::logic::{Cell, IS_EMPTY};
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<Cell>>,
    pub active: u8
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let cells:Vec<Vec<Cell>> = (0..width)
        .map(|_| {
            (0..height)
                .map(|_| IS_EMPTY)
                .collect()
        })
        .collect();
        Grid {
            width,
            height,
            cells,
            active:0,
        }
    }
   
    pub fn update_grid(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell_state(x, y);
                Grid::update_cell(cell, Api { x, y, grid: self });
                
            }
        }
        self.active = self.active.wrapping_add(1); 
    }
      
    pub fn get_cell_state(&self, x: i32, y: i32) -> Cell {
       let idx = self.get_current_index(x, y);
       self.cells[idx.0 as usize][idx.1 as usize]

    }

    pub fn get_current_index(&self,x:i32,y:i32) -> (i32,i32){
        (x,y)
    } 

    fn update_cell(cell: Cell, api: Api){
        if cell.count.wrapping_sub(api.grid.active) == 1 {
            return;
        }
        cell.update(api);
    
    }
}


