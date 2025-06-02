pub use super::greedy_algorithm_player::*;
pub use super::minmax_player::*;
pub use super::region_counting_player::*;
pub use super::*;

pub use crate::game::*;
pub use crate::rendering::constants::*;

pub fn get_cpu_player(difficulty: &Difficulty) -> Box<dyn MoveGenerator> {
    match difficulty {
        Difficulty::Easy => Box::new(GreadyAlgorithmPlayer::default()),
        Difficulty::Medium => Box::new(RegionCountingPlayer::default()),
        Difficulty::Hard => Box::new(MinmaxPlayer::default()),
    }
}
