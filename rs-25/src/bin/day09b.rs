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
    //   - (1) find the tiles that are marked as x, programmatically
    //   - (2) check for rectangles that have one of these tiles as corner
    //   - (3) implement simple collision checking - check for intersecion of lines
    // (this is very un-generic, yes)

    // (1)
    let mut hot_tile1 = None;
    let mut hot_tile2 = None;
    for i in 0..red_tiles.len() {
        let t1 = red_tiles[i];
        let t2_index = if i == red_tiles.len() - 1 { 0 } else { i + 1 };
        let t2 = red_tiles[t2_index];

        if (t1.0 - t2.0).abs() > 10000 {
            match hot_tile1 {
                Some(_) => {
                    hot_tile2 = Some(t1);
                    break;
                }
                None => {
                    hot_tile1 = Some(t2);
                }
            }
        }
    }

    let t1 = hot_tile1.unwrap();
    let t2 = hot_tile2.unwrap();

    // (2)
    let mut biggest_area = 0;
    for tile1 in vec![t1, t2] {
        'loopy: for i2 in 0..red_tiles.len() {
            let tile2 = red_tiles[i2];
            let area = ((tile1.0 - tile2.0).abs() + 1) * ((tile1.1 - tile2.1).abs() + 1);

            // (3)
            let x1 = tile1.0.min(tile2.0);
            let x2 = tile1.0.max(tile2.0);
            let y1 = tile1.1.min(tile2.1);
            let y2 = tile1.1.max(tile2.1);

            for t in &red_tiles {
                if t.0 > x1 && t.0 < x2 && t.1 > y1 && t.1 < y2 {
                    // this check has one fault: it does not properly take the inner
                    // part of the circle into account. So we add another cheap check
                    // afterwards
                    continue 'loopy;
                }
            }
            // cheap indeed, but works
            let other_y = if tile1 == t1 { t2.1 } else { t1.1 };
            if other_y > y1 && other_y < y2 {
                continue 'loopy;
            }

            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    println!("{biggest_area}");
}
