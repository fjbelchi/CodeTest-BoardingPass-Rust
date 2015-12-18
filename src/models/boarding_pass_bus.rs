use models::city::City;
use models::boarding_pass::BoardingPass;

#[derive(Hash, PartialEq, Eq, Debug)]
struct BoardingPassBus<'a> {
    boarding_id: &'static str,
    city_from: &'a City,
    city_to: &'a City,
    seat: &'a str,
    bus_number: i32,
}

impl<'a> BoardingPass for BoardingPassBus<'a> {
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

impl<'a> BoardingPassBus<'a> {
    fn new(boarding_id: &'static str,
           city_from: &'a City,
           city_to: &'a City,
           seat: &'a str,
           bus_number: i32)
           -> BoardingPassBus<'a> {
        BoardingPassBus {
            boarding_id: boarding_id,
            city_from: city_from,
            city_to: city_to,
            seat: seat,
            bus_number: bus_number,
        }
    }
}

#[test]
fn test_constructor() {
    let madrid = City::new("Madrid".to_string());
    let barna = City::new("Barna".to_string());
    let pass = BoardingPassBus::new("123", &madrid, &barna, "13A", 32);
    assert!(pass.boarding_id() == "123");
    assert!(pass.city_from() == &madrid);
    assert!(pass.city_to() == &barna);
}
