use super::Solve;

const DEBUG: bool = false;

struct Point {
    x: usize,
    y: usize,
}

// TODO: Remove once done
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x={}, y={})", self.x, self.y)
    }
}

// TODO: Remove once done
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x={}, y={})", self.x, self.y)
    }
}

impl Point {
    fn area(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[derive(Default)]
pub struct Solution {
    points: Vec<Point>,
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
            "./inputs/day9_debug.txt"
        } else {
            "./inputs/day9.txt"
        };

        let content = std::fs::read_to_string(path).expect("it exists");
        self.points = content
            .trim()
            .split('\n')
            .map(|line| {
                let (y, x) = line.split_once(',').unwrap();
                Point {
                    x: x.parse::<usize>().unwrap(),
                    y: y.parse::<usize>().unwrap(),
                }
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut max_area = usize::MIN;
        for (i, point) in self.points.iter().enumerate() {
            for other in self.points[i + 1..].iter() {
                let area = point.area(other);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        println!("Max Area: {max_area}");
    }

    fn part2(&mut self) {}
}
