use std::env;
mod problem;
mod solution;

use problem::*;
use solution::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem;
    match &args[..] {
        [_, f] => problem = Problem::new(f),
        _ => panic!("Incorrect input."),
    }
    
    let mut state = State::new(problem.dim);
    let mut solution = Solution::new(problem);
    solution.find(0, 0, &mut state);
    solution.output();
}


