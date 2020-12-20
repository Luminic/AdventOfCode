use std::fs;
use std::path::Path;

use std::collections::HashMap;

const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const SHINY_GOLD: &str = "shiny gold";

trait HelpfulFunctionsForStringSlices<T> {
    fn remove_last_word(self) -> T;
}

impl<'a> HelpfulFunctionsForStringSlices<&'a str> for &'a str {
    fn remove_last_word(self) -> &'a str {
        self.rsplitn(2, ' ').nth(1).unwrap_or("")
    }
}

fn can_hold_gold<'a>(
    bag: &'a str,
    rules: &HashMap<&'a str, HashMap<&'a str, u32>>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    match cache.get(bag) {
        Some(v) => return *v,
        None => {}
    }

    for needed_bag in rules.get(bag).expect("No rule found for bag").keys() {
        if *needed_bag == SHINY_GOLD || can_hold_gold(needed_bag, rules, cache) {
            cache.insert(bag, true);
            return true;
        }
    }
    cache.insert(bag, false);
    false
}

fn nr_bags_inside<'a>(
    bag: &'a str,
    rules: &HashMap<&'a str, HashMap<&'a str, u32>>,
    cache: &mut HashMap<&'a str, u32>,
) -> u32 {
    match cache.get(bag) {
        Some(v) => return *v,
        None => {}
    }

    let mut bags_inside_bag = 0;
    let rule = rules.get(bag).expect("No rule found for bag");
    for needed_bag in rule.keys() {
        bags_inside_bag +=
            (nr_bags_inside(needed_bag, rules, cache) + 1) * rule.get(needed_bag).unwrap();
    }

    cache.insert(bag, bags_inside_bag);
    bags_inside_bag
}

fn main() {
    // Load input into `file_contents`
    let path = Path::new("../../day7_input.txt");
    let file_contents = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
    };

    // Parse input into a hashmap
    // String slices should all "point" to either data in `file_contents` or be static
    let mut rules: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    for rule in file_contents.as_str().lines() {
        if rule.is_empty() {
            continue;
        }

        let mut rule_parts = rule.split("contain");
        let bag = rule_parts
            .next()
            .expect("Couldn't find bag")
            .trim_matches(|c| !ASCII_LOWERCASE.contains(c))
            .remove_last_word();

        let contents = rule_parts
            .next()
            .expect("Couldn't find bag requirements")
            .split(',')
            .map(|s| s.trim_matches(|c| !"0123456789".contains(c) && !ASCII_LOWERCASE.contains(c)))
            .take_while(|v| *v != "no other bags")
            .map(|s| {
                let mut split = s.splitn(2, ' ');
                let count = split
                    .next()
                    .expect("Couldn't find count in bag requirements")
                    .parse::<u32>()
                    .expect("Couldn't parse count in bag requirements");
                let bag = split
                    .next()
                    .expect("Couldn't find bag in bag requirements")
                    .remove_last_word();
                (bag, count)
            })
            .collect::<HashMap<_, _>>();

        rules.insert(bag, contents);
    }

    // println!("{:?}", rules);

    // Part one
    let mut cache = HashMap::new();
    let mut nr_bags_cols_that_can_hold_shiny_gold = 0u32;
    for bag in rules.keys() {
        if can_hold_gold(bag, &rules, &mut cache) {
            nr_bags_cols_that_can_hold_shiny_gold += 1;
        }
    }

    println!(
        "{} bag colors can hold 1+ shiny gold bags.",
        nr_bags_cols_that_can_hold_shiny_gold
    );

    // Part two
    let mut cache = HashMap::new();
    let nr_bags_inside_shiny_gold = nr_bags_inside(SHINY_GOLD, &rules, &mut cache);

    println!(
        "There must be {} bags inside of a shiny gold bag.",
        nr_bags_inside_shiny_gold
    )
}
