pub struct ReactStateless {
    name: String
}

impl ReactStateless {

    pub fn new(name: &str) -> Self {
        ReactStateless { name: name.to_string() }
    }

    pub fn print_name(&self) {
        println!("{}", self.name);
    }
}