use super::Solve;
use std::collections::{HashMap, VecDeque};

const DEBUG: bool = false;

#[derive(Default)]
pub struct Solution {
    devices: HashMap<String, Vec<String>>,
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
        let path = if DEBUG {
            "./inputs/day11_debug.txt"
        } else {
            "./inputs/day11.txt"
        };

        let content = std::fs::read_to_string(path).expect("it exists");
        let _: Vec<_> = content
            .trim()
            .split('\n')
            .map(|line| {
                let (node, neighbours) = line
                    .split_once(": ")
                    .expect("guaranteed from problem input");

                let neighbours = neighbours.split(" ").map(|s| s.to_string()).collect();
                self.devices.insert(node.to_string(), neighbours);
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut paths = 0;
        let mut queue = VecDeque::new();
        queue.push_back("you".to_string());
        let target = "out".to_string();

        while let Some(curr) = queue.pop_front() {
            let neighbours = self.devices.get(&curr).unwrap();
            if neighbours.contains(&target) {
                paths += 1;
            } else {
                for neighbour in neighbours.iter() {
                    queue.push_front(neighbour.clone());
                }
            }
        }

        println!("Total paths: {paths}");
    }

    fn part2(&mut self) {}
}
