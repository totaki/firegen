extern crate yaml_rust;
extern crate tera;

use self::yaml_rust::{Yaml};
use self::tera::{Context, Tera};

pub static TEMPLATE: &str = "react-component.template";

pub struct ReactStateless<'a> {
    name: &'a str,
}

impl<'a>  ReactStateless<'a>  {

    pub fn new(name: &str) -> ReactStateless {
        ReactStateless { name }
    }

    pub fn print_name(&self) {
        println!("{}", self.name);
    }

    pub fn render(&self, t: Tera, c: Context, tn: &str) {
//        c.add("name", &self.name);
//        let rendered = t.render(tn, &c)
//            .expect("Failed to render template");
//        println!("{}", rendered);
    }
}