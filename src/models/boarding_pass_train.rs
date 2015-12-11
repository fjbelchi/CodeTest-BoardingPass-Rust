use models::city::City;
use models::boarding_pass::BoardingPass;

pub struct BoardingPassTrain {
    boarding_id: &'static str,
    city_from: City,
    city_to: City,
}


impl BoardingPass for BoardingPassTrain {

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

impl BoardingPassTrain {
    pub fn new(boarding_id: &'static str, city_from: City, city_to: City) -> BoardingPassTrain {
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
