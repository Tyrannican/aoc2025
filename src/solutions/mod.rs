mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub trait Solve {
    fn parse_input(&mut self);
    fn part1(&mut self);
    fn part2(&mut self);
}

pub fn load(day: usize) -> Box<dyn Solve> {
    match day {
        1 => Box::new(day1::Solution::new()),
        2 => Box::new(day2::Solution::new()),
        3 => Box::new(day3::Solution::new()),
        4 => Box::new(day4::Solution::new()),
        5 => Box::new(day5::Solution::new()),
        6 => Box::new(day6::Solution::new()),
        7 => Box::new(day7::Solution::new()),
        8 => Box::new(day8::Solution::new()),
        9 => Box::new(day9::Solution::new()),
        _ => panic!("invalid advent of code day supplied: {day}"),
    }
}
