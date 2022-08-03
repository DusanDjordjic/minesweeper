mod minesweeper;

use crate::minesweeper::Minesweeper;

fn main() {
   let minesweeper = Minesweeper::new(10, 10, 10);
   minesweeper.print_fields();
}
