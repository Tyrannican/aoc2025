use super::Solve;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Number(isize),
    Mul,
    Add,
}

#[derive(Debug)]
struct Problem {
    values: Vec<isize>,
    operation: Operation,
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            operation: Operation::Add,
        }
    }
}

#[derive(Default)]
pub struct Solution {
    problems: Vec<Problem>,
    raw: Vec<String>,
}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }

    fn raw_to_problems(&self) -> Vec<Problem> {
        let mut problem_list = Vec::new();
        let (problems, op_str) = self.raw.split_at(self.raw.len() - 1);
        let operations = self.op_str_to_operations(&op_str[0]);

        let mut idx = (problems[0].len() - 1) as isize;
        let mut op_idx = operations.len() - 1;

        let mut values = Vec::new();
        while idx >= 0 {
            let mut s = String::new();
            for row in problems.iter() {
                let bytes = row.as_bytes();
                s.push(bytes[idx as usize] as char);
            }

            let s = s.trim();
            if !s.is_empty() {
                values.push(s.parse::<isize>().unwrap());
            } else {
                let problem = Problem {
                    values: values.drain(..).collect(),
                    operation: operations[op_idx],
                };
                problem_list.push(problem);
                op_idx -= 1;
            }

            idx -= 1;
        }

        problem_list.push(Problem {
            values,
            operation: operations[0],
        });

        problem_list
    }

    fn op_str_to_operations(&self, s: &str) -> Vec<Operation> {
        s.split(" ")
            .filter_map(|s| {
                let s = s.trim().as_bytes();
                if s.is_empty() {
                    None
                } else {
                    match s[0] {
                        b'*' => Some(Operation::Mul),
                        b'+' => Some(Operation::Add),
                        _ => unreachable!("no digits here"),
                    }
                }
            })
            .collect::<Vec<Operation>>()
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {
        let content = std::fs::read_to_string("./inputs/day6.txt").expect("it's valid");

        let operations: Vec<Vec<Operation>> = content
            .trim()
            .split("\n")
            .map(|line| {
                self.raw.push(line.to_string());
                line.split(" ")
                    .filter_map(|s| {
                        let s = s.trim();
                        if s.is_empty() {
                            return None;
                        }

                        let bytes = s.as_bytes();
                        let first = bytes[0];
                        if char::from(first).is_numeric() {
                            return Some(Operation::Number(s.parse().unwrap()));
                        } else {
                            match first {
                                b'*' => return Some(Operation::Mul),
                                b'+' => return Some(Operation::Add),
                                _ => {}
                            }

                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let mut idx = 0;
        while idx < operations[0].len() {
            let mut problem = Problem::default();
            for operation in operations.iter() {
                match operation[idx] {
                    Operation::Number(value) => problem.values.push(value),
                    o => problem.operation = o,
                }
            }
            self.problems.push(problem);
            idx += 1;
        }
    }

    fn part1(&mut self) {
        let total = self
            .problems
            .iter()
            .map(|problem| match problem.operation {
                Operation::Mul => problem.values.iter().product::<isize>(),
                Operation::Add => problem.values.iter().sum(),
                _ => unreachable!("operations are never set to Number"),
            })
            .sum::<isize>();

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let problems = self.raw_to_problems();
        let total = problems
            .iter()
            .map(|problem| match problem.operation {
                Operation::Mul => problem.values.iter().product::<isize>(),
                Operation::Add => problem.values.iter().sum(),
                _ => unreachable!("operations are never set to Number"),
            })
            .sum::<isize>();

        println!("Part 2: {total}");
    }
}
