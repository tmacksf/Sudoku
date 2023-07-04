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

    fn set(&mut self, row: usize, col: usize, num: i8) {
        self.puzzle[row][col] = num;
    }

    fn next_empty(&self, mut row: usize, mut col: usize) -> (usize, usize) {
        if row == 8 && col == 8 {
            return (10, 10);
        }

        loop {
            col += 1;

            if col > 8 {
                col = 0;
                row += 1
            }
            if row > 8 {
                return (10, 10);
            }

            if self.at(row, col) == 0 {
                return (row, col);
            }
        }
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
    if p.at(row, col) != 0 {
        return false;
    }
    if col > 8 {
        eprintln!("Out of bounds for col: {col}");
        return false;
    } else if row > 8 {
        eprintln!("Out of bounds for row: {row}");
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

fn individual_square(p: &mut Solution, row: usize, col: usize) -> bool {
    // checks an individual square
    let mut b = false;
    for row in row..row + 3 {
        let mut possible = 0;
        let mut c = 0;
        let mut n = 0;
        for num in 0..9 {
            for col in col..col + 3 {
                if is_number_possible(p, row, col, num) {
                    possible += 1;
                    c = col;
                    n = num;
                }
            }
        }
        if possible == 1 {
            println!("Row: {row}, Col: {c}, Num: {n}");
            // check just in case
            if is_number_possible(p, row, c, n) {
                p.set(row, c, n);
                b = true;
            }
        }
    }
    return b;
}

fn square_option(p: &mut Solution) -> bool {
    // checks every square to see if there are places where a number has only 1 space to filled
    let square_row = [0, 3, 6];
    let square_col = [0, 3, 6];
    let mut b = false;
    let mut temp;

    for row in square_row {
        for col in square_col {
            temp = individual_square(p, row, col);
            if (temp == true) {
                b = true;
            }
        }
    }

    return b;
}

fn col_option(p: &mut Solution) -> bool {
    // checks every column to see if there are places where a number only has 1 space to fill

    return false;
}

fn row_option(p: &mut Solution) -> bool {
    //

    return false;
}

fn only_option(p: &mut Solution) -> bool {
    // checks to see if there are any squares where only 1 number is available
    let mut row = 0;
    let mut col = 0;
    let mut filled = 0;
    (col, row) = p.next_empty(row, col);

    while col != 10 {
        println!("Row: {row}, Col: {col}");
        let mut flag = 0;
        let mut num = 0;
        for n in 0..9 {
            if is_number_possible(p, row, col, n) {
                flag += 1;
                num = n;
            }
        }
        if flag == 1 {
            filled += 1;
            p.set(row, col, num);
        }
        (col, row) = p.next_empty(1 + row, col);
    }

    if filled != 0 {
        return true;
    }

    return false;
}

fn solve() -> Solution {
    let mut puzzle = init();
    //let mut puzzle = puzzleinit;
    puzzle._print();

    let mut solved = only_option(&mut puzzle);

    while solved {
        solved = only_option(&mut puzzle);
    }
    square_option(&mut puzzle);

    return puzzle;
}

fn main() {
    println!("Sudoku solver");
    let sln = solve();
    sln._print();
}
