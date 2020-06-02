use crate::Error;
use crate::Result;
use itertools::Itertools;
use std::str::FromStr;

const SIZE: i32 = 375;

pub(crate) fn run(input: &String) -> Result<()> {
    let coordinates = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Coordinate>>>()?;

    part1(&coordinates)?;
    part2(&coordinates)?;
    Ok(())
}

fn part1(coordinates: &Vec<Coordinate>) -> Result<()> {
    // Part 1: find size of largest finite area nearest to a single point.
    let mut area_sizes = vec![0; coordinates.len()];

    for p in (0..SIZE).cartesian_product(0..SIZE) {
        // For every grid point, find distances to all points, and their minimum.
        let dists = coordinates.iter().map(|&pc| pc.manhattan_distance(Coordinate::from_point(p) )).collect_vec();
        let min_dist = dists.iter().min().unwrap();
        // Only look at points without a tie for minimum.
        if let Some(((i, _),)) = dists.iter().enumerate()
                                             .filter(|j| j.1 == min_dist).collect_tuple() {
            // If area is at the edge...
            if p.0 == 0 || p.0 == SIZE-1 || p.1 == 0 || p.1 == SIZE-1 {
                // ... remove it from consideration, since it is infinite.
                area_sizes[i] = i32::min_value();
            } else {
                area_sizes[i] += 1;
            }
        }
    }

    let max_area_size = area_sizes.into_iter().max().unwrap();
    println!("Max area: {}", max_area_size);
    Ok(())
}

fn part2(coordinates: &Vec<Coordinate>) -> Result<()> {
    let region_size = (0..SIZE).cartesian_product(0..SIZE)
        .map(|p| coordinates.iter().map(|&pc| pc.manhattan_distance(Coordinate::from_point(p))).sum::<i32>())
        .filter(|&i| i < 10000)
        .count();

    println!("Max region: {}", region_size);
    Ok(())
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance(self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn from_point(p: (i32, i32)) -> Self {
        Coordinate { x: p.0, y: p.1 }
    }
}

impl FromStr for Coordinate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Coordinate> {
        let comma_index = s.find(',').unwrap();
        let x: i32 = s[0..comma_index].trim().parse()?;
        let y: i32 = s[comma_index + 1..].trim().parse()?;
        Ok(Coordinate { x, y })
    }
}