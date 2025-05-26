use super::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct GreadyAlgorithmPlayer {}

impl MoveGenerator for GreadyAlgorithmPlayer {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)> {
        if board.all_is_clicked() {
            return None;
        }
        let walls = collect_wall_statistics(board);

        // look for wall adjacent to cell with max counter = 3
        // if none found look for wall adjacent to cell with max counter = 0 or 1
        // if none found take random with max counter = 2
        if let Some(wall_next_to_almost_closed_cell) =
            choose_wall_index(&walls, |&ws| ws.max_adjacent_counter == 3)
        {
            return Some((
                wall_next_to_almost_closed_cell.row,
                wall_next_to_almost_closed_cell.col,
            ));
        }

        if let Some(wall_next_to_empty_cell) =
            choose_wall_index(&walls, |&ws| ws.max_adjacent_counter <= 1)
        {
            return Some((wall_next_to_empty_cell.row, wall_next_to_empty_cell.col));
        }

        if let Some(wall) = choose_wall_index(&walls, |&ws| ws.max_adjacent_counter == 2) {
            return Some((wall.row, wall.col));
        }

        unreachable!("We should fill all the cases by now!!!")
    }
}

#[cfg(test)]
mod gready_algorithm_player_tests {
    use super::*;
    use crate::game::Player;

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
        let mut valid_moves: Vec<(RowType, ColType)> = vec![
            (0, 1),
            (1, 2),
            (2, 1),
            (3, 0),
            (3, 1),
            (3, 2),
            (4, 0),
            (4, 1),
        ];

        // act
        let mut next_move = gready_algorithm_player.next_move(&board);

        // assert
        assert!(valid_moves.contains(&next_move.expect("the move is possible"))); // wall with counter 0

        if let Some((row, col)) = next_move {
            // arrange
            valid_moves.retain(|valid_move| *valid_move != (row, col));
            let _ = board.click_wall(row, col, player);

            // act
            next_move = gready_algorithm_player.next_move(&board);

            // assert
            assert!(valid_moves.contains(&next_move.expect("the move is possible")));
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

        let valid_moves: Vec<(RowType, ColType)> = vec![(3, 0), (3, 1)];

        let gready_algorithm_player = GreadyAlgorithmPlayer::default();

        // act
        let next_move = gready_algorithm_player.next_move(&board);

        // assert
        assert!(valid_moves.contains(&next_move.expect("the move is possible")));
    }
}
