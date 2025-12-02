use super::Solve;

#[derive(Default)]
pub struct Solution {
    ids: Vec<(usize, usize)>,
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
        let content = std::fs::read_to_string("./inputs/day2.txt").expect("it's a text file...");
        self.ids = content
            .trim()
            .split(',')
            .map(|id_range| {
                let (s, e) = id_range
                    .split_once('-')
                    .expect("ids are structured like this");

                (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        for (start, end) in self.ids.iter() {
            for i in *start..=*end {
                let b = i.to_string();
                let (first, last) = b.as_bytes().split_at(b.len() / 2);
                if first == last {
                    total += i;
                }
            }
        }

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {}
}
