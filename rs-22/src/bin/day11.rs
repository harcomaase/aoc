use std::fs;

struct Monkey {
    items: Vec<i64>,
    operation: char,
    operation_value: Option<i64>,
    test_divisible_by: i64,
    test_true_target: usize,
    test_false_target: usize,
    inspections: usize,
}

fn main() {
    let input = fs::read_to_string("../input/22/day11.txt").unwrap();

    // parse input
    let mut monkeys = Vec::new();
    for paragraph in input.split("\n\n") {
        let mut lines = paragraph.lines().skip(1);
        let starting_items: Vec<i64> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let (operation, operation_value) = {
            let op = lines.next().unwrap();
            let sign = match &op[23..24] {
                "+" => '+',
                "*" => '*',
                _ => panic!("unexpected operation: {}", op),
            };
            let value = match &op[25..].parse::<i64>() {
                Ok(value) => Some(*value),
                Err(_) => None,
            };
            (sign, value)
        };
        let test_divisible_by = lines.next().unwrap()[21..].parse::<i64>().unwrap();
        let test_true_target = lines.next().unwrap()[29..].parse().unwrap();
        let test_false_target = lines.next().unwrap()[30..].parse().unwrap();

        monkeys.push(Monkey {
            items: starting_items,
            operation,
            operation_value,
            test_divisible_by,
            test_true_target,
            test_false_target,
            inspections: 0,
        });
    }

    // simulate rounds
    for _i in 0..20 {
        for i in 0..monkeys.len() {
            let (mut test_true_items, mut test_false_items) = process_monkey(&mut monkeys[i]);
            let test_true_target = monkeys[i].test_true_target;
            let test_false_target = monkeys[i].test_false_target;
            monkeys[test_true_target].items.append(&mut test_true_items);
            monkeys[test_false_target]
                .items
                .append(&mut test_false_items);
        }

        // println!("After round {}:", _i + 1);
        // monkeys.iter().for_each(|m| println!("{:?}", m.items));
        // println!();
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("monkey business: {}", monkey_business)
}

fn process_monkey(monkey: &mut Monkey) -> (Vec<i64>, Vec<i64>) {
    let mut test_true_items = Vec::new();
    let mut test_false_items = Vec::new();
    while !monkey.items.is_empty() {
        let mut item = monkey.items.remove(0);

        match monkey.operation {
            '+' => match monkey.operation_value {
                Some(value) => item += value,
                None => item += item,
            },
            '*' => match monkey.operation_value {
                Some(value) => item *= value,
                None => item *= item,
            },
            _ => panic!(),
        }

        item = (item as f64 / 3.0).floor() as i64;

        let divisible = item % monkey.test_divisible_by;
        if divisible == 0 {
            test_true_items.push(item);
        } else {
            test_false_items.push(item);
        }
        monkey.inspections += 1;
    }
    (test_true_items, test_false_items)
}
