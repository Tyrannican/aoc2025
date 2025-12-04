use super::Solve;

const CARDINALS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const MAX_ROLLS: usize = 4;

#[derive(Default)]
pub struct Solution {
    paper: Vec<Vec<u8>>,
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
        let input = std::fs::read_to_string("./inputs/day4.txt").expect("this exists");
        for line in input.lines() {
            self.paper.push(line.as_bytes().to_vec());
        }
    }

    fn part1(&mut self) {
        let mut total = 0;
        for (x, row) in self.paper.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                let mut count = 0;
                if *col != b'@' {
                    continue;
                }

                for c in CARDINALS {
                    let c_x = x as isize + c.0;
                    let c_y = y as isize + c.1;
                    if (c_x < 0 || c_x >= self.paper.len() as isize)
                        || (c_y < 0 || c_y >= row.len() as isize)
                    {
                        continue;
                    }

                    if self.paper[c_x as usize][c_y as usize] == b'@' {
                        count += 1;
                    }
                }

                if count < MAX_ROLLS {
                    total += 1;
                }
            }
        }

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let mut total = 0;
        let total_len = self.paper.len();

        loop {
            let mut removed = 0;
            let mut marked = Vec::new();
            for (x, row) in self.paper.iter().enumerate() {
                let row_len = row.len();
                for (y, col) in row.iter().enumerate() {
                    let mut count = 0;
                    if *col != b'@' {
                        continue;
                    }

                    for c in CARDINALS {
                        let c_x = x as isize + c.0;
                        let c_y = y as isize + c.1;
                        if (c_x < 0 || c_x >= total_len as isize)
                            || (c_y < 0 || c_y >= row_len as isize)
                        {
                            continue;
                        }

                        let target = self.paper[c_x as usize][c_y as usize];
                        if target == b'@' {
                            count += 1;
                        }
                    }

                    if count < MAX_ROLLS {
                        total += 1;
                        removed += 1;
                        marked.push((x, y));
                    }
                }
            }

            for (x, y) in marked {
                self.paper[x][y] = b'.';
            }

            if removed == 0 {
                break;
            }
        }

        println!("Part 2: {total}");
    }
}
