use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("../../day2_input.txt");
    let file_contents = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
    };
    
    let lines = file_contents.split('\n');

    let mut valid_passwords: usize = 0;
    for line in lines {
        if line == "" {break;}
        let mut split = line.split(" ");
        let mut range = split.next().unwrap().split('-');
        
        let min = range.next().unwrap().parse::<usize>().unwrap();
        let max = range.next().unwrap().parse::<usize>().unwrap();
        let letter = split.next().unwrap().chars().next().unwrap();
        let password = split.next().unwrap();

        let occurrences = password.matches(letter).count();
        if min <= occurrences && occurrences <= max {
            valid_passwords += 1;
        }
    }
    println!("{} valid passwords!", valid_passwords);
}
