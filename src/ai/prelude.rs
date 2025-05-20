pub use super::greedy_algorithm_player::*;
pub use super::region_counting_player::*;
pub use super::*;

use crate::game::Difficulty;

pub fn get_cpu_player(difficulty: &Difficulty) -> Box<dyn MoveGenerator> {
    match difficulty {
        Difficulty::Easy => Box::new(GreadyAlgorithmPlayer::default()),
        Difficulty::Medium => Box::new(RegionCountingPlayer::default()),
    }
}
