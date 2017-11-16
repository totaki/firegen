extern crate tera;
extern crate yaml_rust;
extern crate clap;
mod handlers;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlLoader};
use tera::{Context, Tera};
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Firegen")
                          .version("0.1.0")
                          .author("Sergey Emelyanov <karagabul@gmail.com>")
                          .about("This application help create Python or ReactJS files from templates")
                          .arg(Arg::with_name("input")
                               .help("Sets the input file to use, format .yml")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("output")
                               .help("Sets the output path where save result")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("path")
                               .help("Sets the path where stored templates")
                               .required(true)
                               .index(3))
                          .get_matches();

//    let tera = Tera::new("templates/**/*").expect("Failed to render template");
//    let mut f = File::open("firegen.yml").expect("file not found");
//    let mut contents = String::new();
//    f.read_to_string(&mut contents)
//        .expect("something went wrong reading the file");
//
//    let docs = YamlLoader::load_from_str(&contents).expect("Fuck rust");
//    let doc = &docs[0];
//
//    let mut ctx = Context::new();
//    let name = doc["files"]["name"].as_str().unwrap();
//    ctx.add("name", &name);
//    let rendered = tera.render("react-component.template", &ctx).expect("Failed to render template");
//    println!("{}", rendered);
//    let react = handlers::react::ReactStateless::new("Hello");
//
//    let react1 = handlers::react::ReactStateless::new("Hello1");
//    react1.print_name();
//    react.print_name();
}
