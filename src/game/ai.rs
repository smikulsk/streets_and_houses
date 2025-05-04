use std::fmt::Debug;

use super::*;

pub type RowType = usize;
pub type ColType = usize;
pub type CounterType = usize;

pub trait MoveGenerator : Debug {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)>;
}

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

#[derive(Default, Debug)]
pub struct GreadyAlgorithmPlayer {}

impl MoveGenerator for GreadyAlgorithmPlayer {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)> {
        if board.all_is_clicked() {
            return None;
        }
        // look for wall adjacent to cell with max counter = 3
        // if none found look for wall adjacent to cell with max counter = 0 or 1
        // if none found take first with max counter = 2
        let mut walls = vec![];

        for (row, row_walls) in board.walls.iter().enumerate() {
            for (col, wall) in row_walls.iter().enumerate() {
                if wall.is_clicked {
                    continue;
                }
                let max_counter = wall
                    .adjacent_cells
                    .iter()
                    .map(|(r, c)| board.cells[*r][*c].counter)
                    .max()
                    .expect("at least one cell is adjacent to the wall");

                walls.push(WallStatistics::new(row, col, max_counter));
            }
        }
        if let Some(wall_next_to_almost_closed_cell) =
            walls.iter().find(|&ws| ws.max_adjacent_counter == 3)
        {
            return Some((
                wall_next_to_almost_closed_cell.row,
                wall_next_to_almost_closed_cell.col,
            ));
        }

        if let Some(wall_next_to_empty_cell) = walls.iter().find(|&ws| ws.max_adjacent_counter <= 1)
        {
            return Some((wall_next_to_empty_cell.row, wall_next_to_empty_cell.col));
        }

        if let Some(wall) = walls.iter().find(|&ws| ws.max_adjacent_counter == 2) {
            return Some((wall.row, wall.col));
        }

        unreachable!("We should fill all the cases by now!!!")
    }
}

#[cfg(test)]
mod gready_algorithm_player_tests {
    use super::*;

    #[test]
    fn when_all_is_clicked_there_is_no_other_move() {
        // arrange
        let mut board = Board::new(1, 1);
        let player = Player::Player1;
        let _ = board.click_wall(0, 0, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 1, player);
        let _ = board.click_wall(2, 0, player);

        assert!(board.all_is_clicked());

        let gready_algorithm_player = GreadyAlgorithmPlayer::default();

        // act
        let next_move = gready_algorithm_player.next_move(&board);

        // assert
        assert!(next_move.is_none());
    }

    #[test]
    fn first_take_wall_next_to_almost_closed_cell() {
        // arrange
        let mut board = Board::new(1, 2);
        let player = Player::Player1;
        let _ = board.click_wall(0, 0, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 1, player);

        let gready_algorithm_player = GreadyAlgorithmPlayer::default();

        // act
        let next_move = gready_algorithm_player.next_move(&board);

        // assert
        assert_eq!(next_move, Some((2, 0)));
    }

    #[test]
    fn when_no_wall_next_to_almost_closed_cell_take_the_one_with_smallest_counter() {
        // arrange
        let mut board = Board::new(2, 2);
        let player = Player::Player1;
        let _ = board.click_wall(0, 0, player);
        let _ = board.click_wall(1, 0, player);

        let gready_algorithm_player = GreadyAlgorithmPlayer::default();

        // act
        let mut next_move = gready_algorithm_player.next_move(&board);

        // assert
        assert_eq!(next_move, Some((0, 1))); // wall with counter 0

        if let Some((row, col)) = next_move {
            let _ = board.click_wall(row, col, player);

            next_move = gready_algorithm_player.next_move(&board);

            // assert
            assert_eq!(next_move, Some((1, 2))); // wall with counter 1
        }
    }

    #[test]
    fn when_no_other_option_take_wall_with_counter_2() {
        // arrange
        let mut board = Board::new(1, 2);
        let player = Player::Player1;
        let _ = board.click_wall(0, 0, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 1, player);
        let _ = board.click_wall(2, 0, player);
        let _ = board.click_wall(4, 0, player);

        let gready_algorithm_player = GreadyAlgorithmPlayer::default();

        // act
        let next_move = gready_algorithm_player.next_move(&board);

        // assert
        assert_eq!(next_move, Some((3, 0)));
    }
}
