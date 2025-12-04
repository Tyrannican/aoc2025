mod solutions;

use crate::solutions::load;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].parse().unwrap();
    let mut soln = load(day);
    soln.part1();
    soln.part2();
}
