use itertools::Itertools;
use std::collections::HashMap;

use crate::network::models::{Board, FieldState, GameRoundEvent, Position};

pub fn round_handler(event: GameRoundEvent) -> Position {
    let opponent_board = event.get_opponent_board();

    let strategies = [next_attack, second_attack, first_attack];

    strategies
        .iter()
        .find_map(|strategy| strategy(&opponent_board))
        .expect("No strategy returned a position")
}

fn first_attack(board: &Board) -> Option<Position> {
    board.find_field(vec![FieldState::Unknown], |position| {
        position.x % 2 == position.y % 2 && !board.is_occupied(*position)
    })
}

fn second_attack(board: &Board) -> Option<Position> {
    let damaged_field = board.find_field(vec![FieldState::Damaged], |_| true)?;

    let mut neighbor_fields = HashMap::from([
        (
            Position {
                x: damaged_field.x - 1,
                y: damaged_field.y,
            },
            0,
        ),
        (
            Position {
                x: damaged_field.x + 1,
                y: damaged_field.y,
            },
            0,
        ),
        (
            Position {
                x: damaged_field.x,
                y: damaged_field.y - 1,
            },
            0,
        ),
        (
            Position {
                x: damaged_field.x,
                y: damaged_field.y + 1,
            },
            0,
        ),
    ]);

    let placements = board.detect_placements();

    for placement in placements {
        for ship in &placement.ships {
            for field in ship.get_occupied_fields() {
                if let Some(count) = neighbor_fields.get_mut(&field) {
                    *count += 1;
                }
            }
        }
    }

    let position = neighbor_fields
        .iter()
        .sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a))
        .map(|(position, _)| position)
        .find(|&position| board.check_field(*position, vec![FieldState::Unknown]))?;

    Some(*position)
}

fn next_attack(board: &Board) -> Option<Position> {
    let damaged_fields = board.find_fields(vec![FieldState::Damaged], |_| true);

    if damaged_fields.len() == 1 {
        return None;
    }

    let first_damaged_field = damaged_fields.first()?;
    let last_damaged_field = damaged_fields.last()?;

    let placements = board.detect_placements();

    if first_damaged_field.x != last_damaged_field.x {
        let mut neighbor_fields = HashMap::from([
            (
                Position {
                    x: first_damaged_field.x - 1,
                    y: first_damaged_field.y,
                },
                0,
            ),
            (
                Position {
                    x: last_damaged_field.x + 1,
                    y: last_damaged_field.y,
                },
                0,
            ),
        ]);

        for placement in placements {
            for ship in &placement.ships {
                for field in ship.get_occupied_fields() {
                    if let Some(count) = neighbor_fields.get_mut(&field) {
                        *count += 1;
                    }
                }
            }
        }

        let position = neighbor_fields
            .iter()
            .sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a))
            .map(|(position, _)| position)
            .find(|&position| board.check_field(*position, vec![FieldState::Unknown]))?;

        Some(*position)
    } else {
        let mut neighbor_fields = HashMap::from([
            (
                Position {
                    x: first_damaged_field.x,
                    y: first_damaged_field.y - 1,
                },
                0,
            ),
            (
                Position {
                    x: last_damaged_field.x,
                    y: last_damaged_field.y + 1,
                },
                0,
            ),
        ]);

        for placement in placements {
            for ship in &placement.ships {
                for field in ship.get_occupied_fields() {
                    if let Some(count) = neighbor_fields.get_mut(&field) {
                        *count += 1;
                    }
                }
            }
        }

        let position = neighbor_fields
            .iter()
            .sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a))
            .map(|(position, _)| position)
            .find(|&position| board.check_field(*position, vec![FieldState::Unknown]))?;

        Some(*position)
    }
}
