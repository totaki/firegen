extern crate tera;
extern crate yaml_rust;
mod handlers;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlLoader};
use tera::{Context, Tera};

fn main() {

    let tera = Tera::new("templates/**/*").expect("Failed to render template");
    let mut f = File::open("firegen.yml").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&contents).expect("Fuck rust");
    let doc = &docs[0];

    let mut ctx = Context::new();
    let name = doc["files"]["name"].as_str().unwrap();
    ctx.add("name", &name);
    let rendered = tera.render("react-component.template", &ctx).expect("Failed to render template");
    println!("{}", rendered);
    let react = handlers::react::ReactStateless::new("Hello");

    let react1 = handlers::react::ReactStateless::new("Hello1");
    react1.print_name();
    react.print_name();
}
