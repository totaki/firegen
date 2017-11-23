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

}