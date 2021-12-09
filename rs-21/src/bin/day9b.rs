fn main() {
    let file_content = std::fs::read_to_string("../input/21/day9.txt").expect("read input file");

    let input: Vec<Vec<u32>> = file_content
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let width = input[0].len();
    let height = input.len();
    let mut low_point_values: Vec<(usize, usize)> = Vec::new();
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
                low_point_values.push((x, y));
            }
        }
    }

    let mut basin_sizes: Vec<usize> = low_point_values
        .iter()
        .map(|(x, y)| {
            let mut basin_points = Vec::new();
            determine_basin_size(*x, *y, width, height, &input, &mut basin_points);
            basin_points.len()
        })
        .collect();
    basin_sizes.sort_by(|a, b| b.cmp(&a));
    let result: usize = basin_sizes.iter().take(3).product();

    println!("result: {}", result);
}

fn determine_basin_size(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    input: &Vec<Vec<u32>>,
    basin_points: &mut Vec<(usize, usize)>,
) {
    let current = input[y][x];
    if current == 9 {
        return;
    }
    if basin_points.contains(&(x, y)) {
        return;
    }
    basin_points.push((x, y));
    if y > 0 && input[y - 1][x] > current {
        determine_basin_size(x, y - 1, width, height, input, basin_points);
    }
    if y + 1 < height && input[y + 1][x] > current {
        determine_basin_size(x, y + 1, width, height, input, basin_points);
    }
    if x > 0 && input[y][x - 1] > current {
        determine_basin_size(x - 1, y, width, height, input, basin_points);
    }
    if x + 1 < width && input[y][x + 1] > current {
        determine_basin_size(x + 1, y, width, height, input, basin_points);
    }
}
