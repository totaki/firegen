extern crate yaml_rust;
extern crate tera;
use handlers::BaseHandler;
use self::yaml_rust::{Yaml};
use self::tera::{Context, Tera};

const TEMPLATE: &str = "react-component.template";

pub struct ReactStateless;

impl ReactStateless  {

}

impl BaseHandler for ReactStateless {

    fn is_current(&self, s: &str) -> bool {
        ReactStateless::compare_strings(s, TEMPLATE)
    }
}