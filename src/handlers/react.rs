pub struct ReactStateless {
    name: String
}

impl ReactStateless {

    pub fn new<S>(name: S) -> Self
        where S: Into<String> {
        
        ReactStateless { name: name.into() }
    }

    pub fn print_name(&self) {
        println!("{}", self.name);
    }
}