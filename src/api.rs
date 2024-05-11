use crate::{grid::Grid, logic::{Cell, IS_EMPTY}};
#[derive( Debug)]
pub struct Api<'a> {
    pub x: i32,
    pub y: i32,
    pub grid: &'a mut Grid,
}

impl <'a>Api<'a> {
    pub fn get_neighbour(&mut self,dx: i32, dy: i32) -> Cell {
		if dx > 2 || dy > 2 {
            panic!("Out of cell neighbour range!") 
		}
		let (neighbour_x,neighbour_y) = (self.x + dx, self.y + dy);
		if neighbour_x > self.grid.rows -1 || neighbour_y > self.grid.cols - 1
		|| neighbour_x < 1 || neighbour_y <1{
			return IS_EMPTY;
		}

		self.grid.cell_state(neighbour_x, neighbour_y)
	}

	pub fn set_cell(&mut self, dx:i32,dy:i32,cell:Cell){
		if dx > 2 || dy > 2  {
            panic!("Out of cell neighbour range!") 
		}
		let (neighbour_x,neighbour_y) = (self.x + dx, self.y + dy);
		if neighbour_x > self.grid.rows -1 || neighbour_y > self.grid.cols - 1
		|| neighbour_x < 1 || neighbour_y <1{
			return
		}
		self.grid.cells[neighbour_x as usize][neighbour_y as usize] = cell;
	}
}
