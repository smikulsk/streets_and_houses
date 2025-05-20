use crate::ai::prelude::*;

use std::cmp::Ordering;

pub mod controller;

#[derive(Debug, Clone)]
pub enum GameMode {
    OnePlayer(Box<dyn MoveGenerator>),
    TwoPlayer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Player1,
    Player2,
    CPU,
}

#[derive(Debug, Clone)]
pub struct GameStatistics {
    pub player1_points: usize,
    pub player2_points: usize,
    pub cpu_points: usize,
    pub winner: Option<Player>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    _id: (usize, usize),
    pub counter: usize,
    pub owner: Option<Player>,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            _id: (row, col),
            counter: 0,
            owner: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Wall {
    _id: (usize, usize),
    pub is_clicked: bool,
    pub adjacent_cells: Vec<(usize, usize)>,
    pub adjacent_joints: Vec<(Direction, usize, usize)>,
}

impl Wall {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            _id: (row, col),
            is_clicked: false,
            adjacent_cells: vec![],
            adjacent_joints: vec![],
        }
    }

    pub fn adjacent_to(&mut self, cell_ids: &[(usize, usize)]) -> &mut Self {
        self.adjacent_cells = cell_ids.to_vec();
        self
    }

    pub fn with_joint(&mut self, direction: Direction, row: usize, col: usize) -> &mut Self {
        self.adjacent_joints.push((direction, row, col));
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Joint {
    _id: (usize, usize),
    north_wall_clicked: bool,
    east_wall_clicked: bool,
    south_wall_clicked: bool,
    west_wall_clicked: bool,
}

impl Joint {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            _id: (row, col),
            north_wall_clicked: false,
            east_wall_clicked: false,
            south_wall_clicked: false,
            west_wall_clicked: false,
        }
    }

    pub fn set_wall_clicked(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.north_wall_clicked = true,
            Direction::East => self.east_wall_clicked = true,
            Direction::South => self.south_wall_clicked = true,
            Direction::West => self.west_wall_clicked = true,
        }
    }

    pub fn get_joint_mask(&self) -> usize {
        [
            self.north_wall_clicked,
            self.east_wall_clicked,
            self.south_wall_clicked,
            self.west_wall_clicked,
        ]
        .iter()
        .map(|&flag| usize::from(flag))
        .fold(0, |acc, x| 2 * acc + x)
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub joints: Vec<Vec<Joint>>,
    pub walls: Vec<Vec<Wall>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..height)
            .map(|row| {
                (0..width)
                    .map(|col| Cell::new(row, col))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let joints = (0..height + 1)
            .map(|row| {
                (0..width + 1)
                    .map(|col| Joint::new(row, col))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let walls = (0..2 * height + 1)
            .map(|row| {
                if row % 2 == 0 {
                    (0..width)
                        .map(|col| {
                            let adjacent_cells = [
                                (row as isize / 2 - 1, col as isize),
                                (row as isize / 2, col as isize),
                            ]
                            .iter()
                            .filter_map(|(r, c)| {
                                check_coordinates(width as isize, height as isize, *r, *c)
                            })
                            .collect::<Vec<_>>();

                            Wall::new(row, col)
                                .adjacent_to(&adjacent_cells.clone())
                                .with_joint(Direction::East, row / 2, col)
                                .with_joint(Direction::West, row / 2, col + 1)
                                .clone()
                        })
                        .collect::<Vec<_>>()
                } else {
                    (0..width + 1)
                        .map(|col| {
                            let adjacent_cells = [
                                (row as isize / 2, col as isize - 1),
                                (row as isize / 2, col as isize),
                            ]
                            .iter()
                            .filter_map(|(r, c)| {
                                check_coordinates(width as isize, height as isize, *r, *c)
                            })
                            .collect::<Vec<_>>();

                            Wall::new(row, col)
                                .adjacent_to(&adjacent_cells.clone())
                                .with_joint(Direction::South, row / 2, col)
                                .with_joint(Direction::North, row / 2 + 1, col)
                                .clone()
                        })
                        .collect::<Vec<_>>()
                }
            })
            .collect::<Vec<_>>();

        Self {
            width,
            height,
            cells,
            joints,
            walls,
        }
    }

    pub fn click_wall(&mut self, row: usize, col: usize, player: Player) -> Result<bool, String> {
        if row > 2 * self.height || (row % 2 == 0 && col >= self.width) || col > self.width {
            return Err("Wrong coordinates of wall".to_string());
        }
        let wall = &mut self.walls[row][col];
        let mut additional_move = false;
        if !wall.is_clicked {
            wall.is_clicked = true;
            for (cell_row, cell_col) in &wall.adjacent_cells {
                let cell = &mut self.cells[*cell_row][*cell_col];
                if cell.counter < 4 {
                    cell.counter += 1;
                    if cell.counter == 4 {
                        cell.owner = Some(player);
                        additional_move = true;
                    }
                }
            }
            for (direction, row, col) in &wall.adjacent_joints {
                self.joints[*row][*col].set_wall_clicked(*direction);
            }
        } else {
            return Err("Wall is already clicked!".to_string());
        }

        Ok(additional_move)
    }

    pub fn all_is_clicked(&self) -> bool {
        for row in &self.walls {
            for wall in row {
                if !wall.is_clicked {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_statistics(&self) -> GameStatistics {
        let player1_points = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| cell.owner == Some(Player::Player1))
                    .count()
            })
            .sum::<usize>();
        let player2_points = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| cell.owner == Some(Player::Player2))
                    .count()
            })
            .sum::<usize>();
        let cpu_points = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| cell.owner == Some(Player::CPU))
                    .count()
            })
            .sum::<usize>();
        let winner = match player1_points.cmp(&player2_points) {
            Ordering::Greater => compare_with_cpu(player1_points, cpu_points),
            Ordering::Less => Some(Player::Player2),
            Ordering::Equal => compare_with_cpu(player1_points, cpu_points),
        };
        GameStatistics {
            player1_points,
            player2_points,
            cpu_points,
            winner,
        }
    }
}

fn compare_with_cpu(player1_points: usize, cpu_points: usize) -> Option<Player> {
    match player1_points.cmp(&cpu_points) {
        Ordering::Greater => Some(Player::Player1),
        Ordering::Less => Some(Player::CPU),
        Ordering::Equal => None,
    }
}

fn check_coordinates(
    width: isize,
    height: isize,
    row: isize,
    col: isize,
) -> Option<(usize, usize)> {
    if 0 <= row && row < height && 0 <= col && col < width {
        return Some((row as usize, col as usize));
    }
    None
}

#[cfg(test)]
mod joint_tests {
    use super::*;

    #[test]
    fn when_initialized_no_wall_clicked_is_true() {
        let joint = Joint::new(0, 0);

        assert!(!joint.north_wall_clicked);
        assert!(!joint.east_wall_clicked);
        assert!(!joint.south_wall_clicked);
        assert!(!joint.west_wall_clicked);
    }

    #[test]
    fn setting_north_wall_clicked_works_correctly() {
        let mut joint = Joint::new(0, 0);

        joint.set_wall_clicked(Direction::North);

        assert!(joint.north_wall_clicked);
        assert!(!joint.east_wall_clicked);
        assert!(!joint.south_wall_clicked);
        assert!(!joint.west_wall_clicked);
    }

    #[test]
    fn setting_east_wall_clicked_works_correctly() {
        let mut joint = Joint::new(0, 0);

        joint.set_wall_clicked(Direction::East);

        assert!(!joint.north_wall_clicked);
        assert!(joint.east_wall_clicked);
        assert!(!joint.south_wall_clicked);
        assert!(!joint.west_wall_clicked);
    }

    #[test]
    fn setting_south_wall_clicked_works_correctly() {
        let mut joint = Joint::new(0, 0);

        joint.set_wall_clicked(Direction::South);

        assert!(!joint.north_wall_clicked);
        assert!(!joint.east_wall_clicked);
        assert!(joint.south_wall_clicked);
        assert!(!joint.west_wall_clicked);
    }

    #[test]
    fn setting_west_wall_clicked_works_correctly() {
        let mut joint = Joint::new(0, 0);

        joint.set_wall_clicked(Direction::West);

        assert!(!joint.north_wall_clicked);
        assert!(!joint.east_wall_clicked);
        assert!(!joint.south_wall_clicked);
        assert!(joint.west_wall_clicked);
    }

    #[test]
    fn joint_masks_are_generated_correctly() {
        for expected_mask in 0..16 {
            let mut joint = Joint::new(0, 0);

            joint.north_wall_clicked = expected_mask & 8 > 0;
            joint.east_wall_clicked = expected_mask & 4 > 0;
            joint.south_wall_clicked = expected_mask & 2 > 0;
            joint.west_wall_clicked = expected_mask & 1 > 0;

            assert_eq!(expected_mask, joint.get_joint_mask());
        }
    }
}
