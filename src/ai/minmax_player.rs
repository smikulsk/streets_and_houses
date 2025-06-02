use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use super::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct MinmaxPlayer {
    pub max_depth: usize,
}

type MinmaxCache = std::collections::HashMap<u64, MinmaxBestMoves>;

#[derive(Default, Debug, Clone)]
struct MinmaxBestMoves {
    score: i32,
    moves: Vec<(RowType, ColType)>,
}

impl MinmaxBestMoves {
    fn new(score: i32, moves: &[(RowType, ColType)]) -> Self {
        Self {
            score,
            moves: moves.to_vec(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MinmaxParamters<'a> {
    board: &'a Board,
    depth: usize,
    is_maximizing_player: bool,
    player: Player,
}

impl<'a> MinmaxParamters<'a> {
    fn new(board: &'a Board, depth: usize, is_maximizing_player: bool, player: Player) -> Self {
        Self {
            board,
            depth,
            is_maximizing_player,
            player,
        }
    }
}

impl<'a> Hash for MinmaxParamters<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for wall in self.board.walls.iter().flatten() {
            wall.is_clicked.hash(state);
        }
        for cell in self.board.cells.iter().flatten() {
            cell.owner.hash(state);
        }
        self.depth.hash(state);
        self.is_maximizing_player.hash(state);
        self.player.hash(state);
    }
}

impl MinmaxPlayer {
    pub fn new() -> Self {
        Self {
            max_depth: MAX_MINMAX_DEPTH,
        }
    }

    fn minmax(
        &self,
        params: MinmaxParamters,
        mut alpha: i32,
        mut beta: i32,
        cache: &mut MinmaxCache,
    ) -> MinmaxBestMoves {
        let hash = hash_state(params);

        if let Some(result) = cache.get(&hash) {
            return result.clone();
        }

        if params.depth == 0 || params.board.all_is_clicked() {
            let result = MinmaxBestMoves::new(self.evaluate(params.board), &[]);
            cache.insert(hash, result.clone());
            return result;
        }

        let mut best_score = if params.is_maximizing_player {
            i32::MIN
        } else {
            i32::MAX
        };
        let mut best_moves = vec![];

        for (row, col) in available_moves(params.board) {
            let mut new_board = params.board.clone();
            let additional_move = new_board
                .click_wall(row, col, params.player)
                .expect("the wall was not clicked twice");

            let cur_best = if additional_move {
                self.minmax(
                    MinmaxParamters::new(
                        &new_board,
                        params.depth,
                        params.is_maximizing_player,
                        params.player,
                    ),
                    alpha,
                    beta,
                    cache,
                )
            } else {
                self.minmax(
                    MinmaxParamters::new(
                        &new_board,
                        params.depth - 1,
                        !params.is_maximizing_player,
                        params.player.opponent(),
                    ),
                    alpha,
                    beta,
                    cache,
                )
            };
            #[cfg(feature = "print_debug")]
            if depth == self.max_depth {
                println!("Evaluating move ({row}, {col}): score = {score}");
            }

            if cur_best.score == best_score {
                best_moves.push((row, col));
            }
            if params.is_maximizing_player {
                if cur_best.score > best_score {
                    best_score = best_score.max(cur_best.score);
                    best_moves = vec![(row, col)];
                    alpha = alpha.max(best_score);

                    if beta < alpha {
                        break;
                    }
                }
            } else if cur_best.score < best_score {
                best_score = cur_best.score;
                best_moves = vec![(row, col)];
                beta = beta.min(best_score);
                if beta < alpha {
                    break;
                }
            }
        }

        let result = MinmaxBestMoves::new(best_score, &best_moves);
        cache.insert(hash, result.clone());
        result
    }

    fn evaluate(&self, board: &Board) -> i32 {
        let stats = board.get_statistics();
        stats.cpu_points as i32 - stats.player1_points as i32
    }
}

impl MoveGenerator for MinmaxPlayer {
    fn next_move(&self, board: &Board) -> Option<(RowType, ColType)> {
        let mut cache = HashMap::new();
        let best = self.minmax(
            MinmaxParamters::new(board, self.max_depth, true, Player::CPU),
            i32::MIN,
            i32::MAX,
            &mut cache,
        );
        #[cfg(feature = "print_debug")]
        println!("Best move: {:?}", &best.moves);
        choose_wall_index(&best.moves, |_| true).copied()
    }
}

fn hash_state(params: MinmaxParamters<'_>) -> u64 {
    let hasher = &mut DefaultHasher::new();
    params.hash(hasher);
    hasher.finish()
}

#[cfg(test)]
mod minmax_player_tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn gready_vs_minmax() {
        let sample_move_generator = GreadyAlgorithmPlayer::default();
        let tested_move_generator = MinmaxPlayer::new();

        let stats = play_game(sample_move_generator, tested_move_generator);
        assert_eq!(Some(Player::CPU), stats.winner);
    }

    #[test]
    fn region_vs_minmax() {
        let sample_move_generator = RegionCountingPlayer::default();
        let tested_move_generator = MinmaxPlayer::new();

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
            perform_all_moves(&MinmaxPlayer::new(), &mut board, Player::CPU);
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

    #[test]
    fn double_cell_should_be_taken_in_the_middle() {
        let s = " ----- XXXXX -----
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
 XXXXX ----- -----
|     X     X     X
|     X     X     X
|     X     X     X
|     X     X     X
|     X     X     X
 ----- ----- -----
|     X     |     X
|     X     |     X
|     X     |     X
|     X     |     X
|     X     |     X
 XXXXX XXXXX XXXXX";
        if let Ok(mut board) = Board::from_str(s) {
            perform_all_moves(&MinmaxPlayer::new(), &mut board, Player::CPU);
            assert_eq!(
                " ----- XXXXX -----
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
X     |     X     X
 XXXXX ----- -----
|     X     X     X
|     X     X     X
|     X     X     X
|     X     X     X
|     X     X     X
 XXXXX ----- -----
|     X     |     X
|     X     |     X
|     X     |     X
|     X     |     X
|     X     |     X
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
