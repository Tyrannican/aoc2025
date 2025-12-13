use super::Solve;

#[derive(Default)]
pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {}

    fn part1(&mut self) {}

    fn part2(&mut self) {}
}
