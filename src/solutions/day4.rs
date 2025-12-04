use std::collections::HashSet;

use super::Solve;

#[derive(PartialEq)]
enum Roll {
    Empty,
    Full,
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Default)]
pub struct Solution {
    width: usize,
    paper: Vec<Roll>,
}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }

    fn dist(&self, idx: isize) -> (isize, isize) {
        (idx / self.width as isize, idx % self.width as isize)
    }

    pub fn positions(&self, idx: isize) -> Vec<isize> {
        let mut out = Vec::with_capacity(8);
        let (curr_row, curr_col) = self.dist(idx);
        for i in [idx - self.width as isize, idx, idx + self.width as isize] {
            for j in -1isize..=1 {
                let (other_row, other_col) = self.dist(i + j);
                let dist = curr_row.abs_diff(other_row) + curr_col.abs_diff(other_col);
                if dist <= 2 && i + j != idx {
                    out.push(i + j);
                }
            }
        }

        out
    }

    pub fn indices(&self) -> HashSet<isize> {
        self.paper
            .iter()
            .enumerate()
            .filter_map(|(idx, paper)| {
                if *paper == Roll::Full {
                    Some(idx as isize)
                } else {
                    None
                }
            })
            .collect::<HashSet<isize>>()
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {
        let input = std::fs::read("./inputs/day4.txt").expect("this exists");
        self.width = input
            .iter()
            .position(|b| *b == b'\n')
            .expect("file is guaranteed to have new lines");

        self.paper = input
            .into_iter()
            .filter_map(|b| match b {
                b'.' => Some(Roll::Empty),
                b'@' => Some(Roll::Full),
                _ => None,
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        let indices = self.indices();
        for item in indices.iter() {
            let positions = self.positions(*item);
            let count = positions
                .into_iter()
                .filter(|p| indices.contains(&p))
                .count();

            if count < 4 {
                total += 1;
            }
        }

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let mut total = 0;

        loop {
            let mut found = 0;
            let indices = self.indices();
            for item in indices.iter() {
                let positions = self.positions(*item);
                let count = positions
                    .into_iter()
                    .filter(|p| indices.contains(&p))
                    .count();

                if count < 4 {
                    found += 1;
                    self.paper[*item as usize] = Roll::Empty;
                }
            }

            if found != 0 {
                total += found;
            } else {
                break;
            }
        }

        println!("Part 2: {total}");
    }
}
