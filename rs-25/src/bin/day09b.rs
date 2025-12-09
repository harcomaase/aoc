use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").unwrap();

    let red_tiles: Vec<(i64, i64)> = input
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .map(|s: Vec<i64>| (s[0], s[1]))
        .collect();

    // I visualised the tiles, and it is a big circle that looks roughly like this

    /*
               #     #
          #                #
        #                    #
      #                        #
     #                          #
    #######################x     #
                           #
    #######################x     #
    #                           #
     #                          #
      #                       #
        #                    #
           #              #
              #  #  #  #
        */

    // ... which leads to the conclusion, that one of the points marked as 'x'
    // is probably a corner of the rectangle we are looking for.
    // The plan:
    //   - find the tiles that are marked as x, programmatically
    //   - check for rectangles that have one of these tiles as corner
    //   - implement simple collision checking - check for intersecion of lines

    //TODO: below is part 1 code - update with above notes
    let mut biggest_area = 0;
    for i1 in 0..red_tiles.len() {
        for i2 in (i1 + 1)..red_tiles.len() {
            let area = ((red_tiles[i1].0 - red_tiles[i2].0).abs() + 1)
                * ((red_tiles[i1].1 - red_tiles[i2].1).abs() + 1);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    println!("{biggest_area}");

    let mut i: usize = 0;
    for _ in 0..100000 {
        for _ in 0..100000 {
            i += 1;
        }
    }

    println!("{i}");
}
