extern crate yaml_rust;
extern crate tera;

use self::yaml_rust::{Yaml};
use self::tera::{Context, Tera};


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

    pub fn render(&self, input: &Yaml) {
        let tera = Tera::new("templates/**/*").expect("Failed to render template");
        let mut ctx = Context::new();
        let name = input["files"]["name"].as_str().unwrap();
        ctx.add("name", &name);
        let rendered = tera.render("react-component.template", &ctx)
            .expect("Failed to render template");
        println!("{}", rendered);
    }
}