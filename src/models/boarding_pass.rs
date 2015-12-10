use models::city::City;

pub trait BoardingPass {
    fn boarding_id(&self) -> &str;
    fn city_from(&self) -> &City;
    fn city_to(&self) -> &City;
}
