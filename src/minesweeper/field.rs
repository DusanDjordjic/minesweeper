use super::position::Position;

pub struct Field {
    pub position: Position,
    pub is_mine: bool,
    pub is_flag: bool,
    pub revealed: bool,
    pub near_mines: usize,
}

impl Field {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position: Position::new(x, y),
            is_mine: false,
            is_flag: false,
            revealed: false,
            near_mines: 0,
        }
    }

   
    pub fn print_position(&self) {
        self.position.print();
    }

    pub fn print(&self) {
        if self.is_mine {
            print!("*");
        } else if self.is_flag{
            print!(">");
        } else if self.revealed{
            print!("{}", self.near_mines);
        }else {
            print!("#");
        }
    } 
}
