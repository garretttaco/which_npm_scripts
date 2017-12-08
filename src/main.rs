#[macro_use]
extern crate json;

use std::env;
use std::io;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pathname = Dir::get_pathname(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    let filename = String::from("package.json");
    let full_path = Dir::resolve_pathname(pathname, filename).unwrap();
    let path = Path::new(&full_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    let mut package_json = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
            print!("{} contains:\n{}\n", display, s);
           s
        },
    };

    let parsed_json = &json::parse(&package_json).unwrap()["scripts"];
    println!("{:?}", parsed_json.dump());
}


struct Dir {}

impl Dir {
    fn get_pathname(args: &[String]) -> Result<String, &'static str> {
        if args.len() < 2 {
            return Err("You need to supply the path to the directory containing your package.json as the first command line argument")
        }
        let pathname = args[1].clone();
        Ok(pathname)
    }

    fn resolve_pathname(string_one: String, string_two: String) -> Result<String, &'static str> {
        let full_path = format!("{}{}", string_one, string_two);
        Ok(full_path)
    }
}
