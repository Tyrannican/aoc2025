use anyhow::Result;
mod day1;
mod day2;
mod day3;
mod day4;

pub trait Solve {
    fn parse_input(&mut self);
    fn part1(&mut self);
    fn part2(&mut self);
}

pub fn load(day: usize) -> Result<Box<dyn Solve>> {
    match day {
        1 => Ok(Box::new(day1::Solution::new())),
        2 => Ok(Box::new(day2::Solution::new())),
        3 => Ok(Box::new(day3::Solution::new())),
        4 => Ok(Box::new(day4::Solution::new())),
        _ => anyhow::bail!("invalid advent of code day supplied: {day}"),
    }
}
