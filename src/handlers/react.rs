pub struct ReactStateless<'a> {
    name: &'a str
}

impl<'a>  ReactStateless<'a>  {

    pub fn new(name: &str) -> ReactStateless {
        ReactStateless { name }
    }

    pub fn print_name(&self) {
        println!("{}", self.name);
    }
}