use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day08.txt").unwrap();

    let jboxes: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    // calculate distances
    let mut distances = Vec::with_capacity(jboxes.len() * jboxes.len());
    for i1 in 0..jboxes.len() {
        for i2 in (i1 + 2)..jboxes.len() {
            if i1 == i2 {
                continue;
            }
            let dist = {
                let dx = jboxes[i2][0] - jboxes[i1][0];
                let dy = jboxes[i2][1] - jboxes[i1][1];
                let dz = jboxes[i2][2] - jboxes[i1][2];
                let d = dx * dx + dy * dy + dz * dz;
                (d as f64).sqrt()
            };

            distances.push((i1, i2, dist));
        }
    }
    distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    // connect jboxes - create circuits
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for distance in distances {
        let i1 = distance.0;
        let i2 = distance.1;

        let mut jbox1_circuit = None;
        let mut jbox2_circuit = None;
        for (i, circuit) in circuits.iter().enumerate() {
            // check if there is already a circuit with one of the jboxes connected
            if circuit.iter().any(|ci| *ci == i1) {
                jbox1_circuit = Some(i);
            }
            if circuit.iter().any(|ci| *ci == i2) {
                jbox2_circuit = Some(i);
            }
            if jbox1_circuit.is_some() && jbox2_circuit.is_some() {
                break;
            }
        }
        match (jbox1_circuit, jbox2_circuit) {
            (Some(c1), Some(c2)) => {
                // both jboxes are already connected to a circuit
                if c1 != c2 {
                    // combine both circuits to one (move all jboxes to circuit c1, remove c2)
                    while let Some(c) = circuits[c2].pop() {
                        circuits[c1].push(c);
                    }
                    circuits.remove(c2);
                }
            }
            (Some(c1), None) => {
                // only jbox1 connected to circuit -> add jbox2 to circuit
                circuits[c1].push(i2);
            }
            (None, Some(c2)) => {
                // only jbox2 connected to circuit -> add jbox1 to circuit
                circuits[c2].push(i1);
            }
            (None, None) => {
                // no jbox connected yet, create new circuit
                circuits.push(vec![i1, i2]);
            }
        }

        // check if all jboxes are connected into 1 circuit
        if circuits[0].len() == jboxes.len() {
            println!("all connected! Amount of circuits: {}", circuits.len());
            println!("{}", jboxes[i1][0] * jboxes[i2][0]);
            break;
        }
    }
}
