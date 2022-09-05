use std::fs;
use std::path::Path;

use std::collections::HashMap;

// Checks if a number (stored in a string) is in a range (inclusive)
fn nr_str_in_range(nr_str: &str, min: i32, max: i32) -> bool {
    match nr_str.parse::<i32>() {
        Ok(yr) => {
            min <= yr && yr <= max
        },
        Err(_) => false,
    }
}

fn height_is_valid(hgt_str: &str) -> bool {
    let chars = hgt_str.as_bytes();
    
    let mut split_location = chars.len();
    for (i, c) in chars.iter().enumerate() {
        if !"0123456789".contains(*c as char) {
            split_location = i;
            break;
        }
    }

    let nr;
    match std::str::from_utf8(&chars[0..split_location]) {
        Ok(val) => nr = val,
        Err(_) => return false,
    }
    let unit;
    match std::str::from_utf8(&chars[split_location..chars.len()]) {
        Ok(val) => unit = val,
        Err(_) => return false,
    }

    return match unit {
        "cm" => nr_str_in_range(nr, 150, 193),
        "in" => nr_str_in_range(nr, 59, 76),
        _ => false
    }
}

fn hair_col_is_valid(hcl_str: &str) -> bool {
    if !(hcl_str.len() == 7) {return false;} // Note: length in bytes is acceptable because the only valid chars are '#', '0'-'9', and 'a'-'f' (false negatives are acceptable)
    for (i, c) in hcl_str.chars().enumerate() {
        if i == 0 {
            if c != '#' {return false;}
        } else {
            if !"0123456789abcdef".contains(c) {return false;}
        }
    }
    true
}

fn main() {
    // Load file
    let path = Path::new("../../day4_input.txt");
    let file_contents = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
    };
    let unparsed_passports = file_contents.split("\n\n");

    // Parse input
    let mut passports: Vec<HashMap<&str, &str>> = Vec::new();

    for passport in unparsed_passports {
        passports.push(HashMap::new());
        let passport_split = passport.split(|c|{c == '\n' || c == ' '});
        for field in passport_split {
            let mut field_split = field.split(':');
            match field_split.next() {
                Some(field_name) => {
                    match field_split.next() {
                        Some(field_value) => {
                            // Insert the field name : value pair into the current passport
                            passports.last_mut().unwrap().insert(field_name, field_value);
                        },
                        None => {},
                    }
                },
                None => {},
            }
        }
    }

    // Map from a required field to a function determining if the value for that field is valid
    let passport_requirements: HashMap<&str, Box<dyn Fn(&str) -> bool>> = vec![
        ("byr", Box::new(
            |v: &str|{nr_str_in_range(v, 1920, 2002)}
        ) as Box<dyn Fn(&str) -> bool>),
        ("iyr", Box::new(
            |v: &str|{nr_str_in_range(v, 2010, 2020)}
        )),
        ("eyr", Box::new(
            |v: &str|{nr_str_in_range(v, 2020, 2030)}
        )),
        ("hgt", Box::new(
            height_is_valid
        )),
        ("hcl", Box::new(
            hair_col_is_valid
        )),
        ("ecl", Box::new(
            |v: &str|{["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v)}
        )),
        ("pid", Box::new(
            |v: &str|{v.len() == 9 && v.chars().fold(true, |acc,elem|{acc && "0123456789".contains(elem)})}
        )),
    ].into_iter().collect();

    // Count invalid passports
    let mut nr_invalid_passports = 0;
    let nr_passports = passports.len(); // Store size before moving passports

    for passport in passports {
        for (req, func) in passport_requirements.iter() {
            match passport.get(req) {
                Some(field_data) => {
                    if func(field_data) {
                        continue;
                    }
                },
                None => {}
            }
            nr_invalid_passports += 1;
            break;
        }
    }
    let nr_valid_passports = nr_passports - nr_invalid_passports;
    println!("{} valid pasports!", nr_valid_passports);
    
}