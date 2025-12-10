use super::Solve;

const DEBUG: bool = true;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Light {
    On,
    Off,
}

impl Light {
    fn switch(&mut self) {
        match self {
            Self::On => *self = Self::Off,
            Self::Off => *self = Self::On,
        }
    }
}

#[derive(Debug, Default)]
struct Machine {
    lights: Vec<Light>,
    config: Vec<Light>,
    instructions: Vec<Vec<u8>>,
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
                        let mut configuration = Vec::new();
                        for ch in bytes.iter() {
                            match ch {
                                b'#' => configuration.push(Light::On),
                                b'.' => configuration.push(Light::Off),
                                _ => {}
                            }
                        }

                        let len = configuration.len();
                        machine.config = configuration;
                        machine.lights = vec![Light::Off; len];
                    }

                    // Instructions
                    b'(' => {
                        let mut instructions = Vec::new();
                        for ch in bytes.iter() {
                            match ch {
                                b'0'..=b'9' => {
                                    instructions.push(ch - b'0');
                                }
                                _ => {}
                            }
                        }

                        machine.instructions.push(instructions);
                    }

                    // Joltage
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
        for machine in self.machines.iter() {
            println!("{machine:?}");
        }
    }

    fn part2(&mut self) {}
}
