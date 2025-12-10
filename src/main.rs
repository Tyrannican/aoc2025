mod solutions;

use crate::solutions::load;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(arg) = args.get(1) else {
        eprintln!("Require a valid AoC day!");
        return;
    };
    let day = arg.parse().unwrap();
    let mut soln = load(day);
    soln.part1();
    soln.part2();
}
