use std::collections::{HashMap, HashSet};

use super::Solve;

#[derive(Default)]
pub struct Solution {
    ids: Vec<(u64, u64)>,
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
        let content = std::fs::read_to_string("./inputs/day2.txt").expect("it's a text file...");
        self.ids = content
            .trim()
            .split(',')
            .map(|id_range| {
                let (s, e) = id_range
                    .split_once('-')
                    .expect("ids are structured like this");

                (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        for (mut start, end) in self.ids.iter().copied() {
            while start <= end {
                let num_digits = start.checked_ilog10().unwrap_or(0) + 1;
                let pow = 10_i32.pow(num_digits / 2);
                let left = start / pow as u64;
                let right = start % pow as u64;

                if left == right {
                    total += start;
                    start += 10;
                } else {
                    start += 1;
                }
            }
        }

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let mut total = 0;
        for (mut start, end) in self.ids.iter().copied() {
            if start < 10 {
                start = 10
            }

            while start <= end {
                if start < 100 {
                    if start % 10 == start / 10 {
                        total += start;
                        start += 10;
                    }
                } else {
                    let s = start.to_string();

                    for i in 0..=s.len() / 2 {
                        let first: String = s.chars().take(i).collect();
                        let mut second: String = s.chars().skip(i).take(s.len() - i).collect();
                        second = second.replace(&first, "");
                        if second.is_empty() {
                            total += start;
                            break;
                        }
                    }
                }

                start += 1;
            }
        }

        println!("Part 2: {total}");
    }
}
