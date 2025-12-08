use std::collections::{HashMap, HashSet};

use super::Solve;

const DEBUG: bool = false;

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn dist(&self, other: &Self) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);

        ((dx * dx) + (dy * dy) + (dz * dz)).isqrt()
    }
}

// TODO: Remove when done
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x={},y={},z={})", self.x, self.y, self.z)
    }
}

// TODO: Remove when done
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x={},y={},z={})", self.x, self.y, self.z)
    }
}

#[derive(Default)]
pub struct Solution {
    junctions: Vec<((Point, Point), usize)>,
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
            "./inputs/day8_debug.txt"
        } else {
            "./inputs/day8.txt"
        };

        let content = std::fs::read_to_string(path).expect("it exists");
        let junctions: Vec<Point> = content
            .trim()
            .split("\n")
            .map(|line| {
                let mut pt = Point::default();
                let _: Vec<_> = line
                    .splitn(3, ',')
                    .enumerate()
                    .map(|(i, n)| {
                        let n = n.parse::<usize>().unwrap();
                        match i {
                            0 => pt.x = n,
                            1 => pt.y = n,
                            2 => pt.z = n,
                            _ => unreachable!(),
                        }
                    })
                    .collect();

                pt
            })
            .collect();

        self.junctions = junctions
            .iter()
            .enumerate()
            .map(|(i, pt1)| {
                junctions[i + 1..]
                    .iter()
                    .enumerate()
                    .map(|(_, pt2)| ((*pt1, *pt2), pt1.dist(pt2)))
                    .collect::<Vec<((Point, Point), usize)>>()
            })
            .flatten()
            .collect();

        self.junctions.sort_by(|a, b| a.1.cmp(&b.1));

        // TODO: Global Adj Matrix?
    }

    fn part1(&mut self) {
        let size = if DEBUG { 10 } else { 1000 };
        let mut matrix: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();
        let shortest: Vec<((Point, Point), usize)> =
            self.junctions[..size].into_iter().map(|p| *p).collect();

        for ((pt1, pt2), weight) in shortest {
            matrix
                .entry(pt1)
                .and_modify(|v| {
                    v.push((pt2, weight));
                })
                .or_insert(vec![(pt2, weight)]);
            matrix
                .entry(pt2)
                .and_modify(|v| {
                    v.push((pt1, weight));
                })
                .or_insert(vec![(pt1, weight)]);
        }

        let mut count = HashSet::new();
        let mut visited = HashSet::new();
        for k in matrix.keys() {
            if !visited.contains(k) {
                dfs(&matrix, &mut visited, *k);
            }
            count.insert(visited.len());
            visited.clear();
        }

        let mut counts = Vec::from_iter(count.into_iter());
        counts.sort_by(|a, b| b.cmp(&a));

        let total = counts[..3].iter().product::<usize>();
        println!("Part 1: {total}");
    }

    fn part2(&mut self) {}
}

fn dfs(matrix: &HashMap<Point, Vec<(Point, usize)>>, visited: &mut HashSet<Point>, pt: Point) {
    visited.insert(pt);
    let value = matrix.get(&pt).unwrap();
    for (inner_pt, _) in value.into_iter() {
        if !visited.contains(&inner_pt) {
            dfs(matrix, visited, *inner_pt);
        }
    }
}
