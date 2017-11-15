pub struct ReactStateless {
    name: i32
}

impl ReactStateless {

    pub fn new(name: i32) -> Self {
        ReactStateless { name }
    }

    pub fn print_name(&self) {
        println!("{}", self.name);
    }
}