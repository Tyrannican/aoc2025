use super::Solve;

#[derive(Default)]
pub struct Solution {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
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
        let content = std::fs::read_to_string("./inputs/day5.txt").expect("it exists");
        let (ranges, ids) = content
            .split_once("\n\n")
            .expect("guaranteed to have two newlines");

        self.ranges = ranges
            .trim()
            .split('\n')
            .map(|range| {
                let (start, end) = range.split_once('-').expect("all have a -");
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            })
            .collect();

        self.ids = ids
            .trim()
            .split('\n')
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        for id in self.ids.iter() {
            for (start, end) in self.ranges.iter() {
                if id >= start && id <= end {
                    total += 1;
                    break;
                }
            }
        }

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {}
}
