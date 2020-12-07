use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Bag {
    color: String,
    quantity: i64,
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.color.eq(&other.color)
    }
}
impl Eq for Bag {}

fn main() {
    let filename = "../input/20/day7.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let regex = Regex::new(r"([0-9]*) ?([a-z ]+)").expect("valid regex");

    let my_bag = Bag {
        color: String::from("shiny gold"),
        quantity: 1,
    };
    let mut bag_map = HashMap::new();

    for line in file.lines() {
        let formatted = line.replace("bags", "").replace("bag", "").replace('.', "");
        let bags: Vec<&str> = formatted.split("contain").collect();

        let outer_bag = map_to_bag(bags.get(0).expect("valid index").trim(), &regex);
        let inner_bags = match bags.get(1).expect("valid index").trim() {
            "no other" => vec![],
            raw_bag => raw_bag
                .split(",")
                .map(|s| map_to_bag(s.trim(), &regex))
                .collect(),
        };

        bag_map.insert(outer_bag, inner_bags);
    }

    let bags_that_my_bag_contains = count_containing_bags(&my_bag, &bag_map);

    println!("{}", bags_that_my_bag_contains);
}

fn count_containing_bags(bag: &Bag, bag_map: &HashMap<Bag, Vec<Bag>>) -> i64 {
    let mut count = 0;
    let containing_bags = bag_map.get(bag).expect("valid key");
    for containing_bag in containing_bags {
        let sub_count = count_containing_bags(containing_bag, bag_map);
        count += containing_bag.quantity + containing_bag.quantity * sub_count;
    }
    count
}

fn map_to_bag(input: &str, regex: &Regex) -> Bag {
    let captures = regex.captures(input).expect("successful capture");

    let quantity = match captures.get(1).expect("capture group 1").as_str() {
        "" => 1,
        other => other.parse().expect("valid number"),
    };
    let color = captures.get(2).expect("capture group 1").as_str();

    Bag {
        color: String::from(color),
        quantity: quantity,
    }
}
