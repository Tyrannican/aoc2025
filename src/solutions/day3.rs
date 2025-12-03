use super::Solve;

#[derive(Default)]
pub struct Solution {
    inputs: Vec<Vec<char>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {
        let content = std::fs::read_to_string("./inputs/day3.txt").expect("it exists");
        self.inputs = content
            .trim()
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();
    }

    fn part1(&mut self) {
        let total = self
            .inputs
            .iter()
            .map(|input| {
                let max = input.iter().max().unwrap();
                let idx = input.iter().position(|c| c == max).unwrap();
                let (left, mut right) = input.split_at(idx);
                let max = right[0];
                if right.len() == 1 {
                    let other = left.iter().max().unwrap();
                    let idx = left.iter().position(|c| c == other).unwrap();
                    let next = left[idx];
                    format!("{next}{max}").parse::<usize>().unwrap()
                } else {
                    right = &right[1..];
                    let other = right.iter().max().unwrap();
                    let idx = right.iter().position(|c| c == other).unwrap();
                    let next = right[idx];
                    format!("{max}{next}").parse::<usize>().unwrap()
                }
            })
            .sum::<usize>();

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {}
}
