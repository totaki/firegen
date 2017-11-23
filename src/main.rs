extern crate tera;
extern crate yaml_rust;
mod handlers;
mod utils;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{Yaml, YamlLoader};
use std::hash::Hash;
use tera::{Context, Tera};
use std::path::{Path, PathBuf};


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
        let template_name = doc["files"]["template"].as_str();

        let mut ctx = Context::new();
        for (key, value) in doc["files"]["properties"].as_hash().unwrap().iter() {
            ctx.add(key.as_str().unwrap(), &value.as_str().unwrap());
        }
        let tera = Tera::new("templates/**/*").expect("Failed to render template");
        let render_result = tera.render(&template_name.unwrap(), &ctx);
        let mut output = String::new();
        output.push_str(doc["files"]["properties"]["name"].as_str().unwrap());
        output.push_str(".");
        output.push_str(doc["files"]["extension"].as_str().unwrap());
        let path =  Path::new(run_args.output.as_str()).join(output.as_str());
        let mut output_file = File::create(path).expect("Cant create file");
        output_file.write_all(render_result.unwrap().as_bytes());
    }

}
