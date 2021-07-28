struct Person {
    first_name: String,
    last_name: String,
}

impl Person { //外置接口，把函数实现全部放在此接口里
    fn new(a: &str, b: &str) -> Person {
        Person {
            first_name: a.to_string(),
            last_name: b.to_string(),
        }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
}

    //Set last lastname
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_puple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut p = Person::new("Mary", "Doe");
    println!("full_name: {}", p.full_name());

    p.set_last_name("Sheldon");
    println!("full_name: {}", p.full_name());

    println!("Tuple: {:?}", p.to_puple());

}
