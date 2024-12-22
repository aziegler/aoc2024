use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let mut start_pos: (isize, isize) = (0, 0);
    let mut end_pos: (isize, isize) = (0, 0);
    let wall_dist: &mut HashMap<Point, isize> = &mut HashMap::<Point, isize>::new();
    let maze: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(y, c)| {
            c.chars()
                .enumerate()
                .map(|(x, ch)| match (ch) {
                    '#' => true,
                    'S' => {
                        start_pos = (x.clone() as isize, y.clone() as isize);
                        false
                    }
                    'E' => {
                        end_pos = (x.clone() as isize, y.clone() as isize);
                        false
                    }
                    _ => false,
                })
                .collect()
        })
        .collect();
    let initial_path = find_path(
        &maze,
        Point { pos: start_pos },
        end_pos,
        wall_dist,
        None,
        true,
    );
    let possible_walls: Vec<(&Point, &isize)> = wall_dist
        .iter()
        .filter(|(p, dist)| {
            **dist < initial_path
                && p.pos.0 > 0
                && p.pos.1 > 0
                && ((p.pos.1 as usize) < maze.len())
                && ((p.pos.0 as usize) < maze.get(0).unwrap().len())
        })
        .collect();
    let possible_deletes: Vec<Vec<&(&Point, &isize)>> =
        possible_walls.iter().permutations(2).collect();
    let tests: Vec<Option<isize>> = possible_deletes
        .iter()
        .map(|todelete| {
            let new_maze = &mut maze.clone();
            let mut line = new_maze.get(todelete[0].0.pos.1 as usize).unwrap().clone();
            line.remove(todelete[0].0.pos.0 as usize);
            line.insert(todelete[0].0.pos.0 as usize, false);
            new_maze.insert(todelete[0].0.pos.1 as usize, line);
            let mut other_line = new_maze.get(todelete[1].0.pos.1 as usize).unwrap().clone();
            other_line.remove(todelete[1].0.pos.0 as usize);
            other_line.insert(todelete[1].0.pos.0 as usize, false);
            new_maze.insert(todelete[1].0.pos.1 as usize, other_line);
            let walls: usize = new_maze.iter().flat_map(|c| c).filter(|d| **d).count();
            let init_walls: usize = maze.iter().flat_map(|c| c).filter(|d| **d).count();
            println!("New maze {}, init {}", walls, init_walls);
            let new_path = find_path(
                &new_maze,
                Point { pos: start_pos },
                end_pos,
                &mut HashMap::new(),
                Some(initial_path),
                false,
            );
            if new_path > 0 {
                Some(initial_path - new_path)
            } else {
                None
            }
        })
        .collect();
    Some(
        tests
            .iter()
            .filter(|c| c.is_some() && c.unwrap() > 99)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn find_path(
    maze: &Vec<Vec<bool>>,
    mut pos: Point,
    end_pos: (isize, isize),
    wall_dist: &mut HashMap<Point, isize>,
    max_dist: Option<isize>,
    compute_walls: bool,
) -> isize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let dist: &mut HashMap<Point, isize> = &mut HashMap::<Point, isize>::new();

    dist.insert(pos, 0);

    loop {
        let distance = dist.get(&pos).unwrap().clone();
        if pos.pos == end_pos {
            return distance;
        }
        if max_dist.is_some_and(|d| d <= distance) {
            return -1;
        }
        visited.insert(pos);
        for neighbour in compute_neighbors(pos) {
            let wall = *maze
                .get(neighbour.pos.1 as usize)
                .unwrap()
                .get(neighbour.pos.0 as usize)
                .unwrap();
            if wall {
                if compute_walls {
                    let wall_distance = wall_dist.get(&neighbour);
                    if wall_distance.is_none()
                        || (wall_distance.is_some() && wall_distance.unwrap() > &(distance + 1))
                    {
                        wall_dist.insert(neighbour, distance + 1);
                    }
                }
                continue;
            }
            if visited.contains(&neighbour) {
                continue;
            }
            let neighbour_dist = dist.get(&neighbour);
            if neighbour_dist.is_none()
                || (neighbour_dist.is_some() && neighbour_dist.unwrap() > &(distance + 1))
            {
                dist.insert(neighbour, distance + 1);
            }

            frontier.push(Node {
                cost: distance + 1,
                point: neighbour,
            });
        }
        let next = frontier.pop();
        if next.is_none() {
            return -1;
        }
        pos = next.unwrap().point;
    }
}

pub fn compute_neighbors(point: Point) -> Vec<Point> {
    vec![
        Point {
            pos: (point.pos.0, point.pos.1 + 1),
        },
        Point {
            pos: (point.pos.0, point.pos.1 - 1),
        },
        Point {
            pos: (point.pos.0 - 1, point.pos.1),
        },
        Point {
            pos: (point.pos.0 + 1, point.pos.1),
        },
    ]
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    cost: isize,
    point: Point,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pos: (isize, isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
