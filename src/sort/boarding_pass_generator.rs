use models::boarding_pass::BoardingPass;
use models::city::City;
use models::boarding_pass_train::BoardingPassTrain;

struct BoardingPassGenerator;

impl BoardingPassGenerator {
    pub fn generate<'a>(a: &'a City, b: &'a City) -> Vec<Box<BoardingPass + 'a>> {
        let mut vector: Vec<Box<BoardingPass>> = Vec::new();

        let to = BoardingPassTrain::new("1", a, b);
        vector.push(Box::new(to));

        let from = BoardingPassTrain::new("2", b, a);
        vector.push(Box::new(from));
        
        vector
    }
}
