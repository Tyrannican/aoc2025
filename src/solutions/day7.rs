use super::Solve;

type Pos = (usize, usize);

#[derive(Default)]
pub struct Solution {
    depth: usize,
    start: Pos,
    splits: usize,
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
        let content = std::fs::read_to_string("./inputs/day7.txt").expect("it's fine");
        let mut count = 0;
        let mut lanes = [0_u8; 256];
        let _ = content
            .split("\n")
            .enumerate()
            .map(|(x, line)| {
                for (y, ch) in line.as_bytes().iter().enumerate() {
                    match *ch {
                        b'S' => {
                            lanes[y] = 1;
                            self.start = (x, y);
                        }
                        b'^' => {
                            if lanes[y] == 0 {
                                continue;
                            }

                            if lanes[y - 1] == 0 {
                                lanes[y - 1] = 1;
                            }

                            if lanes[y + 1] == 0 {
                                lanes[y + 1] = 1;
                            }

                            lanes[y] = 0;
                            count += 1;
                        }
                        _ => {}
                    }
                }

                self.depth += 1;
            })
            .collect::<Vec<_>>();

        self.splits = count;
    }

    fn part1(&mut self) {
        println!("Part 1: {}", self.splits);
    }

    fn part2(&mut self) {}
}
