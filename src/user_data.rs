use rand::Rng;



use crate::{
    api::Api,
    logic::{Cell, IS_EMPTY},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UserData {
    Empty,
    Sand,
    Water,
    Wall,
}

impl UserData {
    pub fn update_data(&self, cell: Cell, api: Api) {
      
        match self {
            UserData::Empty => {},
            UserData::Wall => {},
            UserData::Sand => update_sand(cell, api),
            UserData::Water => update_water(cell, api),
            
        }
    }
}
pub fn update_sand(cell: Cell, mut api: Api) {
    let dx = rand::thread_rng().gen_range(-1..=1);

    let below = api.get_neighbour(0, 1);
    let diagonal = api.get_neighbour(dx, 1);

    if below.user_data == UserData::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, 2, cell);// move down by 2 for speed
    
    } else if diagonal.user_data == UserData::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(dx, 1, cell);
    } else if below.user_data == UserData::Water {
        api.set_cell(0, 0, below);
        api.set_cell(0, 1, cell);
    } else {
        api.set_cell(0, 0, cell);
    }
}

fn update_water(cell: Cell, mut api: Api) {
    let dx = rand::thread_rng().gen_range(-6..=6);

    let below = api.get_neighbour(0, 1);
    let diagonal = api.get_neighbour(dx, 1);

    if below.user_data == UserData::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(0, 2, cell); // move down by 2 for speed
    } else if below.user_data == UserData::Water && diagonal.user_data == UserData::Empty {
        api.set_cell(0, 0, IS_EMPTY);
        api.set_cell(dx, 0, cell);
    } else if below.user_data == UserData::Sand {
        api.set_cell(0, 0, cell);
    } else if diagonal.user_data == UserData::Sand{
        api.set_cell(dx, 0, cell);
    }
    else{
        api.set_cell(0, 0, cell);
    }
}

