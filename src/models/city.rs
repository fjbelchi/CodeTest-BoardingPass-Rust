#[derive(PartialEq, Eq,Clone)]
pub struct City {
    name: String
}

impl City {
    pub fn new(name: String) -> City {
        City {name: name}
    }
}
