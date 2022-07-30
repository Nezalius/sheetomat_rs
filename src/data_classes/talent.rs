pub struct Talent {
    name: String,
    value: i32,
    cap: u32
}

pub impl Talent {
    pub fn new(name: String, val: i32, cap: u32) -> Self {
        Talent {name: name, value: val, cap: cap}
    }

    pub fn print_talent(&self) {
        println!("Der Talentwert von {} ist: {}", self.name, self.value);
    }
}