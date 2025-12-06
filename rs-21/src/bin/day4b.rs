struct BingoBoard {
    fields: [[Position; 5]; 5],
    finished: bool,
}

#[derive(Copy, Clone)]
struct Position {
    drawn: bool,
    value: u64,
}

fn main() {
    let file_content =
        std::fs::read_to_string("../input/21/day4.txt").expect("read input file");

    let mut input_lines = file_content.lines();

    let drawn_numbers: Vec<u64> = input_lines
        .next()
        .expect("reading input")
        .split(',')
        .map(|number| number.parse::<u64>().expect("parsing input"))
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut y = 0;
    input_lines.for_each(|line| {
        if line.is_empty() {
            boards.push(BingoBoard {
                fields: create_empty_board(),
                finished: false,
            });
            y = 0;
            return;
        }
        line.trim()
            .split_ascii_whitespace()
            .map(|value| value.parse::<u64>().expect("parsing input"))
            .enumerate()
            .for_each(|(i, number)| {
                let bingo_board = boards.last_mut().expect("building the boards");
                bingo_board.fields[y][i].value = number;
            });
        y += 1;
    });

    println!("parsed {} boards", boards.len());
    boards.iter().for_each(|b| print_board(b));

    let mut finished_boards = 0;
    let boards_count = boards.len();
    'main_loop: for number in drawn_numbers.iter() {
        //update boards
        for board in &mut boards {
            for y in 0..board.fields.len() {
                let row = board.fields[y];
                for x in 0..row.len() {
                    if board.fields[y][x].value == *number {
                        board.fields[y][x].drawn = true;
                    }
                }
            }
        }
        let mut winner = check_winners(&boards);
        while winner.is_some() {
            match winner {
                Some(index) => {
                    let mut board = boards.get_mut(index).expect("accessing boards");
                    board.finished = true;
                    finished_boards += 1;
                    if finished_boards == boards_count {
                        print_board(&board);
                        let board_score = calculate_board_score(&board);
                        println!(
                            "board score: {} * {} = {}",
                            board_score,
                            number,
                            board_score * number
                        );
                        break 'main_loop;
                    }
                }
                None => (),
            }
            winner = check_winners(&mut boards);
        }
    }
}

fn check_winners(boards: &Vec<BingoBoard>) -> Option<usize> {
    for (i, board) in boards.iter().enumerate() {
        if board.finished {
            continue;
        }
        for y in 0..board.fields.len() {
            let row = board.fields[y];
            let mut row_drawn = true;
            let mut column_drawn = true;
            for x in 0..row.len() {
                row_drawn &= board.fields[y][x].drawn;
                column_drawn &= board.fields[x][y].drawn;
            }
            if row_drawn || column_drawn {
                return Option::Some(i);
            }
        }
    }
    Option::None
}

fn calculate_board_score(board: &BingoBoard) -> u64 {
    let mut sum_of_undrawn = 0;
    for y in 0..board.fields.len() {
        let row = board.fields[y];
        for x in 0..row.len() {
            if !board.fields[y][x].drawn {
                sum_of_undrawn += board.fields[y][x].value;
            }
        }
    }
    sum_of_undrawn
}

fn print_board(board: &BingoBoard) {
    println!();
    for y_arr in board.fields {
        for x in y_arr {
            if x.drawn {
                print!(" ({}) ", x.value);
            } else {
                print!("  {}  ", x.value);
            }
        }
        println!();
    }
}

fn create_empty_board() -> [[Position; 5]; 5] {
    [[Position {
        drawn: false,
        value: 0,
    }; 5]; 5]
}
