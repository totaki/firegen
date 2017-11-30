extern crate clap;
use self::clap::{Arg, App};


pub struct RunArgs {
    pub input: String,
    pub output: String,
    pub path: String
}

pub fn get_args() -> RunArgs {
    let matches = App::new("Snowgen")
        .version("0.1.0")
        .author("Sergey Emelyanov <karagabul@gmail.com>")
        .about("This application help create Python or ReactJS files from templates")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("Sets the input file to use, format .yml")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Sets the output path where save result")
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Sets the path where stored templates")
            .takes_value(true))
        .get_matches();

    let input= matches
        .value_of("input")
        .unwrap_or("./snowgen.yml")
        .to_owned();

    let output= matches
        .value_of("output")
        .unwrap_or(".")
        .to_owned();

    let path= matches
        .value_of("path")
        .unwrap_or("~/.snowgen")
        .to_owned();

    RunArgs { input, output, path }
}
