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

        self.ranges.sort_by_key(|r| r.0);
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

    fn part2(&mut self) {
        let mut output = Vec::new();
        let (mut start, mut end) = self.ranges[0];

        for &(n_start, n_end) in self.ranges.iter().skip(1) {
            if n_start <= end + 1 {
                end = end.max(n_end);
            } else {
                output.push((start, end));
                (start, end) = (n_start, n_end)
            }
        }

        output.push((start, end));
        let total = output
            .into_iter()
            .map(|(s, e)| s.abs_diff(e) + 1)
            .sum::<usize>();

        println!("Part 2: {total}");
    }
}
