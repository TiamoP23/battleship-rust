use std::{cell::RefCell, collections::HashMap, thread, time::Duration};

use itertools::Itertools;

use crate::network::models::{
    Board, CellState, GameEvent, GameResponse, GameRoundEvent, Placement, Position,
};

mod board;
mod direction;
mod game_details;
mod game_event;
mod placement;
mod position;
mod ship;
mod with_bounds;

thread_local!(static SPECTATED_GAME_ID: RefCell<Option<String>> = RefCell::new(None));

pub fn handle_game_event(event: GameEvent) -> Option<GameResponse> {
    match event {
        GameEvent::Init(event) => {
            SPECTATED_GAME_ID.with(|game_id| {
                if game_id.borrow().is_none() {
                    *game_id.borrow_mut() = Some(event.details.id.clone());
                    println!("Spectating game {}", event.details.id);
                }
            });

            None
        }
        GameEvent::Round(event) => {
            let own_board = event.get_self_board();
            let opponent_board = event.get_opponent_board();

            let is_spectated_game = SPECTATED_GAME_ID.with(|game_id| {
                if let Some(game_id) = game_id.borrow().as_ref() {
                    game_id == &event.details.id
                } else {
                    false
                }
            });

            if is_spectated_game {
                thread::sleep(Duration::from_millis(500));
                println!("Own Board");
                own_board.print();

                println!("Opponent Board");
                opponent_board.print();
            }

            let position = attack(event);

            if is_spectated_game {
                println!("Shooting at {:?}", position);
            }

            Some(GameResponse::Attack(position))
        }
        GameEvent::Result(event) => {
            SPECTATED_GAME_ID.with(|game_id| {
                if game_id.borrow().is_some() {
                    let won = event.details.get_self().score > event.details.get_opponent().score;

                    if won {
                        println!("Won game {}", event.details.id);
                    } else {
                        println!("Lost game {}", event.details.id);
                    }

                    *game_id.borrow_mut() = None;
                }
            });

            None
        }
        GameEvent::Set(event) => {
            let placement: Placement = place_ships();

            let is_spectated_game = SPECTATED_GAME_ID.with(|game_id| {
                if let Some(game_id) = game_id.borrow().as_ref() {
                    game_id == &event.details.id
                } else {
                    false
                }
            });

            if is_spectated_game {
                println!("Placing ships");

                placement.print();
            }

            Some(GameResponse::PlaceShips(placement))
        }
    }
}

fn place_ships() -> Placement {
    let placement: Placement = rand::random();

    return placement;
}

fn attack(event: GameRoundEvent) -> Position {
    let opponent_board = event.get_opponent_board();

    if let Some(position) = next_attack(opponent_board) {
        return position;
    }

    if let Some(position) = second_attack(opponent_board) {
        return position;
    }

    return first_attack(opponent_board);
}

fn first_attack(board: &Board) -> Position {
    let position = board
        .find_cell(vec![CellState::Unknown], |position| {
            position.x % 2 == position.y % 2 && !board.is_occupied(*position)
        })
        .unwrap();

    return position;
}

fn second_attack(board: &Board) -> Option<Position> {
    let damaged_field = board.find_cell(vec![CellState::Damaged], |_| true)?;

    let mut neighbor_cells = HashMap::from([
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
            for cell in ship.get_occupied_cells() {
                if let Some(count) = neighbor_cells.get_mut(&cell) {
                    *count += 1;
                }
            }
        }
    }

    let position = neighbor_cells
        .iter()
        .sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a))
        .map(|(position, _)| position)
        .find(|&position| board.check_cell(*position, vec![CellState::Unknown]))?;

    return Some(*position);
}

fn next_attack(board: &Board) -> Option<Position> {
    let damaged_fields = board.find_cells(vec![CellState::Damaged], |_| true);

    let first_damaged_field = damaged_fields.first()?;
    let last_damaged_field = damaged_fields.last()?;

    if first_damaged_field.x != last_damaged_field.x {
        let mut neighbor_cells = HashMap::from([
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

        let placements = board.detect_placements();

        for placement in placements {
            for ship in &placement.ships {
                for cell in ship.get_occupied_cells() {
                    if let Some(count) = neighbor_cells.get_mut(&cell) {
                        *count += 1;
                    }
                }
            }
        }

        let position = neighbor_cells
            .iter()
            .sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a))
            .map(|(position, _)| position)
            .find(|&position| board.check_cell(*position, vec![CellState::Unknown]))?;

        return Some(*position);
    } else {
        let neighbor_cells = vec![
            Position {
                x: first_damaged_field.x,
                y: first_damaged_field.y - 1,
            },
            Position {
                x: last_damaged_field.x,
                y: last_damaged_field.y + 1,
            },
        ];

        let position = neighbor_cells
            .iter()
            .find(|&position| board.check_cell(*position, vec![CellState::Unknown]))?;

        return Some(*position);
    }
}
