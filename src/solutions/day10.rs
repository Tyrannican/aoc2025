use std::collections::{HashSet, VecDeque};

use super::Solve;

const DEBUG: bool = false;

#[derive(Debug, Default)]
struct Machine {
    target: u16,
    wiring: Vec<u16>,
    joltage: Vec<usize>,
}

impl Machine {
    pub fn min_presses(&self) -> usize {
        let mut queue: VecDeque<(u16, usize)> = VecDeque::new();
        let mut seen = HashSet::new();
        seen.insert(0);
        queue.push_back((0, 0));

        while !queue.is_empty() {
            let (current, steps) = queue.pop_front().unwrap();

            for wire in self.wiring.iter() {
                let state = current ^ wire;
                if state == self.target {
                    return steps + 1;
                }

                if !seen.contains(&state) {
                    seen.insert(state);
                    queue.push_back((state, steps + 1));
                }
            }
        }

        0
    }
}

#[derive(Default)]
pub struct Solution {
    machines: Vec<Machine>,
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
            "./inputs/day10_debug.txt"
        } else {
            "./inputs/day10.txt"
        };

        let content = std::fs::read_to_string(path).expect("it exists");
        for line in content.trim().split('\n') {
            let mut machine = Machine::default();

            for item in line.split(' ') {
                let bytes = item.as_bytes();
                match bytes[0] {
                    // Lights
                    b'[' => {
                        let slice = &bytes[1..bytes.len() - 1];
                        for (i, ch) in slice.iter().enumerate() {
                            match ch {
                                b'#' => {
                                    machine.target |= 1 << i;
                                }
                                _ => {}
                            }
                        }
                    }

                    // Instructions
                    b'(' => {
                        let mut wire: u16 = 0;
                        for ch in bytes.iter() {
                            match ch {
                                b'0'..=b'9' => {
                                    wire |= 1 << (ch - b'0');
                                }
                                _ => {}
                            }
                        }

                        machine.wiring.push(wire);
                    }

                    // Joltage -- TODO
                    b'{' => {
                        let mut digit = String::new();
                        for ch in bytes.iter() {
                            match ch {
                                b'0'..=b'9' => digit.push(*ch as char),
                                b',' => {
                                    machine.joltage.push(digit.parse::<usize>().unwrap());
                                    digit.clear();
                                }
                                _ => {}
                            }
                        }

                        machine.joltage.push(digit.parse::<usize>().unwrap());
                    }
                    _ => unreachable!("this is impossible given the problem input"),
                }
            }

            self.machines.push(machine);
        }
    }

    fn part1(&mut self) {
        let min_steps = self.machines.iter().map(|m| m.min_presses()).sum::<usize>();
        println!("Part 1: {min_steps}");
    }

    fn part2(&mut self) {}
}
