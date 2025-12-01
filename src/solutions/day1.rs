use super::Solve;

#[derive(Default)]
pub struct Solution {
    lock: Lock,
}

impl Solution {
    pub fn new() -> Self {
        let mut s = Solution::default();
        s.parse_input();
        s
    }
}

struct Lock {
    current: isize,
    zero: usize,
    pass_zero: usize,
}

impl Lock {
    #[inline]
    pub fn update(&mut self, value: isize) {
        self.pass_zero += (value / 100).abs() as usize;

        let value = value % 100;
        let diff = self.current + value;

        match diff {
            ..0 => {
                if self.current != 0 {
                    self.pass_zero += 1;
                }

                self.current = 100 - diff.abs();
            }
            0 => {
                self.zero += 1;
                self.current = diff;
            }
            1..100 => self.current = diff,
            100 => {
                self.current = 0;
                self.zero += 1;
            }
            100.. => {
                self.current = diff - 100;
                self.pass_zero += 1;
            }
        }
    }
}

impl std::fmt::Display for Lock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lock(current={},zero={},pass_zero={})",
            self.current, self.zero, self.pass_zero
        )
    }
}

impl Default for Lock {
    fn default() -> Self {
        Self {
            current: 50,
            zero: 0,
            pass_zero: 0,
        }
    }
}

impl Solve for Solution {
    fn parse_input(&mut self) {
        let content = std::fs::read_to_string("./inputs/day1.txt").expect("this is fine");
        let _ = content
            .lines()
            .map(|line| {
                let (dir, num) = line.split_at(1);
                let mut value = num.parse::<isize>().expect("should be valid");
                if dir == "L" {
                    value = -value
                }

                self.lock.update(value);
            })
            .collect::<Vec<_>>();
    }

    fn part1(&mut self) {
        println!("Part 1: {}", self.lock.zero);
    }

    fn part2(&mut self) {
        println!("Part 2: {}", self.lock.zero + self.lock.pass_zero);
    }
}
