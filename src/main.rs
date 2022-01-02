use std::collections::HashSet;

/** type alias for the sudoku number table */
type GameGrid = [[usize; 9]; 9];

fn check_row(game: GameGrid, ret: &mut HashSet<usize>, row: usize) -> () {
    for idx in 0..9 {
        if game[row][idx] != 0 {
            ret.insert(game[row][idx]);
        }
    }
}

fn check_column(game: GameGrid, ret: &mut HashSet<usize>, col: usize) -> () {
    for idx in 0..9 {
        if game[idx][col] != 0 {
            ret.insert(game[idx][col]);
        }
    }
}

fn check_block(game: GameGrid, ret: &mut HashSet<usize>, row: usize, col: usize) -> () {
    // map (row, col) to up left of block
    // z.B (7,1) -> (6,0) map to nearest multiple of 3.
    let (r, c) = (row - row % 3, col - col % 3);
    for idx in 0..3 {
        for idy in 0..3 {
            if game[r + idx][c + idy] != 0 {
                ret.insert(game[r + idx][c + idy]);
            }
        }
    }
}

fn generate_possible_numbers(game: GameGrid, row: usize, col: usize) -> HashSet<usize> {
    let mut rets: HashSet<usize> = HashSet::new();
    check_row(game, &mut rets, row);
    check_column(game, &mut rets, col);
    check_block(game, &mut rets, row, col);

    let mut retval = HashSet::new();
    for idx in 1..10 as usize {
        if !rets.contains(&idx) {
            retval.insert(idx);
        }
    }
    retval
}

fn print_board(game: GameGrid) -> () {
    print!("-------------------------\n");
    for row in 0..9 {
        print!("| ");
        for col in 0..9 {
            let num = game[row][col];
            let num_string: String = if num > 0 { num.to_string() } else { '_'.to_string() };
            if col % 3 == 2 {
                print!("{0} | ", num_string);
            } else {
                print!("{0} ", num_string);
            }
        }
        if row % 3 == 2 {
            print!("\n-------------------------");
        }
        print!("\n");
    }
}

fn play(game: &mut GameGrid) -> bool {
    if let Some((row, col)) = next_empty_cell(*game) {
        let set = generate_possible_numbers(*game, row, col);
        //  println!("{:?}, {:?}, {:?}", row, col, set);

        for num in set.iter() {
            game[row][col] = *num;
            //   println!("will try {:?} at {:?},{:?} ", *num, row, col);

            if !play(game) {
                //   println!("back to {:?},{:?} ", row, col);
                game[row][col] = 0;
            } else {
                return true;
            }
        }
        return false;
    } else {
        true
    }
}

fn next_empty_cell(game: GameGrid) -> Option<(usize, usize)> {
    for row in 0..9 {
        for col in 0..9 {
            if game[row][col] == 0 {
                return Some((row, col));
            }
        }
    }
    return None;
}

fn is_valid_solution(game: GameGrid) -> bool {
    let mut a: HashSet<usize> = HashSet::new();
    let mut b: HashSet<usize> = HashSet::new();
    let mut c: HashSet<usize> = HashSet::new();

    for row in 0..9 {
        check_row(game, &mut a, row);
        check_column(game, &mut b, row);

        if a.len() < 9 || b.len() < 9 {
            return false;
        }

        a.clear();
        b.clear();
    }

    for row in 0..3 {
        for col in 0..3 {
            check_block(game, &mut c, row * 2, col * 2);
            if c.len() < 9 {
                return false;
            }
            c.clear();
        }
    }
    true
}

fn solve_sudoku(mut game: GameGrid) -> () {
    println!("The input puzzle is:");
    print_board(game);
    println!("---------------------------------");
    if play(&mut game) {
        println!("The sudoku puzzle has a solution!");
    } else {
        println!("The sudoku puzzle is NOT solved!");
    }

    // TODO: When do constraint problems like sukdoku have a unique solution?
    if is_valid_solution(game) {
        println!("The solution shown below IS valid:");
    } else {
        println!("The solution shown below is NOT valid");
    }
    print_board(game);
}

fn main() {
    let games: [GameGrid; 2] = [
        [ // easy sudoku
            [0, 3, 0, 0, 0, 0, 6, 2, 0],
            [9, 6, 0, 0, 0, 0, 5, 0, 0],
            [0, 0, 0, 0, 2, 4, 0, 0, 0],
            [8, 0, 9, 5, 0, 0, 0, 1, 0],
            [2, 0, 0, 0, 0, 0, 0, 6, 0],
            [7, 0, 0, 9, 8, 0, 4, 5, 0],
            [3, 0, 8, 0, 0, 5, 9, 0, 7],
            [0, 0, 1, 0, 4, 7, 0, 3, 0],
            [0, 0, 5, 0, 0, 3, 1, 0, 6],
        ],
        [ // hard sudoku
            [0, 8, 6, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 5, 0, 9, 2, 0],
            [0, 0, 0, 0, 4, 0, 5, 0, 3],
            [0, 7, 0, 0, 0, 2, 0, 1, 0],
            [0, 0, 0, 0, 0, 3, 0, 9, 0],
            [0, 2, 8, 0, 0, 7, 0, 0, 0],
            [2, 0, 0, 6, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 3, 4],
            [3, 0, 0, 1, 0, 0, 0, 0, 7],
        ]
    ];

    games.map(solve_sudoku);
}
