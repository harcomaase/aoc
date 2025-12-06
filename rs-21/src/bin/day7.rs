fn main() {
    let file_content =
        std::fs::read_to_string("../input/21/day7.txt").expect("read input file");

    let crabs: Vec<u32> = file_content
        .split(",")
        .map(|line| line.parse::<u32>().expect("parsing input file"))
        .collect();
    let min = crabs.iter().min().expect("minimum position");
    let max = crabs.iter().max().expect("maximum position");

    let mut total_fuels : Vec<u32> = Vec::new();
    for i in *min..=*max {
        let mut total_fuel = 0;
        for crab in crabs.iter() {
            let fuel = u32::max(i, *crab) - u32::min(i, *crab);
            total_fuel += fuel;
        }
        total_fuels.push(total_fuel);
    }
    let min_fuel = total_fuels.iter().min().expect("minimum fuel calculation");
    println!("min_fuel: {}", min_fuel);
}
