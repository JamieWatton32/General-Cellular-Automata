use std::thread::panicking;

use crate::{grid::Grid, logic::{Cell, IS_EMPTY}, materials::Material, PARTICLE_SIZE};
#[derive(Debug)]
pub struct Api<'a> {
    pub x: i32,
    pub y: i32,
    pub grid: &'a mut Grid,
}

impl <'a>Api<'a> {
    pub fn get_neighbour(&mut self,dx: i32, dy: i32) -> Cell {
		if dx > 2*PARTICLE_SIZE || dy > 2*PARTICLE_SIZE {
            panic!("Out of cell neighbour range!") 
		}
		let (neighbour_x,neighbour_y) = (self.x + dx, self.y + dy);

		if neighbour_x > self.grid.width-2*PARTICLE_SIZE || neighbour_y > self.grid.height-2*PARTICLE_SIZE 
		|| neighbour_x < PARTICLE_SIZE || neighbour_y < PARTICLE_SIZE{
            return Cell {
                material: Material::Wall,
                count: self.grid.active,
            };
		}
	

		self.grid.get_cell_state(neighbour_x, neighbour_y)
	}

	pub fn set_cell(&mut self, dx:i32,dy:i32,cell:Cell){
		if dx > 2*PARTICLE_SIZE || dy > 2*PARTICLE_SIZE  {
            panic!("Out of cell neighbour range!") 
		}
		let (neighbour_x,neighbour_y) = (self.x + dx, self.y + dy);

		if neighbour_x > self.grid.width - PARTICLE_SIZE  || neighbour_y >= self.grid.height - PARTICLE_SIZE
		|| neighbour_x < PARTICLE_SIZE || neighbour_y < PARTICLE_SIZE{
			return
		}
	
        
		let idx = self.grid.get_current_index(neighbour_x,neighbour_y);
        self.grid.cells[idx] = cell;
        self.grid.cells[idx].count= self.grid.active.wrapping_add(1);
		
	}
}
