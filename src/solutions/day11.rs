use super::Solve;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Hash, Eq)]
struct StrVec {
    key: [u8; 3],
}

impl StrVec {
    fn new(s: &[u8]) -> Self {
        let mut key = [0; 3];
        key[..s.len()].copy_from_slice(s);
        Self { key }
    }
}

impl AsRef<[u8]> for StrVec {
    fn as_ref(&self) -> &[u8] {
        &self.key
    }
}

impl PartialEq for StrVec {
    fn eq(&self, other: &Self) -> bool {
        self.key.as_ref() == other.key.as_ref()
    }
}

#[derive(Default)]
pub struct Solution {
    devices: HashMap<StrVec, HashSet<StrVec>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }

    fn valid_paths(
        &self,
        curr: &StrVec,
        dst: &StrVec,
        visited: &mut HashSet<StrVec>,
        seen_fft: bool,
        seen_dac: bool,
        cache: &mut HashMap<(StrVec, bool, bool), usize>,
    ) -> usize {
        if curr == dst {
            return if seen_fft && seen_dac { 1 } else { 0 };
        }

        let key = (*curr, seen_fft, seen_dac);
        if let Some(&cached) = cache.get(&key) {
            return cached;
        }

        let mut count = 0;
        let sfft = seen_fft || *curr == StrVec::new(b"fft");
        let sdac = seen_dac || *curr == StrVec::new(b"dac");

        if let Some(neighbours) = self.devices.get(curr) {
            for neighbour in neighbours.iter() {
                if !visited.contains(neighbour) {
                    visited.insert(*neighbour);
                    count += self.valid_paths(neighbour, dst, visited, sfft, sdac, cache);
                    visited.remove(neighbour);
                }
            }
        }

        cache.insert(key, count);

        count
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {
        let content = std::fs::read_to_string("./inputs/day11.txt").expect("it exists");
        let _: Vec<_> = content
            .trim()
            .split('\n')
            .map(|line| {
                let (node, neighbours) = line
                    .split_once(": ")
                    .expect("guaranteed from problem input");

                let neighbours = neighbours
                    .split(" ")
                    .map(|s| StrVec::new(s.as_bytes()))
                    .collect();

                self.devices
                    .insert(StrVec::new(node.as_bytes()), neighbours);
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut paths = 0;
        let mut queue = VecDeque::new();
        queue.push_back(StrVec::new("you".as_bytes()));
        let target = StrVec::new("out".as_bytes());

        while let Some(curr) = queue.pop_front() {
            let neighbours = self.devices.get(&curr).unwrap();
            if neighbours.contains(&target) {
                paths += 1;
            } else {
                for neighbour in neighbours.iter() {
                    queue.push_front(*neighbour);
                }
            }
        }

        println!("Part 1: {paths}");
    }

    fn part2(&mut self) {
        let src = StrVec::new(b"svr");
        let dst = StrVec::new(b"out");
        let mut visited = HashSet::new();
        let mut cache = HashMap::new();
        let count = self.valid_paths(&src, &dst, &mut visited, false, false, &mut cache);
        println!("Part 2: {count}");
    }
}
