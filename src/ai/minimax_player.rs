use super::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct MinimaxPlayer {
    pub max_depth: usize,
}

impl MinimaxPlayer {
    pub fn new() -> Self {
        Self {
            max_depth: MAX_MINMAX_DEPTH,
        }
    }

    fn minimax(
        &self,
        board: &Board,
        depth: usize,
        maximizing_player: bool,
        mut alpha: i32,
        mut beta: i32,
        player: Player,
    ) -> (i32, Option<(RowType, ColType)>) {
        if depth == 0 || board.all_is_clicked() {
            return (self.evaluate(board), None);
        }

        let mut best_score = if maximizing_player {
            i32::MIN
        } else {
            i32::MAX
        };
        let mut best_move = None;

        for (row, col) in available_moves(board) {
            let mut new_board = board.clone();
            let additional_move = new_board
                .click_wall(row, col, player)
                .expect("the wall was not clicked twice");

            let (score, _) = if additional_move {
                self.minimax(&new_board, depth, maximizing_player, alpha, beta, player)
            } else {
                self.minimax(
                    &new_board,
                    depth - 1,
                    !maximizing_player,
                    alpha,
                    beta,
                    player.opponent(),
                )
            };
            #[cfg(feature="print_debug")]
            if depth == self.max_depth {
                println!("Evaluating move ({row}, {col}): score = {score}");
            }

            if maximizing_player {
                if score > best_score {
                    best_score = best_score.max(score);
                    best_move = Some((row, col));
                    alpha = alpha.max(best_score);

                    if beta <= alpha {
                        break;
                    }
                }
            } else if score < best_score {
                best_score = score;
                best_move = Some((row, col));
                beta = beta.min(best_score);
                if beta <= alpha {
                    break;
                }
            }
        }

        (best_score, best_move)
    }

    fn evaluate(&self, board: &Board) -> i32 {
        let stats = board.get_statistics();
        stats.cpu_points as i32 - stats.player1_points as i32
    }
}

fn available_moves(board: &Board) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    let mut unclicked_walls = collect_wall_statistics(board);
    unclicked_walls.sort_by_key(wall_priority);
    unclicked_walls.into_iter().map(|ws| (ws.row, ws.col))
}

impl MoveGenerator for MinimaxPlayer {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)> {
        let (_, best_move) =
            self.minimax(board, self.max_depth, true, i32::MIN, i32::MAX, Player::CPU);
        #[cfg(feature="print_debug")]
        println!("Best move: {:?}", best_move);
        best_move
    }
}

#[cfg(test)]
mod minimax_player_tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn gready_vs_minimax() {
        let sample_move_generator = GreadyAlgorithmPlayer::default();
        let tested_move_generator = MinimaxPlayer::new();

        let stats = play_game(sample_move_generator, tested_move_generator);
        assert_eq!(Some(Player::CPU), stats.winner);
    }

    #[test]
    fn region_vs_minimax() {
        let sample_move_generator = RegionCountingPlayer::default();
        let tested_move_generator = MinimaxPlayer::new();

        let stats = play_game(sample_move_generator, tested_move_generator);
        assert_eq!(Some(Player::CPU), stats.winner);
    }

    #[test]
    fn load_board() {
        let s = " XXXXX XXXXX XXXXX
X     |     X     |
X     |     X     |
X     |     X     |
X     |     X     |
X     |     X     |
 ----- ----- -----
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
 XXXXX XXXXX -----
XAAAAAXCCCCCX     X
XAAAAAXCCCCCX     X
XAAAAAXCCCCCX     X
XAAAAAXCCCCCX     X
XAAAAAXCCCCCX     X
 XXXXX XXXXX XXXXX";
        if let Ok(mut board) = Board::from_str(s) {
            perform_all_moves(&MinimaxPlayer::new(), &mut board, Player::CPU);
            assert_eq!(
                " XXXXX XXXXX XXXXX
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
 ----- ----- -----
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
 XXXXX XXXXX XXXXX
XAAAAAXCCCCCXCCCCCX
XAAAAAXCCCCCXCCCCCX
XAAAAAXCCCCCXCCCCCX
XAAAAAXCCCCCXCCCCCX
XAAAAAXCCCCCXCCCCCX
 XXXXX XXXXX XXXXX
",
                format!("{board}")
            );
        }
    }

    fn play_game(
        first_player_move_generator: impl MoveGenerator,
        second_player_move_generator: impl MoveGenerator,
    ) -> GameStatistics {
        let width = 5;
        let height = 5;

        let mut board = Board::new(width, height);

        while !board.all_is_clicked() {
            perform_all_moves(&first_player_move_generator, &mut board, Player::Player1);
            if board.all_is_clicked() {
                break;
            }
            perform_all_moves(&second_player_move_generator, &mut board, Player::CPU);
        }
        let stats = board.get_statistics();
        println!("{board}");
        println!("{stats:?}");
        stats
    }

    fn perform_all_moves(move_generator: &impl MoveGenerator, board: &mut Board, player: Player) {
        let (mut row, mut col) = move_generator
            .next_move(board)
            .expect("there is a move available");

        while let Ok(true) = board.click_wall(row, col, player) {
            if let Some((r, c)) = move_generator.next_move(board) {
                row = r;
                col = c;
            } else {
                break;
            }
        }
    }
}
