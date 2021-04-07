use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn cat(path: &str) -> String {
    match File::open(path) {
        Ok(mut file) => {
            let mut res = String::new();
            match file.read_to_string(&mut res) {
                Ok(_) => res,
                Err(_) => String::from("Error while reading from file"),
            }
        },
        Err(_) => String::from("Failed to open file"),
    }
}

pub fn cat_stdio() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => print!("{}", input),
            Err(_) => {
                println!("{}", "Unable to read from stdin");
                break;
            }
        }
        input.clear()
    }
}

fn main() {
    let mut files = env::args();
    files.next();
    match files.len() {
        0 => cat_stdio(),
        nb => for _ in 0..nb {
            match files.next() {
                Some(value) => print!("{}", cat(value.as_str())),
                None => println!("no file provided"),
            }
        },
    }
}
