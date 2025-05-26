use super::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct RegionCountingPlayer {}

impl MoveGenerator for RegionCountingPlayer {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)> {
        if board.all_is_clicked() {
            return None;
        }
        let walls = collect_wall_statistics(board);

        let wall_region_size_map =
            build_region_size_map(board, &walls, |&ws| ws.max_adjacent_counter == 3);

        // look for wall adjacent to cell with max counter = 3 and choose the one which creates the biggest region
        if let Some(max_region) = wall_region_size_map.keys().max() {
            if let Some(wall_next_to_almost_closed_cell) =
                choose_wall_index(&wall_region_size_map[max_region], |&ws| {
                    ws.max_adjacent_counter == 3
                })
            {
                return Some((
                    wall_next_to_almost_closed_cell.row,
                    wall_next_to_almost_closed_cell.col,
                ));
            }
        }

        // if none found look for wall adjacent to cell with max counter = 0 or 1
        if let Some(wall_next_to_empty_cell) =
            choose_wall_index(&walls, |&ws| ws.max_adjacent_counter <= 1)
        {
            return Some((wall_next_to_empty_cell.row, wall_next_to_empty_cell.col));
        }

        // if none found take random with max counter = 2 which will create the smallest region
        let wall_region_size_map =
            build_region_size_map(board, &walls, |&ws| ws.max_adjacent_counter == 2);

        if let Some(min_region) = wall_region_size_map.keys().min() {
            if let Some(wall) = choose_wall_index(&wall_region_size_map[min_region], |&ws| {
                ws.max_adjacent_counter == 2
            }) {
                return Some((wall.row, wall.col));
            }
        }

        unreachable!("We should fill all the cases by now!!!")
    }
}

#[cfg(test)]
mod region_counting_player_tests {
    use super::*;

    #[test]
    fn next_move_will_take_wall_opening_the_smallest_region() {
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

        let expected_moves = [(0, 0), (2, 0), (4, 0), (6, 0)];

        let move_generator = RegionCountingPlayer::default();

        let next_move = move_generator.next_move(&board);

        assert!(next_move.is_some() && expected_moves.contains(&next_move.unwrap()));
    }

    #[test]
    fn next_move_will_take_wall_opening_the_biggest_region() {
        let mut board = Board::new(3, 3);
        let player = crate::game::Player::CPU;

        let _ = board.click_wall(0, 1, player);
        let _ = board.click_wall(0, 2, player);
        let _ = board.click_wall(1, 0, player);
        let _ = board.click_wall(1, 1, player);
        let _ = board.click_wall(1, 3, player);
        let _ = board.click_wall(2, 0, player);
        let _ = board.click_wall(2, 1, player);
        let _ = board.click_wall(3, 0, player);
        let _ = board.click_wall(3, 2, player);
        let _ = board.click_wall(4, 2, player);
        let _ = board.click_wall(5, 0, player);
        let _ = board.click_wall(5, 1, player);
        let _ = board.click_wall(5, 3, player);
        let _ = board.click_wall(6, 1, player);

        let expected_moves = [(0, 0), (1, 2), (2, 2), (3, 3)];

        let move_generator = RegionCountingPlayer::default();

        let next_move = move_generator.next_move(&board);
        dbg!(&next_move);

        assert!(next_move.is_some() && expected_moves.contains(&next_move.unwrap()));
    }
}
