use std::env;

use icub3d_sudoku_solver::Board;

use failure::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut b = Board::new(args[1].to_string()).unwrap();
    if b.solve() {
        print!("{}", b.to_string());
    } else {
        println!("could not be solved");
    }
    Ok(())
}
