fn main() {
    let file_content = std::fs::read_to_string("../input/21/day9.txt").expect("read input file");

    let input: Vec<Vec<u32>> = file_content
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let width = input[0].len();
    let height = input.len();
    let mut low_point_values: Vec<u32> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let mut low_point = true;
            let current = input[y][x];
            if y > 0 {
                low_point &= input[y - 1][x] > current;
            }
            if y + 1 < height {
                low_point &= input[y + 1][x] > current;
            }
            if x > 0 {
                low_point &= input[y][x - 1] > current;
            }
            if x + 1 < width {
                low_point &= input[y][x + 1] > current;
            }
            if low_point {
                low_point_values.push(current);
            }
        }
    }

    println!("low_points: {}", low_point_values.len());
    println!(
        "risk: {}",
        low_point_values.iter().map(|p| p + 1).sum::<u32>()
    );
}
