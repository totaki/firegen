extern crate tera;
extern crate yaml_rust;
mod handlers;
mod utils;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlLoader};
use tera::{Context, Tera};

use std::path::Path;


fn main() {

    let run_args = utils::cli_args::get_args();
    let mut can_run = true;

    if !Path::new(run_args.input.as_str()).exists() {
        println!("Input file not exists: {}", run_args.input);
        can_run = false;
    }

    if !Path::new(run_args.output.as_str()).exists() {
        println!("Output path not exists: {}", run_args.output);
        can_run = false;
    }

    if !Path::new(run_args.path.as_str()).exists() {
        println!("Template path not exists: {}", run_args.path);
        can_run = false;
    }


    if can_run {
        let mut input_file = File::open(run_args.input).expect("File not exists");
        let mut input_string = String::new();
        input_file.read_to_string(&mut input_string)
            .expect("something went wrong reading the file");

        let docs = YamlLoader::load_from_str(&input_string)
            .expect("Cannot parse input .yml");
        let doc = &docs[0];
        let react = handlers::react::ReactStateless::new("Hello");
        react.render(&doc);

    }


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
