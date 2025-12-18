use super::Solve;

const DEBUG: bool = false;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

#[derive(Default, Debug)]
pub struct Solution {
    shapes: Vec<usize>,
    trees: Vec<Region>,
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
            "./inputs/day12_debug.txt"
        } else {
            "./inputs/day12.txt"
        };

        let mut process_present = |present: &str| {
            let count = present
                .split("\n")
                .map(|line| line.as_bytes().iter().filter(|&c| *c == b'#').count())
                .sum::<usize>();
            self.shapes.push(count);
        };

        let mut process_trees = |trees: &str| {
            let _: Vec<_> = trees
                .split('\n')
                .map(|row| {
                    let (dims, presents) = row.split_once(": ").unwrap();
                    let (width, height) = dims.split_once("x").unwrap();
                    self.trees.push(Region {
                        width: width.parse::<usize>().unwrap(),
                        height: height.parse::<usize>().unwrap(),
                        presents: presents
                            .split(' ')
                            .map(|p| p.parse::<usize>().unwrap())
                            .collect(),
                    });
                })
                .collect();
        };
        let content = std::fs::read_to_string(path).expect("it exists");
        let _: Vec<_> = content
            .trim()
            .split("\n\n")
            .map(|section| {
                if section.contains("x") {
                    process_trees(section);
                } else {
                    process_present(section);
                }
            })
            .collect();
    }

    fn part1(&mut self) {
        let result = self.trees.iter().fold(0, |acc, tree| {
            let area = tree.width * tree.height;
            let max_area = tree
                .presents
                .iter()
                .fold(0, |fits, present_count| fits + (present_count * 9));

            if area >= max_area { acc + 1 } else { acc }
        });

        println!("Part 1: {result}")
    }

    fn part2(&mut self) {
        println!("Part 2: The North Pole is decorated!");
    }
}
