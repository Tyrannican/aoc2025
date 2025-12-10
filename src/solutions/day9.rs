use super::Solve;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    #[inline]
    fn area(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Edge {
    pt1: Point,
    pt2: Point,
}

impl Edge {
    fn new(pt1: Point, pt2: Point) -> Self {
        Self { pt1, pt2 }
    }

    #[inline]
    fn on(&self, pt: &Point) -> bool {
        let (p1, p2) = (self.pt1, self.pt2);

        let dx = pt.x as isize - p1.x as isize;
        let dy = pt.y as isize - p1.y as isize;
        let dxe = p2.x as isize - p1.x as isize;
        let dye = p2.y as isize - p1.y as isize;

        let cross_product = (dx * dye) - (dy * dxe);
        if cross_product != 0 {
            false
        } else {
            if dxe.abs() > dye.abs() {
                if dxe > 0 {
                    p1.x <= pt.x && pt.x <= p2.x
                } else {
                    p2.x <= pt.x && pt.x <= p1.x
                }
            } else {
                if dye > 0 {
                    p1.y <= pt.y && pt.y <= p2.y
                } else {
                    p2.y <= pt.y && pt.y <= p1.y
                }
            }
        }
    }

    #[inline]
    fn intersects(&self, pt: &Point) -> bool {
        if *pt == self.pt1 || *pt == self.pt2 {
            return true;
        }

        let mut pt = pt.clone();
        let (mut a, mut b) = (self.pt1, self.pt2);

        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }

        if pt.y == a.y || pt.y == b.y {
            pt.y = pt.y.saturating_add(1);
        }

        if (pt.y > b.y || pt.y < a.y) || pt.x > a.x.max(b.x) {
            false
        } else if pt.x < a.x.min(b.x) {
            true
        } else {
            let m_red = if ((a.x as isize - b.x as isize).abs()) as usize > usize::MIN {
                (b.y - a.y) / (b.x - a.x)
            } else {
                usize::MAX
            };

            let m_blue = if ((a.x as isize - b.x as isize).abs()) as usize > usize::MIN {
                (pt.y - a.y) / (pt.x - a.x)
            } else {
                usize::MAX
            };

            m_blue >= m_red
        }
    }
}

struct Polygon {
    edges: Vec<Edge>,
}

impl Polygon {
    fn new(points: &[Point]) -> Self {
        let mut edges: Vec<Edge> = points
            .windows(2)
            .map(|w| {
                let (pt1, pt2) = (w[0], w[1]);
                Edge::new(pt1, pt2)
            })
            .collect();

        edges.push(Edge {
            pt1: points[points.len() - 1],
            pt2: points[0],
        });

        Self { edges }
    }

    #[inline]
    fn contains(&self, pt: &Point) -> bool {
        if self.edges.iter().any(|edge| edge.on(pt)) {
            true
        } else {
            let count = self.edges.iter().filter(|edge| edge.intersects(pt)).count();
            count % 2 == 1
        }
    }

    #[inline]
    fn check_edge(&self, edge: &Edge) -> bool {
        let (p1, p2) = (edge.pt1, edge.pt2);
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        if min_x == max_x {
            for y in (min_y..=max_y).step_by(1000) {
                let check = Point { x: min_x, y };
                if !self.contains(&check) {
                    return false;
                }
            }
        } else {
            assert!(min_y == max_y);
            for x in (min_x..=max_x).step_by(1000) {
                let check = Point { x, y: min_y };
                if !self.contains(&check) {
                    return false;
                }
            }
        }
        true
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
        let content = std::fs::read_to_string("./inputs/day9.txt").expect("it exists");
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

        println!("Part 1: {max_area}");
    }

    fn part2(&mut self) {
        let mut max_area = usize::MIN;
        let polygon = Polygon::new(&self.points);

        for (i, pt1) in self.points.iter().enumerate() {
            for pt2 in self.points[i + 1..].iter() {
                let pt3 = Point { x: pt1.x, y: pt2.y };
                let pt4 = Point { x: pt2.x, y: pt1.y };
                if polygon.contains(&pt3) && polygon.contains(&pt4) {
                    let edges = [
                        Edge::new(*pt1, pt3),
                        Edge::new(pt3, *pt2),
                        Edge::new(*pt2, pt4),
                        Edge::new(pt4, *pt1),
                    ];

                    if edges.iter().all(|edge| polygon.check_edge(edge)) {
                        let area = pt1.area(pt2);
                        if area > max_area {
                            max_area = area;
                        }
                    }
                }
            }
        }

        println!("Part 2: {max_area}");
    }
}
