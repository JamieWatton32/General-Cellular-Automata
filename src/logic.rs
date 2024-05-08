use crate::materials::Material;



#[derive(Clone, Debug, Copy)]
pub struct Cell{
    pub material:Material,
    pub red:u8,
    pub green:u8,
    pub blue:u8,
} 


impl Cell{
    fn new(material:Material) -> Cell{
        Cell{
            material,
            red:0,
            green:0,
            blue:0,
        }
    }

    pub fn update(&self, sim: Simulator) {
        self.material.update(*self, sim)
    }
}

pub static IS_EMPTY: Cell = Cell{
    material: Material::Empty,
    red:0,
    green:0,
    blue:0,
};

#[derive(Debug)]
pub struct Simulator<'a>{
    pub(crate) x: i32,
    pub(crate) y:i32,
    pub(crate) grid: &'a mut Grid
}



impl <'a>Simulator<'a>{
    pub fn fetch_cell(&mut self, dx:i32, dy:i32) -> Cell{
        if dx < 0 || dy <0 {
            panic!("Cell out of bounds!")
        }

        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if new_x < 0 || new_x > self.grid.width - 1 || new_y < 0 || new_y > self.grid.height - 1 {
            return Cell {
                material: Material::Empty,
                red: 0,
                green: 0,
                blue: 0,
            };
        }
        self.grid.get_cell(new_x, new_y)
    }

    pub fn set_cell(&mut self, dx:i32, dy:i32, cell:Cell){
        if dx < 0 || dy <0 {
            panic!("Cell out of bounds!")
        }
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        if new_x < 0 || new_x > self.grid.width - 1 || new_y < 0 || new_y > self.grid.height - 1 {
            return;
        }
        
        let new_idx = self.grid.get_index(new_x, new_y);
        self.grid.cells[new_idx] = cell;

    }

}
#[derive(Debug)]
pub struct Grid{
    width: i32,
    height:i32,
    cells: Vec<Cell>,
}

//public methods
impl Grid {
    pub fn new(width: i32, height: i32) -> Grid{
        let cells = (0..width * height).map(|_i| IS_EMPTY).collect();
        Grid{
            width:width,
            height:height,
            cells:cells
        }
    }
    pub fn width(&self) -> i32{
        self.width
    }
    pub fn height(&self) -> i32{
        self.height
    }
    pub fn get_index(&self, x:i32,y:i32) -> usize{
        let idx = x * self.height + y;
        idx as usize
    }
    pub fn get_cell(&self,x:i32, y:i32) -> Cell{
        let idx = self.get_index(x, y);
        self.cells[idx]
    }
}



