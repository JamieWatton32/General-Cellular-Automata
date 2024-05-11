use crate::logic::Cell;
use crate::materials::Material;

#[derive(Clone, Debug, PartialEq)]

pub struct Grid {
    pub rows: i32,
    pub cols: i32,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {

    pub fn new(rows: i32, cols: i32) -> Self {
        let mut blank_grid = vec![];
        for i in 0..rows{
            let mut row = vec![];
            for j in 0..cols{   
                row.push(Cell::new(Material::Empty,i,j))
            }
            blank_grid.push(row);
        }
        Grid { 
            rows, 
            cols, 
            cells:blank_grid 
        } 
    }

    pub fn update_grid(&mut self, x: usize, y: usize){
        self.cells[x][y].update_cells(self)
    }

    pub fn get_state(&self,dx: usize,dy: usize,cell: Cell)-> Cell{
        if dx > 1 || dy > 1{
            panic!("Out of cell neighbour range!")
        }
        println!("{:?} cell in function",cell);
        let new_x = self.cells[cell.x as usize][cell.y as usize].x + dx as i32 ;
        println!("newx {new_x}");
        let new_y = self.cells[cell.x as usize][cell.y as usize].y + dy as i32 ;
        println!("newy {new_y}");
        println!("{:?} cell with new x y",self.cells[new_x as usize][new_y as usize]);
        self.cells[new_x as usize][new_y as usize]
    }
}

