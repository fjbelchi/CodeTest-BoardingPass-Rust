use models::boarding_pass::BoardingPass;
use models::city::City;
use models::boarding_pass_train::BoardingPassTrain;

struct BoardingPassGenerator;

impl BoardingPassGenerator {
    pub fn generate() -> Vec<Box<BoardingPass>> {
        let mut vector: Vec<Box<BoardingPass>> = Vec::new();
        let madrid = City::new("Madrid".to_string());
        let barna = City::new("Barna".to_string());

        let madrid_barna = BoardingPassTrain::new("1", &madrid, &barna);
        vector.push(Box::new(madrid_barna));

        let barna_madrid = BoardingPassTrain::new("2", &barna, &madrid);
        vector.push(Box::new(barna_madrid));
        
        vector
    }
}
