#![allow(dead_code)]

mod field;
mod position;

use field::Field;
use position::Position;
use rand::{thread_rng, Rng};

pub struct Minesweeper {
   width: usize,
   height: usize,
   fields: Vec<Vec<Field>>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let mut new_fields = Vec::new();

        for x in 0..width {
            let mut new_row = Vec::new();
            for y in 0..height {
                new_row.push(Field::new(x, y))
            }
            new_fields.push(new_row);
        }
        generate_mines(&mut new_fields, mine_count);        
        
        Self {
            width,
            height,
            fields: new_fields,
        }
    }

    pub fn print_fields(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.fields
                    .get(x).unwrap()
                    .get(y).unwrap()
                    .print();
            }
            println!("");
        }
    } 


    pub fn print_fields_positions(&self) {
        for row in self.fields.iter() {
            for field in row.iter() {
                field.print_position();
            }
        }
    }
}

fn generate_mines(fields: &mut Vec<Vec<Field>>, mine_count: usize) {
    let width = fields.len();
    let height = fields.get(0).unwrap().len();
    let mut generated_mine_count = 0;
    let mut rng = thread_rng();
    while generated_mine_count != mine_count {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let mut field = fields.get_mut(x).unwrap()
                            .get_mut(y).unwrap();
        if !field.is_mine {
            field.is_mine = true;
            generated_mine_count += 1;
        }
        
    }
}

fn calculate_mine_counts(fields: &mut Vec<Vec<Field>>) {
   
}
