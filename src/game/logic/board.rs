use prettytable::{Cell, Row, Table};

use crate::network::models::{
    Board, BoardOrBool, Direction, FieldState, Placement, Position, Ship,
};

impl Board {
    pub fn get_field(&self, position: Position) -> FieldState {
        if position.x < 0 || position.y < 0 || position.x > 9 || position.y > 9 {
            return FieldState::Empty;
        }

        self.fields[position.x as usize][position.y as usize]
    }

    pub fn set_field(&mut self, position: Position, state: FieldState) {
        if position.x < 0 || position.y < 0 || position.x > 9 || position.y > 9 {
            return;
        }

        self.fields[position.x as usize][position.y as usize] = state;
    }

    pub fn check_field(&self, position: Position, state: Vec<FieldState>) -> bool {
        state.contains(&self.get_field(position))
    }

    pub fn is_occupied(&self, position: Position) -> bool {
        let neighbor_fields = vec![
            Position {
                x: position.x - 1,
                y: position.y - 1,
            },
            Position {
                x: position.x,
                y: position.y - 1,
            },
            Position {
                x: position.x + 1,
                y: position.y - 1,
            },
            Position {
                x: position.x - 1,
                y: position.y,
            },
            Position {
                x: position.x + 1,
                y: position.y,
            },
            Position {
                x: position.x - 1,
                y: position.y + 1,
            },
            Position {
                x: position.x,
                y: position.y + 1,
            },
            Position {
                x: position.x + 1,
                y: position.y + 1,
            },
        ];

        for neighbor_field in neighbor_fields {
            if self.check_field(
                neighbor_field,
                vec![FieldState::Ship, FieldState::Destroyed, FieldState::Damaged],
            ) {
                return true;
            }
        }

        false
    }

    pub fn find_fields<CB: Fn(&Position) -> bool>(
        &self,
        state: Vec<FieldState>,
        filter: CB,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        for (x, col) in self.fields.iter().enumerate() {
            for (y, field) in col.iter().enumerate() {
                if state.contains(field) {
                    let position = Position {
                        x: x as i8,
                        y: y as i8,
                    };

                    if !filter(&position) {
                        continue;
                    }

                    positions.push(position);
                }
            }
        }

        positions
    }

    pub fn find_field<CB: Fn(&Position) -> bool>(
        &self,
        state: Vec<FieldState>,
        filter: CB,
    ) -> Option<Position> {
        for (x, col) in self.fields.iter().enumerate() {
            for (y, field) in col.iter().enumerate() {
                if state.contains(field) {
                    let position = Position {
                        x: x as i8,
                        y: y as i8,
                    };

                    if !filter(&position) {
                        continue;
                    }

                    return Some(position);
                }
            }
        }

        None
    }

    pub fn detect_direction(&self, start: &Position) -> Vec<Direction> {
        // Check if ship is horizontal
        if self.check_field(
            Position {
                x: start.x + 1,
                y: start.y,
            },
            vec![FieldState::Ship, FieldState::Destroyed, FieldState::Damaged],
        ) {
            return vec![Direction::Horizontal];
        }

        // Check if ship is vertical
        if self.check_field(
            Position {
                x: start.x,
                y: start.y + 1,
            },
            vec![FieldState::Ship, FieldState::Destroyed, FieldState::Damaged],
        ) {
            return vec![Direction::Vertical];
        }

        // Check possible directions
        let mut possible_directions: Vec<Direction> = vec![];

        if self.check_field(
            Position {
                x: start.x + 1,
                y: start.y,
            },
            vec![FieldState::Unknown],
        ) || self.check_field(
            Position {
                x: start.x - 1,
                y: start.y,
            },
            vec![FieldState::Unknown],
        ) {
            possible_directions.push(Direction::Horizontal);
        }

        if self.check_field(
            Position {
                x: start.x,
                y: start.y + 1,
            },
            vec![FieldState::Unknown],
        ) || self.check_field(
            Position {
                x: start.x,
                y: start.y - 1,
            },
            vec![FieldState::Unknown],
        ) {
            possible_directions.push(Direction::Vertical);
        }

        possible_directions
    }

    pub fn detect_size(&self, start: &Position, direction: &Direction, max_size: &i8) -> i8 {
        let mut size: i8 = 1;

        while size <= *max_size {
            size += 1;

            let x = start.x
                + if *direction == Direction::Horizontal {
                    size - 1
                } else {
                    0
                };
            let y = start.y
                + if *direction == Direction::Vertical {
                    size - 1
                } else {
                    0
                };

            let position = Position { x, y };

            if !self.check_field(
                position,
                vec![FieldState::Ship, FieldState::Destroyed, FieldState::Damaged],
            ) {
                break;
            }
        }

        size - 1
    }

    pub fn detect_unknown_fields_end(
        &self,
        max_size: &i8,
        size: &i8,
        start: &Position,
        direction: &Direction,
    ) -> i8 {
        let mut unknown_end = 0;

        while unknown_end <= (max_size - size) {
            unknown_end += 1;

            let position = Position {
                x: start.x
                    + if *direction == Direction::Horizontal {
                        size + unknown_end - 1
                    } else {
                        0
                    },
                y: start.y
                    + if *direction == Direction::Vertical {
                        size + unknown_end - 1
                    } else {
                        0
                    },
            };

            if !self.check_field(position, vec![FieldState::Unknown]) {
                break;
            }
        }

        unknown_end - 1
    }

    pub fn detect_unknown_fields_start(
        &self,
        max_size: &i8,
        size: &i8,
        start: &Position,
        direction: &Direction,
    ) -> i8 {
        let mut unknown_start = 0;

        while unknown_start <= (max_size - size) {
            unknown_start += 1;

            let position = Position {
                x: start.x
                    - (if *direction == Direction::Horizontal {
                        unknown_start
                    } else {
                        0
                    }),
                y: start.y
                    - (if *direction == Direction::Vertical {
                        unknown_start
                    } else {
                        0
                    }),
            };

            if !self.check_field(position, vec![FieldState::Unknown]) {
                break;
            }
        }

        unknown_start - 1
    }

    pub fn detect_complete_ships(&self) -> Placement {
        let mut placement = Placement::new();

        loop {
            let start = self.find_field(vec![FieldState::Ship, FieldState::Destroyed], |field| {
                !placement.is_occupied(field)
            });

            let start = match start {
                Some(start) => start,
                None => break,
            };

            let direction = self.detect_direction(&start);
            let size = self.detect_size(
                &start,
                &direction[0],
                &placement.get_missing_ship_sizes()[0],
            );

            let ship = Ship {
                start,
                direction: direction[0],
                size,
            };

            placement.add_ship(ship).expect("Invalid ship detected");
        }

        placement
    }

    pub fn detect_damaged_ship(&self, placement: &Placement) -> Option<Vec<Placement>> {
        let mut missing_ship_sizes = placement.get_missing_ship_sizes();

        missing_ship_sizes.dedup();

        let missing_ship_sizes = missing_ship_sizes;
        let max_size = missing_ship_sizes.first()?;

        let mut placements: Vec<Placement> = Vec::new();

        let start = self.find_field(vec![FieldState::Damaged], |_| true)?;

        let possible_directions = self.detect_direction(&start);

        for direction in possible_directions {
            let detected_size = self.detect_size(&start, &direction, max_size);
            let unknown_end =
                self.detect_unknown_fields_end(max_size, &detected_size, &start, &direction);
            let unknown_start =
                self.detect_unknown_fields_start(max_size, &detected_size, &start, &direction);
            let size_min = detected_size + 1;

            for size in &missing_ship_sizes {
                if *size < size_min {
                    continue;
                }

                let missing_fields = size - detected_size;
                let offset_min = std::cmp::max(0, missing_fields - unknown_end);
                let offset_max = std::cmp::min(missing_fields, unknown_start);

                for offset in offset_min..=offset_max {
                    let new_start = Position {
                        x: start.x
                            - (if direction == Direction::Horizontal {
                                offset
                            } else {
                                0
                            }),
                        y: start.y
                            - (if direction == Direction::Vertical {
                                offset
                            } else {
                                0
                            }),
                    };

                    let ship = Ship {
                        start: new_start,
                        direction,
                        size: *size,
                    };

                    let mut new_placement = placement.clone();

                    if new_placement.add_ship(ship).is_ok() {
                        placements.push(new_placement);
                    }
                }
            }
        }

        if placements.is_empty() {
            return None;
        }

        Some(placements)
    }

    pub fn detect_placements(&self) -> Vec<Placement> {
        let placement = self.detect_complete_ships();

        self.detect_damaged_ship(&placement)
            .unwrap_or_else(|| vec![placement])
    }

    pub fn print(&self) {
        let board: Vec<Vec<&str>> = self
            .fields
            .iter()
            .map(|col| {
                col.iter()
                    .map(|field| match field {
                        FieldState::Empty => ".",
                        FieldState::Unknown => " ",
                        FieldState::Ship => "O",
                        FieldState::Damaged => "x",
                        FieldState::Destroyed => "X",
                    })
                    .collect()
            })
            .collect();

        let mut table = Table::new();

        for row in board {
            table.add_row(Row::new(row.iter().map(|field| Cell::new(field)).collect()));
        }

        table.printstd();
    }
}

impl From<BoardOrBool> for Option<Board> {
    fn from(val: BoardOrBool) -> Self {
        match val {
            BoardOrBool::Board(board) => Some(board),
            BoardOrBool::Bool(_) => None,
        }
    }
}
