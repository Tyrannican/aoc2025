use super::Solve;
use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

#[derive(Default)]
pub struct Solution {
    start: Pos,
    splits: usize,
    splitters: HashSet<Pos>,
    depth: usize,
}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }

    fn meets_splitter(&self, beam: Pos) -> Option<(Pos, Pos)> {
        let (mut x, y) = beam;
        while x < self.depth {
            if !self.splitters.contains(&(x + 1, y)) {
                x += 1;
            } else {
                return Some(((x, y - 1), (x, y + 1)));
            }
        }

        None
    }

    fn timelines(&self, beam: Pos, cache: &mut HashMap<Pos, usize>) -> usize {
        if let Some(&cached) = cache.get(&beam) {
            return cached;
        }

        let result = match self.meets_splitter(beam) {
            Some((left, right)) => self.timelines(left, cache) + self.timelines(right, cache),
            None => 1,
        };

        cache.insert(beam, result);

        result
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {
        let content = std::fs::read_to_string("./inputs/day7.txt").expect("it's fine");
        let mut splits = 0;
        let mut lanes = [false; 256];
        let _ = content
            .split("\n")
            .enumerate()
            .map(|(x, line)| {
                for (y, ch) in line.as_bytes().iter().enumerate() {
                    match *ch {
                        b'S' => {
                            lanes[y] = true;
                            self.start = (x, y);
                        }
                        b'^' => {
                            if !lanes[y] {
                                continue;
                            }

                            lanes[y] = false;
                            if !lanes[y - 1] {
                                lanes[y - 1] = true;
                            }

                            if !lanes[y + 1] {
                                lanes[y + 1] = true;
                            }

                            splits += 1;
                            self.splitters.insert((x, y));
                        }
                        _ => {}
                    }
                }

                self.depth += 1;
            })
            .collect::<Vec<_>>();

        self.splits = splits;
    }

    fn part1(&mut self) {
        println!("Part 1: {}", self.splits);
    }

    fn part2(&mut self) {
        let mut cache = HashMap::new();
        let result = self.timelines(self.start, &mut cache);
        println!("Part 2: {result}");
    }
}
