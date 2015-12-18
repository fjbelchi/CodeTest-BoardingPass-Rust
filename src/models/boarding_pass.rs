use std::hash::Hash;
use std::fmt::Debug;
use models::city::City;

pub trait BoardingPass : Hash + PartialEq + Debug {
    fn boarding_id(&self) -> &str;
    fn city_from(&self) -> &City;
    fn city_to(&self) -> &City;
}

// impl PartialEq for BoardingPass {
//     fn eq(&self, other: &BoardingPass) -> bool {
//         self.boarding_id() == other.boarding_id()
//     }
// }
