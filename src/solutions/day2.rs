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
                    let n = start.to_string();
                    let limit = n.len() / 2;
                    for i in 0..=limit {
                        let first: String = n.chars().take(i).collect();
                        let mut second: String = n.chars().skip(i).take(n.len() - i).collect();
                        second = second.replace(&first, "");
                        if second.is_empty() {
                            total += start;
                            break;
                        }
                    }
                    // let digits = start.to_string().chars().collect::<Vec<char>>();
                    // for i in 0..digits.len() / 2 {
                    //     let ch = &digits[0..=i];
                    //     let slice = ch
                    //         .iter()
                    //         .cycle()
                    //         .take(digits.len())
                    //         .copied()
                    //         .collect::<Vec<char>>();
                    //     if slice == digits {
                    //         total += start;
                    //         break;
                    //     }
                    // }
                    // println!("{start} {digits:?}");
                }

                start += 1;
            }
        }

        println!("Part 2: {total}");
    }
}
