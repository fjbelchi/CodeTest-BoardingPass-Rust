// use models::city::City;
pub mod models;
pub mod sort;
use models::city::City;
use sort::boarding_pass_generator::BoardingPassGenerator;
use models::boarding_pass_train::BoardingPassTrain;
use models::boarding_pass::BoardingPass;

fn main() {
    let generator = BoardingPassGenerator::new();
    let madrid = City::new("Madrid".to_string());
    let barna = City::new("Barna".to_string());

    let madrid_barna = BoardingPassTrain::new("1", &madrid, &barna);

    for pass in generator.generate() {
        println!("{:?}", pass);
        let equal = *pass == madrid_barna;
        println!("equal {}", equal);
    }
}
