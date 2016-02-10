use std::hash::Hash;
use std::fmt::Debug;
use models::city::City;

pub trait BoardingPass : Debug {
    fn boarding_id(&self) -> &str;
    fn city_from(&self) -> &City;
    fn city_to(&self) -> &City;
}

impl<'a, 'b> PartialEq<BoardingPass + 'b> for BoardingPass + 'a {
    fn eq(&self, other: &(BoardingPass + 'b)) -> bool {
        self.boarding_id() == other.boarding_id()
    }
}
