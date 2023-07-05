// Solution struct
mod solver;

fn main() {
    println!("Sudoku solver");
    let sln = solver::solve();
    sln.pretty_print();
}
