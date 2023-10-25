use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::network::models::{Board, Direction, FieldState, Position, Ship};

lazy_static! {
    static ref SHIPS: Vec<PlacementShip> = {
        let mut ships: Vec<Ship> = Vec::new();

        for x in 0..10 {
            for y in 0..10 {
                let start = Position { x, y };

                for size in 2..=5 {
                    for direction in [Direction::Horizontal, Direction::Vertical].into_iter() {
                        match direction {
                            Direction::Horizontal => {
                                if x + size > 10 {
                                    continue;
                                }
                            }
                            Direction::Vertical => {
                                if y + size > 10 {
                                    continue;
                                }
                            }
                        }

                        let ship = Ship {
                            start,
                            size,
                            direction,
                        };

                        ships.push(ship);
                    }
                }
            }
        }

        ships
            .into_iter()
            .enumerate()
            .map(|(id, ship)| PlacementShip { id: Some(id), ship })
            .collect_vec()
    };
}

struct PlacementShip {
    id: Option<usize>,
    ship: Ship,
}

#[derive(Debug)]
pub struct Heatmap {
    pub fields: Vec<Vec<u32>>,
}

impl Heatmap {
    fn get_possible_ships(board: &Board) -> Vec<&'static PlacementShip> {
        let mut possible_ships: Vec<&PlacementShip> = Vec::new();

        for ship in SHIPS.iter() {
            let occupied_fields = ship.ship.get_occupied_fields();

            if occupied_fields
                .iter()
                .any(|field| board.check_field(*field, vec![FieldState::Empty]))
            {
                continue;
            }

            possible_ships.push(ship);
        }

        possible_ships
    }

    fn get_ship_heat(board: &Board) -> HashMap<Ship, u32> {
        let mut placements = board.detect_placements();

        if placements.len() == 1 {
            let placement = placements.pop().unwrap();
            let possible_ships = Self::get_possible_ships(board);

            for placement_ship in possible_ships {
                let mut new_placement = placement.clone();

                new_placement.add_ship(placement_ship.ship).ok();

                placements.push(new_placement);
            }
        }

        placements
            .into_iter()
            .map(|placement| placement.ships)
            .fold(HashMap::new(), |mut acc, ships| {
                for ship in ships {
                    if let Some(heat) = acc.get_mut(&ship) {
                        *heat += 1;
                    } else {
                        acc.insert(ship, 1);
                    }
                }

                acc
            })
    }

    pub fn from_board(board: &Board) -> Heatmap {
        let fields = Self::get_ship_heat(board).into_iter().fold(
            vec![vec![0; 10]; 10],
            |mut acc, (ship, heat)| {
                for field in ship.get_occupied_fields() {
                    acc[field.x as usize][field.y as usize] += heat;
                }

                acc
            },
        );

        Heatmap { fields }
    }
}
