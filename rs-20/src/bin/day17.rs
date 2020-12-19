use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    active: bool,
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    let filename = "../input/20/day17.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let mut cubes = init_cubes(&file);
    let mut copy = cubes.clone();

    for round in 0..6 {
        for cube in get_relevant_cubes(&cubes) {
            let adjacent_cubes = get_adjacent_cubes(&cube, &cubes);
            let active_adjacent_cubes = adjacent_cubes
                .into_iter()
                .filter(|cube| cube.active)
                .count();
            if cube.active && (active_adjacent_cubes < 2 || active_adjacent_cubes > 3) {
                let cc = get_cube_at_mut(&mut copy, (cube.x, cube.y, cube.z));
                cc.active = false;
            } else if !cube.active && active_adjacent_cubes == 3 {
                let cc = get_cube_at_mut(&mut copy, (cube.x, cube.y, cube.z));
                cc.active = true;
            }
        }
        cubes.clear();
        for cube in copy.clone() {
            cubes.push(cube);
        }

        println!(
            "active cubes after round {}: {}",
            round,
            cubes.clone().into_iter().filter(|cube| cube.active).count()
        );
    }
}

fn get_relevant_cubes(cubes: &Vec<Cube>) -> Vec<Cube> {
    let mut relevant_cubes = Vec::new();
    for cube in cubes {
        if !cube.active {
            continue;
        }
        if !relevant_cubes.contains(cube) {
            relevant_cubes.push(*cube);
        }
        for c in get_adjacent_cubes(cube, cubes) {
            if !relevant_cubes.contains(&c) {
                relevant_cubes.push(c);
            }
        }
    }
    relevant_cubes
}

fn get_adjacent_cubes(cube: &Cube, cubes: &Vec<Cube>) -> Vec<Cube> {
    let mut adjacent_cubes = Vec::new();
    for x in cube.x - 1..=cube.x + 1 {
        for y in cube.y - 1..=cube.y + 1 {
            for z in cube.z - 1..=cube.z + 1 {
                if cube.x == x && cube.y == y && cube.z == z {
                    continue;
                }
                let cube = get_cube_at(cubes, (x, y, z));
                adjacent_cubes.push(cube);
            }
        }
    }
    adjacent_cubes
}

fn init_cubes(file: &str) -> Vec<Cube> {
    let mut cubes = Vec::new();
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let active = match c {
                '#' => true,
                _ => false,
            };
            cubes.push(Cube {
                active,
                x: x as i64,
                y: y as i64,
                z: 0,
            });
        }
    }
    let dim = 50;
    for x in -dim..=dim {
        for y in -dim..=dim {
            for z in -dim..=dim {
                cubes.push(Cube {
                    active: false,
                    x: x,
                    y: y,
                    z: z,
                });
            }
        }
    }
    cubes
}

fn get_cube_at_mut(cubes: &mut Vec<Cube>, (x, y, z): (i64, i64, i64)) -> &mut Cube {
    for cube in cubes {
        if cube.x == x && cube.y == y && cube.z == z {
            return cube;
        }
    }
    panic!("aaaah");
}

fn get_cube_at(cubes: &Vec<Cube>, (x, y, z): (i64, i64, i64)) -> Cube {
    for cube in cubes {
        if cube.x == x && cube.y == y && cube.z == z {
            return *cube;
        }
    }
    panic!("aaaah");
}
