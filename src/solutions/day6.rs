use super::Solve;

#[derive(Debug, Clone, Copy)]
enum Op {
    Number(isize),
    Mul,
    Add,
}

#[derive(Default)]
pub struct Solution {
    operations: Vec<Vec<Op>>,
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
        let content = std::fs::read_to_string("./inputs/day6.txt").expect("it's valid");

        let operations: Vec<Vec<Op>> = content
            .trim()
            .split("\n")
            .map(|line| {
                let line = line.trim();
                line.split(" ")
                    .filter_map(|s| {
                        let s = s.trim();
                        if s.is_empty() {
                            return None;
                        }

                        let bytes = s.as_bytes();
                        let first = bytes[0];
                        if char::from(first).is_numeric() {
                            return Some(Op::Number(s.parse().unwrap()));
                        } else {
                            match first {
                                b'*' => return Some(Op::Mul),
                                b'+' => return Some(Op::Add),
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
            let mut inner = Vec::with_capacity(operations.len() - 1);
            for op in operations.iter() {
                inner.push(op[idx]);
            }
            self.operations.push(inner);
            idx += 1;
        }
    }

    fn part1(&mut self) {
        let mut total = 0;
        for operation in self.operations.iter() {
            let values = &operation[..operation.len() - 1]
                .into_iter()
                .map(|o| match o {
                    Op::Number(value) => *value,
                    _ => panic!("ahh"),
                })
                .collect::<Vec<isize>>();

            let op = &operation[operation.len() - 1];
            let value = match op {
                Op::Add => values.iter().sum::<isize>(),
                Op::Mul => values.iter().product::<isize>(),
                _ => panic!("ahh"),
            };

            println!("Values: {values:?} Op: {op:?} - Result: {value}");
            total += value;
        }

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {}
}
