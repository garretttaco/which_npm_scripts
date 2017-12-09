extern crate json;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pathname = get_pathname(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    let filename = String::from("package.json");
    let full_path = resolve_pathname(pathname, filename).unwrap();
    let path = Path::new(&full_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    let package_json = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
           s
        },
    };

    let parsed_json = &json::parse(&package_json).unwrap()["scripts"];
    println!("{:#}", parsed_json);
}


fn get_pathname(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        let pathname = env::current_dir().unwrap();
        let forward_slash = String::from("/");
        return Ok(pathname.display().to_string() + &forward_slash)
    }
    let pathname = args[1].clone();
    Ok(pathname)
}

fn resolve_pathname(string_one: String, string_two: String) -> Result<String, &'static str> {
    let pathname = format!("{}{}", string_one, string_two);
    Ok(pathname)
}
