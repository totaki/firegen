extern crate yaml_rust;
extern crate tera;
use self::yaml_rust::{Yaml};
use self::tera::{Context};

pub const TEMPLATE: &str = "react-component.template";


pub fn fill_context(y: &Yaml, c: &mut Context) {
    let name = y["files"]["properties"]["name"].as_str();
    c.add("name", &name)
}
