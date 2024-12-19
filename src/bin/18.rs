use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let (_, obstacles) = parse_obstacles(input).unwrap();
    let max = Position { x: 70, y: 70 };
    let pos = Position { x: 0, y: 0 };
    let mut distances = HashMap::new();
    distances.insert(pos, 0);
    let result = find_exit(
        pos,
        obstacles[0..1024].to_vec(),
        &mut distances,
        VecDeque::new(),
        max,
    );
    print_map(distances, obstacles[0..1024].to_vec(), max);
    result
}

pub fn print_map(
    distances: HashMap<Position, usize>,
    obstacles: Vec<(usize, usize)>,
    max: Position,
) {
    let mut map = String::new();
    for y in 0..(max.y + 1) {
        for x in 0..(max.x + 1) {
            if obstacles.contains(&(x, y)) {
                map.push_str("#");
            } else {
                let char = distances
                    .get(&Position { x: x, y: y })
                    .map_or_else(|| ".".to_string(), |n| format!("{:1}", n % 10));
                map.push_str(&char);
            }
        }
        map.push('\n');
    }
    println!("{}", map);
}

pub fn find_exit(
    pos: Position,
    obstacles: Vec<(usize, usize)>,
    distances: &mut HashMap<Position, usize>,
    mut next: VecDeque<Position>,
    max: Position,
) -> Option<usize> {
    let distance = *distances.get(&pos).unwrap();
    if pos == max {
        let next_pos = next.pop_front();
        if next_pos.is_none() {
            return Some(distance);
        }
        return find_exit(next_pos.unwrap(), obstacles, distances, next, max);
    }
    neighbors(pos, max).iter().for_each(|f| {
        if obstacles.contains(&(f.x, f.y)) {
            return;
        }
        if distances.get(f).is_some() {
            return;
        }
        distances.insert(*f, distance + 1);
        next.push_back(*f);
    });
    let next_pos = next.pop_front();
    if next_pos.is_none() {
        return distances.get(&max).map_or(None, |f| Some(*f));
    }
    return find_exit(next_pos.unwrap(), obstacles, distances, next, max);
}

pub fn neighbors(pos: Position, max: Position) -> Vec<Position> {
    let mut neigh = Vec::new();
    if pos.x > 0 {
        neigh.push(Position {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.x < max.x {
        neigh.push(Position {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        neigh.push(Position {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.y < max.y {
        neigh.push(Position {
            x: pos.x,
            y: pos.y + 1,
        })
    }
    neigh
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, obstacles) = parse_obstacles(input).unwrap();
    let max = Position { x: 70, y: 70 };
    let pos = Position { x: 0, y: 0 };

    let mut val = None;
    for i in (1024..obstacles.len()) {
        let mut distances = HashMap::new();
        distances.insert(pos, 0);
        let result = find_exit(
            pos,
            obstacles[0..i].to_vec(),
            &mut distances,
            VecDeque::new(),
            max,
        );
        if result.is_none() {
            val = Some(*obstacles.get(i - 1).unwrap());
            println!("Solution : {:?}", val);
            break;
        }
    }
    Some(1)
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return (self.x + self.y).cmp(&(other.x + other.y));
    }
}

pub fn parse_obstacles(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    many1(terminated(
        separated_pair(number, tag(","), number),
        opt(newline),
    ))(input)
}

pub fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |c: &str| c.parse::<usize>())(input)
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
