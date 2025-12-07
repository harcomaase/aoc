use std::fs;

struct Beam {
    x: usize,
    y: usize,
    count: usize,
}

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").unwrap();

    let manifold: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let startx = manifold[0].iter().position(|c| *c == 'S').unwrap();

    let mut beams = Vec::new();
    beams.push(Beam {
        x: startx,
        y: 1,
        count: 1,
    });
    let mut timelines = 0;
    loop {
        let mut to_be_removed_indices = Vec::new();
        let mut new_beams = Vec::new();
        for (i, beam) in beams.iter_mut().enumerate() {
            // remove beams that exceed the manifold
            if beam.y >= manifold.len() - 1 {
                to_be_removed_indices.push(i);
                timelines += beam.count;
                continue;
            }

            if manifold[beam.y + 1][beam.x] == '^' {
                // split - remove current beam, create 2 new ones
                to_be_removed_indices.push(i);
                new_beams.push(Beam {
                    x: beam.x - 1,
                    y: beam.y + 1,
                    count: beam.count,
                });
                new_beams.push(Beam {
                    x: beam.x + 1,
                    y: beam.y + 1,
                    count: beam.count,
                });
                continue;
            }
            beam.y += 1;
        }

        while let Some(i) = to_be_removed_indices.pop() {
            beams.remove(i);
        }

        while let Some(beam) = new_beams.pop() {
            if let Some(existing_beam) = beams.iter_mut().find(|b| **b == beam) {
                existing_beam.count += beam.count;
            } else {
                beams.push(beam);
            }
        }

        if beams.is_empty() {
            break;
        }
    }

    println!("{timelines}");
}
