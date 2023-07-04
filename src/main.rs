// Solution struct
struct Solution {
    puzzle: [[i8; 9]; 9],
    given: i8,
    // history: Vec<(i8, i8)>,
}

// Methods
impl Solution {
    fn at(&self, row: usize, col: usize) -> i8 {
        return self.puzzle[row][col];
    }

    fn _print(&self) {
        let mut subsquare_counter = 0;
        println!("+-------+-------+-------+");
        for row in self.puzzle {
            if subsquare_counter == 3 {
                println!("+-------+-------+-------+");
                subsquare_counter = 0;
            }
            subsquare_counter += 1;
            print!("| ");
            let mut counter = 0;
            for col in row {
                print!("{}", col);
                counter += 1;
                if counter == 3 {
                    print!(" | ");
                    counter = 0;
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
        println!("+-------+-------+-------+");
        println!("Squres Left: {}", 81 - self.given);
    }
}

fn init() -> Solution {
    let mut puzzle = Solution {
        puzzle: [
            [0, 0, 7, 8, 0, 1, 4, 6, 9],
            [0, 0, 0, 0, 7, 0, 3, 0, 0],
            [6, 1, 0, 0, 0, 0, 0, 8, 0],
            [0, 0, 0, 4, 1, 0, 6, 0, 0],
            [5, 0, 1, 0, 2, 8, 0, 3, 0],
            [4, 2, 9, 7, 6, 0, 8, 0, 5],
            [7, 0, 0, 3, 0, 0, 1, 0, 0],
            [0, 0, 6, 2, 0, 4, 5, 7, 0],
            [0, 3, 2, 0, 0, 0, 9, 4, 0],
        ],
        given: 0,
        //history: vec![],
    };

    for row in puzzle.puzzle {
        for col in row {
            if col != 0 {
                puzzle.given += 1;
            }
        }
    }

    return puzzle;
}

fn is_number_possible(p: &Solution, row: usize, col: usize, n: i8) -> bool {
    println!("Row {row}, Col: {col}, Number: {n}");
    if p.at(row, col) != 0 {
        return false;
    }
    // check the square
    // gets the top left part of the square its in
    let square_col = match col {
        0 | 1 | 2 => 0,
        3 | 4 | 5 => 3,
        6 | 7 | 8 => 6,
        _ => 10,
    };
    let square_row = match row {
        0 | 1 | 2 => 0,
        3 | 4 | 5 => 3,
        6 | 7 | 8 => 6,
        _ => 10,
    };
    for row_ in square_row..square_row + 3 {
        for col_ in square_col..square_col + 3 {
            if n == p.at(row_, col_) {
                println!("Num {}, Row: {row_}, Col: {col_}", p.at(row_, col_));
                return false;
            }
        }
    }
    // check the col
    for _row in 0..9 {
        if _row == row {
            continue;
        }
        if n == p.at(_row, col) {
            return false;
        }
    }
    // check the row
    for _col in 0..9 {
        if _col == col {
            continue;
        }
        if n == p.at(row, _col) {
            return false;
        }
    }
    return true;
}

fn solve() -> Solution {
    let puzzle = init();

    let x = is_number_possible(&puzzle, 4, 1, 6);
    println!("{x}");

    return puzzle;
}

fn main() {
    println!("Sudoku solver");
    let p = init();
    p._print();
    let _sln = solve();
    //sln._print();
}
