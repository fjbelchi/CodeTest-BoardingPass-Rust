use models::city::City;
use models::boarding_pass::BoardingPass;

pub struct BoardingPassTrain<'a> {
    boarding_id: &'static str,
    city_from: &'a City,
    city_to: &'a City
}


impl<'a> BoardingPass for BoardingPassTrain<'a>  {

    fn boarding_id(&self) -> &str {
        &self.boarding_id
    }

    fn city_from(&self) -> &City {
        &self.city_from
    }

    fn city_to(&self) -> &City {
        &self.city_to
    }
}

impl<'a> BoardingPassTrain<'a>  {
    pub fn new(boarding_id: &'static str, city_from: &'a City, city_to: &'a City) -> BoardingPassTrain<'a> {
        BoardingPassTrain {
            boarding_id: boarding_id,
            city_from: city_from,
            city_to: city_to
        }
    }
}

#[test]
fn test_constructor() {
    let madrid = City::new("Madrid".to_string());
    let barna = City::new("Barna".to_string());
    let pass = BoardingPassTrain::new("123", &madrid, &barna);
    assert!(pass.boarding_id() == "123");
    assert!(pass.city_from() == &madrid);
    assert!(pass.city_to() == &barna);
}
