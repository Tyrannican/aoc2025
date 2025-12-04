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
            .map(|input| largest_joltage(input, 0, 0, 2))
            .sum::<usize>();

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let total = self
            .inputs
            .iter()
            .map(|input| largest_joltage(input, 0, 0, 12))
            .sum::<usize>();

        println!("Part 2: {total}");
    }
}

fn largest_joltage(input: &[u8], idx: usize, total: usize, size: usize) -> usize {
    if size == 0 {
        return total;
    }

    let len = input.len();
    let mut largest = idx;

    for i in idx..=(len - size) {
        if input[i] > input[largest] {
            largest = i;
        }
    }

    largest_joltage(
        input,
        largest + 1,
        total * 10 + input[largest] as usize,
        size - 1,
    )
}
