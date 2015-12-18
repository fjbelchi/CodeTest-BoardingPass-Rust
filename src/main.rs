// use models::city::City;
pub mod models;
pub mod sort;
use std::fmt::Debug;
use sort::boarding_pass_generator::BoardingPassGenerator;

fn main() {
    let generator = BoardingPassGenerator::new();
    
    for pass in generator.generate() {
        println!("{:?}", pass);
    }
}
