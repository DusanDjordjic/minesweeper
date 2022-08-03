pub struct Position {
    x: usize, 
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn print(&self) {
        println!("X: {}, Y: {}", self.x, self.y);
    }
}
