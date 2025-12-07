use super::Solve;

type Pos = (usize, usize);

#[derive(Default)]
pub struct Solution {
    start: Pos,
    splits: usize,
    splitters: Vec<Pos>,
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
        let mut splits = 0;
        let mut lanes = [false; 256];
        let _ = content
            .split("\n")
            .enumerate()
            .map(|(x, line)| {
                for (y, ch) in line.as_bytes().iter().enumerate() {
                    match *ch {
                        b'S' => {
                            lanes[y] = true;
                            self.start = (x, y);
                        }
                        b'^' => {
                            if !lanes[y] {
                                continue;
                            }

                            lanes[y] = false;
                            if !lanes[y - 1] {
                                lanes[y - 1] = true;
                            }

                            if !lanes[y + 1] {
                                lanes[y + 1] = true;
                            }

                            splits += 1;
                            self.splitters.push((x, y));
                        }
                        _ => {}
                    }
                }
            })
            .collect::<Vec<_>>();

        self.splits = splits;
    }

    fn part1(&mut self) {
        println!("Part 1: {}", self.splits);
    }

    fn part2(&mut self) {
        println!("Splitters: {:?}", self.splitters);
    }
}
