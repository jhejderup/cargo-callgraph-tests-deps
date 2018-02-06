extern crate food;

use food::Food;

#[derive(Debug)]
pub struct Pasta;

impl Food for Pasta {
    fn eat(&self) {
        println!("eating some damn good pasta!");
    }
}
