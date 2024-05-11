use crate::api::Api;
use crate::logic::{Cell, IS_EMPTY};


#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub rows: i32,
    pub cols: i32,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {

    pub fn new(rows: i32, cols: i32) -> Self {
        let mut blank_grid = vec![];
        for _ in 0..rows{
            let mut row = vec![];
            for _ in 0..cols{   
                row.push(IS_EMPTY);
            }
            blank_grid.push(row);
        }
        Grid { 
            rows, 
            cols, 
            cells:blank_grid 
        } 
    }
    pub fn cell_state(&self,x:i32,y:i32)->Cell{
        self.cells[x as usize][y as usize]
    }

    pub fn update_grid(&mut self){
        for x in 0..self.rows-1{
            for y in 0..self.cols-1{
                let cell = self.get_cell(x, y);
                Grid::update_cell(
                    cell,
                    Api{
                        x,
                        y,
                        grid:self
                    }
                );
                }
            }
        }

        fn _rows(self)->i32{
            self.rows
        }
        fn _cols(self)-> i32{
            self.cols
        }

        fn get_cell(&self,x:i32, y:i32) -> Cell{
            self.cells[x as usize][y as usize]
        }
    
        fn update_cell(cell:Cell, api:Api){
            cell.update(api);
        }
    }

 





