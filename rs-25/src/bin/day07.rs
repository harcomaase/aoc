use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").unwrap();

    let manifold: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let startx = manifold[0].iter().position(|c| *c == 'S').unwrap();

    let mut beams = Vec::new();
    beams.push((startx, 1));
    let mut split_count = 0;
    loop {
        let mut to_be_removed_indices = Vec::new();
        let mut new_beams = Vec::new();
        for (i, beam) in beams.iter_mut().enumerate() {
            // remove beams that exceed the manifold
            if beam.1 >= manifold.len() - 1 {
                to_be_removed_indices.push(i);
                continue;
            }

            if manifold[beam.1 + 1][beam.0] == '^' {
                // split - remove current beam, create 2 new ones
                to_be_removed_indices.push(i);
                new_beams.push((beam.0 - 1, beam.1 + 1));
                new_beams.push((beam.0 + 1, beam.1 + 1));
                split_count += 1;
                continue;
            }
            beam.1 += 1;
        }

        while let Some(i) = to_be_removed_indices.pop() {
            beams.remove(i);
        }

        while let Some(beam) = new_beams.pop() {
            if !beams.contains(&beam) {
                beams.push(beam);
            }
        }

        if beams.is_empty() {
            break;
        }
    }

    println!("{split_count}");
}
