use super::Solve;

#[derive(Default)]
pub struct Solution {
    inputs: Vec<Vec<u8>>,
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
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
    }

    fn part1(&mut self) {
        let total = self
            .inputs
            .iter()
            .map(|input| {
                let max = input.iter().max().unwrap();
                let idx = input.iter().position(|c| c == max).unwrap();
                let val = input[idx];

                let (slice, rev) = if idx == input.len() - 1 {
                    (&input[..idx], true)
                } else {
                    (&input[idx + 1..], false)
                };

                let max = slice.iter().max().unwrap();
                let idx = slice.iter().position(|c| c == max).unwrap();
                let next = slice[idx];

                if rev {
                    format!("{next}{val}").parse::<usize>().unwrap()
                } else {
                    format!("{val}{next}").parse::<usize>().unwrap()
                }
            })
            .sum::<usize>();

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let mut total = 0;
        for input in self.inputs.iter() {}

        println!("Part 2: {total}");
    }
}
