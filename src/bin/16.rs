use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<isize> {
    let mut start_pos: (isize, isize) = (0, 0);
    let mut end_pos: (isize, isize) = (0, 0);
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
    Some(find_path(
        maze,
        Deer {
            direction: Direction::EAST,
            pos: start_pos,
        },
        end_pos,
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn find_path(maze: Vec<Vec<bool>>, mut deer_pos: Deer, end_pos: (isize, isize)) -> isize {
    let mut visited: HashSet<Deer> = HashSet::new();
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let dist: &mut HashMap<Deer, isize> = &mut HashMap::<Deer, isize>::new();
    dist.insert(deer_pos, 0);

    loop {
        let distance = dist.get(&deer_pos).unwrap().clone();
        if deer_pos.pos == end_pos {
            return distance;
        }
        visited.insert(deer_pos);
        for (deer, cost) in compute_neighbors(deer_pos) {
            let wall = *maze
                .get(deer.pos.1 as usize)
                .unwrap()
                .get(deer.pos.0 as usize)
                .unwrap();
            if wall {
                continue;
            }
            if visited.contains(&deer) {
                continue;
            }
            let deer_dist = dist.get(&deer);
            if deer_dist.is_none()
                || (deer_dist.is_some() && deer_dist.unwrap() > &(distance + cost))
            {
                dist.insert(deer, distance + cost);
            }

            frontier.push(Node {
                cost: distance + cost,
                deer: deer,
            });
        }
        deer_pos = frontier.pop().unwrap().deer;
    }
}

pub fn compute_neighbors(deer: Deer) -> Vec<(Deer, isize)> {
    let mut neigh = Vec::new();
    match deer.direction {
        Direction::EAST => neigh.push((
            Deer {
                direction: Direction::EAST,
                pos: (deer.pos.0 + 1, deer.pos.1),
            },
            1,
        )),
        Direction::WEST => neigh.push((
            Deer {
                direction: Direction::WEST,
                pos: (deer.pos.0 - 1, deer.pos.1),
            },
            1,
        )),
        Direction::NORTH => neigh.push((
            Deer {
                direction: Direction::NORTH,
                pos: (deer.pos.0, deer.pos.1 - 1),
            },
            1,
        )),
        Direction::SOUTH => neigh.push((
            Deer {
                direction: Direction::SOUTH,
                pos: (deer.pos.0, deer.pos.1 + 1),
            },
            1,
        )),
    }
    match deer.direction {
        Direction::EAST => neigh.push((
            Deer {
                direction: Direction::NORTH,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
        Direction::WEST => neigh.push((
            Deer {
                direction: Direction::NORTH,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
        Direction::NORTH => neigh.push((
            Deer {
                direction: Direction::EAST,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
        Direction::SOUTH => neigh.push((
            Deer {
                direction: Direction::EAST,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
    }
    match deer.direction {
        Direction::EAST => neigh.push((
            Deer {
                direction: Direction::SOUTH,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
        Direction::WEST => neigh.push((
            Deer {
                direction: Direction::SOUTH,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
        Direction::NORTH => neigh.push((
            Deer {
                direction: Direction::WEST,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
        Direction::SOUTH => neigh.push((
            Deer {
                direction: Direction::WEST,
                pos: (deer.pos.0, deer.pos.1),
            },
            1000,
        )),
    }
    neigh
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    cost: isize,
    deer: Deer,
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
pub struct Deer {
    direction: Direction,
    pos: (isize, isize),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    EAST,
    WEST,
    NORTH,
    SOUTH,
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
