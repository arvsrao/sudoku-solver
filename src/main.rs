use std::io;
use std::io::Error;
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

fn to_int(ch: char) -> Option<usize> {
   match ch {
    ' ' => Some(0),
    '1' => Some(1),
    '2' => Some(2),
    '3' => Some(3),
    '4' => Some(4),
    '5' => Some(5),
    '6' => Some(6),
    '7' => Some(7),
    '8' => Some(8),
    '9' => Some(9),
      _ => None,
   }
}

fn suffix(idx: usize) -> String {
    match idx {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        _ => "th".to_string(),
    }
}

fn empty(game: GameGrid) -> bool {
    for row in 0..9 { 
        for col in 0..9 {
            if game[row][col] > 0 {
                return false;
            }
        }
    }
    return true;
}

fn read() -> io::Result<GameGrid> {
    let stdin = io::stdin(); // We get `Stdin` here.
    let mut buffer = String::new();

    let mut game: GameGrid =
        [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0]
        ];
    
    // parse one row at a time
    for idx in 0..9 {
        println!("Enter the {}{} row of numbers. Spacebar/(' ') for empty cells", idx+1, suffix(idx+1));
        stdin.read_line(&mut buffer)?;
        let mut idy = 0;
        for cell in buffer.chars() {
            if let Some(v) = to_int(cell) {
                game[idx][idy] = v;
                idy+=1;
            }
        }
        buffer.clear();
    }
    /* special exception for a sudoku board without any inital 
     *  conditions.
     */
    let empty_game = Error::other("Empty Game Board !!");

    if empty(game) {
        return Err(empty_game);
    }
    return Ok(game);
}

fn main() {
    match read() {
        Ok(game) => solve_sudoku(game),
        Err(e)   => println!("error parsing user input: {}", e),
    }
}
