use dyn_clone::DynClone;

use prelude::*;

use quad_rand::ChooseRandom;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::game::Board;

pub type RowType = usize;
pub type ColType = usize;
pub type CounterType = usize;

pub mod greedy_algorithm_player;
pub mod minmax_player;
pub mod prelude;
pub mod region_counting_player;

pub trait MoveGenerator: Debug + DynClone {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)>;
}

dyn_clone::clone_trait_object!(MoveGenerator);

#[derive(Debug)]
struct WallStatistics {
    row: RowType,
    col: RowType,
    max_adjacent_counter: CounterType,
}

impl WallStatistics {
    fn new(row: RowType, col: ColType, max_adjacent_counter: CounterType) -> Self {
        Self {
            row,
            col,
            max_adjacent_counter,
        }
    }
}

fn filter_walls<T>(walls: &[T], filter_condition: impl FnMut(&&T) -> bool) -> Vec<&T> {
    walls.iter().filter(filter_condition).collect::<Vec<_>>()
}

fn choose_wall_index<T>(walls: &[T], filter_condition: impl FnMut(&&T) -> bool) -> Option<&T> {
    filter_walls(walls, filter_condition)
        .as_slice()
        .choose()
        .copied()
}

fn find_region_size(board: &Board, wall: &WallStatistics) -> usize {
    let mut board = board.clone();
    let mut row = wall.row;
    let mut col = wall.col;
    let player = crate::game::Player::CPU;
    let move_generator = GreadyAlgorithmPlayer::default();

    let starting_points = board.statistics.cpu_points;
    if wall.max_adjacent_counter == 2 && board.click_wall(row, col, player).is_ok() {
        if let Some((r, c)) = move_generator.next_move(&board) {
            row = r;
            col = c;
        }
    }
    while let Ok(true) = board.click_wall(row, col, player) {
        if let Some((r, c)) = move_generator.next_move(&board) {
            row = r;
            col = c;
        }
    }
    board.statistics.cpu_points - starting_points
}

fn build_region_size_map<'a>(
    board: &Board,
    walls: &'a [WallStatistics],
    filter_condition: impl FnMut(&&WallStatistics) -> bool,
) -> HashMap<usize, Vec<&'a WallStatistics>> {
    filter_walls(walls, filter_condition)
        .iter()
        .map(|&ws| (find_region_size(board, ws), ws))
        .fold(
            HashMap::<usize, Vec<_>>::new(),
            |mut hashmap, (region_size, ws)| {
                let e = hashmap.entry(region_size).or_default();
                e.push(ws);
                hashmap
            },
        )
}

fn collect_wall_statistics(board: &Board) -> Vec<WallStatistics> {
    board
        .walls
        .iter()
        .flatten()
        .filter_map(|wall| {
            if wall.is_clicked {
                None
            } else {
                let max_counter = wall
                    .adjacent_cells
                    .iter()
                    .map(|(r, c)| board.cells[*r][*c].counter)
                    .max()
                    .expect("at least one cell is adjacent to the wall");
                Some(WallStatistics::new(wall.id.0, wall.id.1, max_counter))
            }
        })
        .collect()
}

fn wall_priority(wall: &WallStatistics) -> usize {
    match wall.max_adjacent_counter {
        3 => 0,
        0 | 1 => 1,
        2 => 2,
        _ => unreachable!("There shjould not be not clicked walls with more than 3 adjacent cells"),
    }
}

#[cfg(test)]
mod ai_functions_tests {
    use super::*;

    #[test]
    fn find_region_size_works_correctly() {
        let mut board = Board::new(3, 3);
        let player = crate::game::Player::CPU;

        let _ = board.click_wall(0, 2, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 1, player);
        let _ = board.click_wall(1, 3, player);
        let _ = board.click_wall(2, 1, player);
        let _ = board.click_wall(3, 0, player);
        let _ = board.click_wall(3, 2, player);
        let _ = board.click_wall(4, 2, player);
        let _ = board.click_wall(5, 0, player);
        let _ = board.click_wall(5, 1, player);
        let _ = board.click_wall(5, 3, player);
        let _ = board.click_wall(6, 1, player);

        let expected_region_sizes = [1, 3, 3, 1, 3, 3, 3, 1, 3, 3, 1, 3];
        let walls = [
            WallStatistics::new(0, 0, 2),
            WallStatistics::new(0, 1, 2),
            WallStatistics::new(1, 2, 2),
            WallStatistics::new(2, 0, 2),
            WallStatistics::new(2, 2, 2),
            WallStatistics::new(3, 1, 2),
            WallStatistics::new(3, 3, 2),
            WallStatistics::new(4, 0, 2),
            WallStatistics::new(4, 1, 2),
            WallStatistics::new(5, 2, 2),
            WallStatistics::new(6, 0, 2),
            WallStatistics::new(6, 2, 2),
        ];

        for (idx, wall) in walls.iter().enumerate() {
            //dbg!(&wall, &expected_region_sizes[idx]);
            assert_eq!(
                expected_region_sizes[idx],
                find_region_size(&board, wall)
            );
        }
    }

    #[test]
    fn find_region_size_works_for_almost_closed_cells() {
        let mut board = Board::new(3, 3);
        let player = crate::game::Player::CPU;

        let _ = board.click_wall(0, 0, player);
        let _ = board.click_wall(0, 2, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 1, player);
        let _ = board.click_wall(1, 3, player);
        let _ = board.click_wall(2, 1, player);
        let _ = board.click_wall(3, 0, player);
        let _ = board.click_wall(3, 2, player);
        let _ = board.click_wall(4, 2, player);
        let _ = board.click_wall(5, 0, player);
        let _ = board.click_wall(5, 1, player);
        let _ = board.click_wall(5, 3, player);
        let _ = board.click_wall(6, 1, player);

        let expected_region_size = 1;
        let wall = WallStatistics::new(2, 0, 3);

        assert_eq!(
            expected_region_size,
            find_region_size(&board, &wall)
        );
    }

    #[test]
    fn find_region_size_works_for_loop_regions() {
        let mut board = Board::new(3, 3);
        let player = crate::game::Player::CPU;

        let _ = board.click_wall(0, 0, player);
        let _ = board.click_wall(0, 1, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 2, player);
        let _ = board.click_wall(3, 0, player);
        let _ = board.click_wall(3, 2, player);
        let _ = board.click_wall(4, 0, player);
        let _ = board.click_wall(4, 1, player);

        let expected_region_size = 4;
        let wall = WallStatistics::new(1, 1, 2);

        assert_eq!(
            expected_region_size,
            find_region_size(&board, &wall)
        );
}
}