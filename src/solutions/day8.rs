use std::collections::{HashMap, HashSet};

use super::Solve;

type BoxMap = HashMap<Point, Vec<Point>>;
type PointWeight = ((Point, Point), usize);

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

#[derive(Default)]
pub struct Solution {
    total: usize,
    junctions: Vec<PointWeight>,
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
        let content = std::fs::read_to_string("./inputs/day8.txt").expect("it exists");
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

        self.total = junctions.len();

        self.junctions = junctions
            .iter()
            .enumerate()
            .map(|(i, pt1)| {
                junctions[i + 1..]
                    .iter()
                    .enumerate()
                    .map(|(_, pt2)| ((*pt1, *pt2), pt1.dist(pt2)))
                    .collect::<Vec<PointWeight>>()
            })
            .flatten()
            .collect();

        self.junctions.sort_by(|a, b| a.1.cmp(&b.1));
    }

    fn part1(&mut self) {
        let size = 1000;
        let mut box_map: BoxMap = HashMap::new();

        let shortest: Vec<(Point, Point)> =
            self.junctions[..size].into_iter().map(|p| p.0).collect();

        for (pt1, pt2) in shortest {
            join(pt1, pt2, &mut box_map);
        }

        let mut circuit_len = count_circuits(&box_map);
        circuit_len.sort_by(|a, b| b.cmp(&a));
        let total = circuit_len[..3].iter().product::<usize>();

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let mut box_map: BoxMap = HashMap::new();
        'outer: for ((pt1, pt2), _) in self.junctions.iter() {
            join(*pt1, *pt2, &mut box_map);
            let mut stack = vec![*pt1, *pt2];
            let mut visited = HashSet::new();
            while !stack.is_empty() {
                let pt = stack.pop().unwrap();
                if visited.contains(&pt) {
                    continue;
                }

                visited.insert(pt);
                if visited.len() == self.total {
                    println!("Part 2: {}", pt1.x * pt2.x);
                    break 'outer;
                }

                let neighbours = box_map.get(&pt).unwrap();
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        stack.push(*neighbour);
                    }
                }
            }
        }
    }
}

fn count_circuits(box_map: &BoxMap) -> Vec<usize> {
    let mut count = HashSet::new();
    for k in box_map.keys() {
        let mut stack = vec![*k];
        let mut visited = HashSet::new();
        while !stack.is_empty() {
            let pt = stack.pop().unwrap();
            if visited.contains(&pt) {
                continue;
            }

            visited.insert(pt);
            let neighbours = box_map.get(&pt).unwrap();
            for neighbour in neighbours {
                if !visited.contains(neighbour) {
                    stack.push(*neighbour);
                }
            }
        }
        count.insert(visited.len());
    }

    Vec::from_iter(count.into_iter())
}

fn join(pt1: Point, pt2: Point, box_map: &mut BoxMap) {
    box_map
        .entry(pt1)
        .and_modify(|v| {
            v.push(pt2);
        })
        .or_insert(vec![pt2]);
    box_map
        .entry(pt2)
        .and_modify(|v| {
            v.push(pt1);
        })
        .or_insert(vec![pt1]);
}
