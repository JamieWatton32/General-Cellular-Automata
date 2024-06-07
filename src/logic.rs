use crate::{api::Api, user_data::UserData};




//A cell is a object that contains a user_data and a rgb color. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell{
    pub user_data:UserData, // user_data of the cell
    pub count:u8,
} 

pub static IS_EMPTY: Cell = Cell{
    user_data:UserData::Empty,
    count:0,
};


impl Cell{
    pub fn _new(user_data: UserData)-> Cell{
        Cell {
            user_data,
            count:0,
        }
    }

    pub fn update(&self, api:Api){
        self.user_data.update_data(*self, api)
    }

   
    
}


