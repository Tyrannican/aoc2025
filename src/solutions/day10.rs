use super::Solve;

const DEBUG: bool = true;

#[derive(Debug, Default)]
struct Machine {
    state: u16,
    target: u16,
    wiring: Vec<u16>,
    joltage: Vec<usize>,
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
                        for (i, ch) in slice.iter().rev().enumerate() {
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
                                    let idx = ch - b'0';
                                    wire |= 1 << idx;
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
        let total = 0;
        for machine in self.machines.iter_mut() {
            println!("{machine:?}");
        }
        // let total: usize = self.machines.iter().map(|m| m.fewest_presses()).sum();
        println!("Part 1: {total}");
    }

    fn part2(&mut self) {}
}
