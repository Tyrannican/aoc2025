use std::collections::{HashSet, VecDeque};

use super::Solve;

const DEBUG: bool = false;

#[derive(Debug, Default)]
struct Machine {
    target: u16,
    wiring: Vec<u16>,
    joltage: Vec<u16>,
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

    pub fn min_presses_joltage(&self) -> usize {
        0
    }
}

#[inline]
fn bit_idxs(mut wire: u16) -> Vec<usize> {
    let mut idxs = Vec::with_capacity(16);
    while wire != 0 {
        idxs.push(wire.trailing_zeros() as usize);
        wire &= wire - 1;
    }
    idxs
}

#[inline]
fn apply_joltage(wire: u16, joltage: &mut [u16]) {
    let idxs = bit_idxs(wire);
    idxs.iter().for_each(|i| joltage[*i] += 1);
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
                            if *ch == b'#' {
                                machine.target |= 1 << i;
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
                                b')' => {
                                    machine.wiring.push(wire);
                                }
                                _ => {}
                            }
                        }
                    }

                    // Joltage -- TODO
                    b'{' => {
                        let mut digit = String::new();
                        for ch in bytes.iter() {
                            match ch {
                                b'0'..=b'9' => digit.push(*ch as char),
                                b',' => {
                                    machine.joltage.push(digit.parse::<u16>().unwrap());
                                    digit.clear();
                                }
                                b'}' => {
                                    machine.joltage.push(digit.parse::<u16>().unwrap());
                                    machine.joltage.resize(16, 0);
                                }
                                _ => {}
                            }
                        }
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

    fn part2(&mut self) {
        for machine in self.machines.iter() {
            let total = machine.min_presses_joltage();
            println!("Total: {total}");
            break;
        }
    }
}
