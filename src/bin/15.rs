use std::{
    collections::HashMap,
    io::{self, Read},
};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<isize> {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let mut map = parse_map(input_parts[0]);
    let path = input_parts[1];
    path.chars().for_each(|c| {
        if c != '\n' {
            step_map(&mut map, c);
        }
    });
    Some(compute_score(map))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn pause() {
    let mut stdin = io::stdin();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn compute_score(map: Map) -> isize {
    map.map
        .into_iter()
        .filter(|(_, o)| *o == Object::Crate_Left)
        .map(|((x, y), _)| 100 * y + x)
        .sum()
}

pub fn print_map(map_data: Map) -> String {
    let mut map: String = String::new();
    (0..map_data.size.1).for_each(|y| {
        (0..2 * map_data.size.0).for_each(|x| {
            if map_data.robot_pos == (x, y) {
                map.push('@');
            } else {
                map.push(match *map_data.map.get(&(x, y)).unwrap() {
                    Object::Wall => '#',
                    Object::Empty => '.',
                    Object::Crate_Left => '[',
                    Object::Crate_Right => ']',
                });
            }
        });
        map.push('\n');
    });
    map
}

pub fn step_map(map: &mut Map, step: char) -> &mut Map {
    let new_pos = move_pos(map.robot_pos, step);
    match *map.map.get(&new_pos).unwrap() {
        Object::Wall => map,
        Object::Empty => {
            map.robot_pos = new_pos;
            map
        }
        Object::Crate_Left => {
            if move_crate(map, step, new_pos, true) {
                map.robot_pos = new_pos;
                map
            } else {
                map
            }
        }
        Object::Crate_Right => {
            if move_crate(map, step, (new_pos.0 - 1, new_pos.1), true) {
                map.robot_pos = new_pos;
                map
            } else {
                map
            }
        }
    }
}

pub fn compute_second_pos(pos: (isize, isize)) -> (isize, isize) {
    (pos.0 + 1, pos.1)
}

pub fn move_crate(map: &mut Map, step: char, current_pos: (isize, isize), do_it: bool) -> bool {
    let new_crate_pos = move_pos(current_pos, step);

    let (c1, c2) = (
        *map.map.get(&new_crate_pos).unwrap(),
        *map.map.get(&compute_second_pos(new_crate_pos)).unwrap(),
    );
    if (step == '<') {
        match c1 {
            Object::Wall => false,
            Object::Empty => {
                if (do_it) {
                    do_move_crate(map, new_crate_pos, current_pos);
                }
                true
            }
            Object::Crate_Left => panic!("Not possible"),
            Object::Crate_Right => {
                if (move_crate(map, step, (new_crate_pos.0 - 1, new_crate_pos.1), do_it)) {
                    if (do_it) {
                        do_move_crate(map, new_crate_pos, current_pos);
                    }
                    true
                } else {
                    false
                }
            }
        }
    } else if (step == '>') {
        match c2 {
            Object::Wall => false,
            Object::Empty => {
                do_move_crate(map, new_crate_pos, current_pos);
                true
            }
            Object::Crate_Right => panic!("Not possible"),
            Object::Crate_Left => {
                if (move_crate(map, step, compute_second_pos(new_crate_pos), do_it)) {
                    if (do_it) {
                        do_move_crate(map, new_crate_pos, current_pos);
                    }
                    true
                } else {
                    false
                }
            }
        }
    } else {
        match (c1, c2) {
            (_, Object::Wall) | (Object::Wall, _) => false,
            (Object::Empty, Object::Empty) => {
                if (do_it) {
                    do_move_crate(map, new_crate_pos, current_pos);
                }
                true
            }
            (Object::Crate_Left, Object::Crate_Right) => {
                if (move_crate(map, step, new_crate_pos, do_it)) {
                    if (do_it) {
                        do_move_crate(map, new_crate_pos, current_pos);
                    }
                    true
                } else {
                    false
                }
            }
            (Object::Empty, Object::Crate_Left) => {
                if move_crate(map, step, compute_second_pos(new_crate_pos), do_it) {
                    if (do_it) {
                        do_move_crate(map, new_crate_pos, current_pos);
                    }
                    true
                } else {
                    false
                }
            }
            (Object::Crate_Right, Object::Empty) => {
                if move_crate(map, step, (new_crate_pos.0 - 1, new_crate_pos.1), do_it) {
                    if (do_it) {
                        do_move_crate(map, new_crate_pos, current_pos);
                    }
                    true
                } else {
                    false
                }
            }
            (Object::Crate_Right, Object::Crate_Left) => {
                if move_crate(map, step, (new_crate_pos.0 - 1, new_crate_pos.1), false)
                    && move_crate(map, step, compute_second_pos(new_crate_pos), false)
                {
                    if (do_it) {
                        move_crate(map, step, (new_crate_pos.0 - 1, new_crate_pos.1), do_it);
                        move_crate(map, step, compute_second_pos(new_crate_pos), do_it);
                        do_move_crate(map, new_crate_pos, current_pos);
                    }
                    true
                } else {
                    false
                }
            }
            _ => panic!(),
        }
    }
}

fn do_move_crate(
    map: &mut Map,
    new_crate_pos: (isize, isize),
    current_pos: (isize, isize),
) -> bool {
    map.map.insert(current_pos, Object::Empty);
    map.map
        .insert(compute_second_pos(current_pos), Object::Empty);
    map.map.insert(new_crate_pos, Object::Crate_Left);
    map.map
        .insert(compute_second_pos(new_crate_pos), Object::Crate_Right);
    true
}

pub fn move_pos(pos: (isize, isize), dir: char) -> (isize, isize) {
    match dir {
        '^' => (pos.0, pos.1 - 1),
        '>' => (pos.0 + 1, pos.1),
        '<' => (pos.0 - 1, pos.1),
        'v' => (pos.0, pos.1 + 1),
        _ => panic!("Not a valid dir"),
    }
}

pub fn parse_map(input: &str) -> Map {
    let mut map: Map = Map {
        map: HashMap::new(),
        robot_pos: (0, 0),
        size: (0, 0),
    };
    input.lines().enumerate().for_each(|(y, line)| {
        map.size.1 += 1;
        map.size.0 = line.len() as isize;
        line.chars().enumerate().for_each(|(x, c)| {
            let (x, y) = (x as isize, y as isize);
            if c == '@' {
                map.robot_pos = (2 * x, y);
            }
            match c {
                '#' => {
                    map.map.insert((2 * x, y), Object::Wall);
                    map.map.insert((2 * x + 1, y), Object::Wall);
                }
                'O' => {
                    map.map.insert((2 * x, y), Object::Crate_Left);
                    map.map.insert((2 * x + 1, y), Object::Crate_Right);
                }
                _ => {
                    map.map.insert((2 * x, y), Object::Empty);
                    map.map.insert((2 * x + 1, y), Object::Empty);
                }
            };
        });
    });

    map
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Object {
    Wall,
    Empty,
    Crate_Left,
    Crate_Right,
}

#[derive(Clone)]
pub struct Map {
    robot_pos: (isize, isize),
    map: HashMap<(isize, isize), Object>,
    size: (isize, isize),
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
