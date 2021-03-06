use models::boarding_pass::BoardingPass;
use models::city::City;
use models::boarding_pass_train::BoardingPassTrain;
use models::boarding_pass_bus::BoardingPassBus;

pub struct BoardingPassGenerator {
    madrid: City,
    barna: City,
}

impl BoardingPassGenerator {
    pub fn new() -> BoardingPassGenerator {
        BoardingPassGenerator {
            madrid: City::new("Madrid".to_string()),
            barna: City::new("Barna".to_string()),
        }
    }

    pub fn generate<'a>(&'a self) -> Vec<Box<BoardingPass + 'a>> {
        let mut vector: Vec<Box<BoardingPass>> = Vec::new();

        let madrid_barna = BoardingPassTrain::new("1", &self.madrid, &self.barna);
        vector.push(Box::new(madrid_barna));

        let barna_madrid = BoardingPassTrain::new("2", &self.barna, &self.madrid);
        vector.push(Box::new(barna_madrid));

        let madrid_barna_bus = BoardingPassBus::new("3", &self.madrid, &self.barna, "13A", 32);
        vector.push(Box::new(madrid_barna_bus));
        
        vector
    }
}
