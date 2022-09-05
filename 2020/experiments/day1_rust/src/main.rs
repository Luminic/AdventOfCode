use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("../../day1_input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n", display)
    }

    let input: Vec<&str> = file_contents.split('\n').collect();

    let mut parsed_input: Vec<isize> = Vec::with_capacity(file_contents.len());
    for value in input.iter() {
        match value.parse() {
            Ok(as_int) => parsed_input.push(as_int),
            Err(_) => {},
        }
    }
    
    for i in 0..parsed_input.len() {
        for j in i+1..parsed_input.len() {
            if parsed_input[i]+parsed_input[j] == 2020 {
                println!("{} * {} = {}", parsed_input[i], parsed_input[j], parsed_input[i]*parsed_input[j])
            }
        }
    }
}
