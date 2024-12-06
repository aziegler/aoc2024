use std::{
    collections::HashSet,
    thread::{self, JoinHandle},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let game = parse(input);
    let (_, pos) = compute_game(game);
    return Some(pos.len());
}

pub fn part_two(input: &str) -> Option<i32> {
    let game = parse(input);
    let original_game = game.clone();
    let (_, pos) = compute_game(original_game);

    let count = pos
        .par_iter()
        .map(|pos| test_obstacle(game.clone(), *pos))
        .sum();

    Some(count)
}

fn test_obstacle(mut game: BoardState, obstacle_pos: (usize, usize)) -> i32 {
    game.extra_obstacle = Some((obstacle_pos.0 as i32, obstacle_pos.1 as i32));
    match compute_game(game) {
        (StepResult::LOOP, _) => return 1,
        _ => return 0,
    }
}

fn compute_game(mut new_game: BoardState) -> (StepResult, HashSet<(usize, usize)>) {
    loop {
        let step = new_game.step_board();
        match step {
            StepResult::CONTINUE => {
                continue;
            }
            _ => {
                return (step, new_game.distinct_pos());
            }
        }
    }
}

#[derive(Clone)]
struct BoardState {
    board: Vec<Vec<bool>>,
    board_dim: (usize, usize),
    current_pos: (usize, usize),
    current_dir: (i32, i32),
    visited_pos: HashSet<((usize, usize), (i32, i32))>,
    extra_obstacle: Option<(i32, i32)>,
}

enum StepResult {
    LOOP,
    EXIT,
    CONTINUE,
}

impl BoardState {
    fn distinct_pos(&mut self) -> HashSet<(usize, usize)> {
        self.visited_pos.iter().map(|c| c.0).collect()
    }

    fn step_board(&mut self) -> StepResult {
        if self
            .visited_pos
            .contains(&(self.current_pos, self.current_dir))
        {
            return StepResult::LOOP;
        }

        self.visited_pos
            .insert((self.current_pos, self.current_dir));
        let next_pos = (
            (self.current_pos.0 as i32 + self.current_dir.0),
            (self.current_pos.1 as i32 + self.current_dir.1),
        );

        if next_pos.0 >= self.board_dim.0 as i32
            || next_pos.1 >= self.board_dim.1 as i32
            || next_pos.0 < 0
            || next_pos.1 < 0
        {
            return StepResult::EXIT;
        }

        if !self
            .board
            .get(next_pos.1 as usize)
            .unwrap()
            .get(next_pos.0 as usize)
            .unwrap()
            || self.extra_obstacle.is_some_and(|c| c == next_pos)
        {
            let next_dir = match self.current_dir {
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                (1, 0) => (0, 1),
                _ => panic!("Impossible dir"),
            };
            self.current_dir = next_dir;
            return StepResult::CONTINUE;
        }

        self.current_pos = (next_pos.0 as usize, next_pos.1 as usize);
        StepResult::CONTINUE
    }
}

fn parse(input: &str) -> BoardState {
    let board: Vec<Vec<bool>> = input
        .lines()
        .map(|f| f.chars().map(|c| c != '#').collect())
        .collect();
    let guard = input
        .lines()
        .enumerate()
        .filter_map(|l| {
            l.1.chars()
                .enumerate()
                .filter_map(|c| match c.1 {
                    '^' => Some(((c.0, l.0), (0, -1))),
                    '<' => Some(((c.0, l.0), (-1, 0))),
                    'v' => Some(((c.0, l.0), (0, 1))),
                    '>' => Some(((c.0, l.0), (1, 0))),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();
    let size = (board.get(0).unwrap().len(), board.len());

    BoardState {
        board: board,
        board_dim: size,
        current_pos: guard.0,
        current_dir: guard.1,
        visited_pos: HashSet::new(),
        extra_obstacle: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
