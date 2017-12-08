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
    println!("{:?}", args);
    let pathname = Dir::get_pathname(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("{:?}", pathname);
    let filename = String::from("package.json");
    let full_path = Dir::resolve_pathname(pathname, filename).unwrap();
    // Create a path to the desired file
    let path = Path::new(&full_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
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
    // `file` goes out of scope, and the "hello.txt" file gets closed
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
