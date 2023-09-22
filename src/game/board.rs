use prettytable::{Cell, Row, Table};

use crate::network::models::{Board, BoardOrBool, CellState, Direction, Placement, Position, Ship};

impl Board {
    pub fn get_cell(&self, position: Position) -> CellState {
        if position.x < 0 || position.y < 0 || position.x > 9 || position.y > 9 {
            return CellState::Empty;
        }

        let Board(cells) = self;

        return cells[position.x as usize][position.y as usize];
    }

    pub fn set_cell(&mut self, position: Position, state: CellState) {
        if position.x < 0 || position.y < 0 || position.x > 9 || position.y > 9 {
            return;
        }

        let Board(cells) = self;

        cells[position.x as usize][position.y as usize] = state;
    }

    pub fn check_cell(&self, position: Position, state: Vec<CellState>) -> bool {
        return state.contains(&self.get_cell(position));
    }

    pub fn is_occupied(&self, position: Position) -> bool {
        let neighbor_cells = vec![
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

        for neighbor_cell in neighbor_cells {
            if self.check_cell(
                neighbor_cell,
                vec![CellState::Ship, CellState::Destroyed, CellState::Damaged],
            ) {
                return true;
            }
        }

        return false;
    }

    pub fn find_cells<CB: Fn(&Position) -> bool>(
        &self,
        state: Vec<CellState>,
        filter: CB,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        let Board(cells) = self;

        for (x, col) in cells.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                if state.contains(cell) {
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

        return positions;
    }

    pub fn find_cell<CB: Fn(&Position) -> bool>(
        &self,
        state: Vec<CellState>,
        filter: CB,
    ) -> Option<Position> {
        let Board(cells) = self;

        for (x, col) in cells.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                if state.contains(cell) {
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

        return None;
    }

    pub fn detect_direction(&self, start: &Position) -> Vec<Direction> {
        // Check if ship is horizontal
        if self.check_cell(
            Position {
                x: start.x + 1,
                y: start.y,
            },
            vec![CellState::Ship, CellState::Destroyed, CellState::Damaged],
        ) {
            return vec![Direction::Horizontal];
        }

        // Check if ship is vertical
        if self.check_cell(
            Position {
                x: start.x,
                y: start.y + 1,
            },
            vec![CellState::Ship, CellState::Destroyed, CellState::Damaged],
        ) {
            return vec![Direction::Vertical];
        }

        // Check possible directions
        let mut possible_directions: Vec<Direction> = vec![];

        if self.check_cell(
            Position {
                x: start.x + 1,
                y: start.y,
            },
            vec![CellState::Unknown],
        ) || self.check_cell(
            Position {
                x: start.x - 1,
                y: start.y,
            },
            vec![CellState::Unknown],
        ) {
            possible_directions.push(Direction::Horizontal);
        }

        if self.check_cell(
            Position {
                x: start.x,
                y: start.y + 1,
            },
            vec![CellState::Unknown],
        ) || self.check_cell(
            Position {
                x: start.x,
                y: start.y - 1,
            },
            vec![CellState::Unknown],
        ) {
            possible_directions.push(Direction::Vertical);
        }

        return possible_directions;
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

            if !self.check_cell(
                position,
                vec![CellState::Ship, CellState::Destroyed, CellState::Damaged],
            ) {
                break;
            }
        }

        return size - 1;
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

            if !self.check_cell(position, vec![CellState::Unknown]) {
                break;
            }
        }

        return unknown_end - 1;
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

            if !self.check_cell(position, vec![CellState::Unknown]) {
                break;
            }
        }

        return unknown_start - 1;
    }

    pub fn detect_complete_ships(&self) -> Placement {
        let mut placement = Placement::new();

        loop {
            let start = self.find_cell(vec![CellState::Ship, CellState::Destroyed], |cell| {
                !placement.is_occupied(cell)
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
                start: start,
                direction: direction[0],
                size,
            };

            placement.add_ship(ship).expect("Invalid ship detected");
        }

        return placement;
    }

    pub fn detect_damaged_ship(&self, placement: &Placement) -> Option<Vec<Placement>> {
        let mut missing_ship_sizes = placement.get_missing_ship_sizes();

        missing_ship_sizes.dedup();

        let missing_ship_sizes = missing_ship_sizes;
        let max_size = missing_ship_sizes.first()?;

        let mut placements: Vec<Placement> = Vec::new();

        let start = self.find_cell(vec![CellState::Damaged], |_| true)?;

        let possible_directions = self.detect_direction(&start);

        for direction in possible_directions {
            let detected_size = self.detect_size(&start, &direction, max_size);
            let unknown_end =
                self.detect_unknown_fields_end(&max_size, &detected_size, &start, &direction);
            let unknown_start =
                self.detect_unknown_fields_start(&max_size, &detected_size, &start, &direction);
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

        return Some(placements);
    }

    pub fn detect_placements(&self) -> Vec<Placement> {
        let placement = self.detect_complete_ships();

        return self
            .detect_damaged_ship(&placement)
            .unwrap_or_else(|| vec![placement]);
    }

    pub fn print(&self) {
        let Board(cells) = self;

        let board: Vec<Vec<&str>> = cells
            .iter()
            .map(|col| {
                col.iter()
                    .map(|cell| match cell {
                        CellState::Empty => ".",
                        CellState::Unknown => " ",
                        CellState::Ship => "O",
                        CellState::Damaged => "x",
                        CellState::Destroyed => "X",
                    })
                    .collect()
            })
            .collect();

        let mut table = Table::new();

        for row in board {
            table.add_row(Row::new(row.iter().map(|cell| Cell::new(cell)).collect()));
        }

        table.printstd();
    }
}

impl Into<Option<Board>> for BoardOrBool {
    fn into(self) -> Option<Board> {
        match self {
            BoardOrBool::Board(board) => Some(board),
            BoardOrBool::Bool(_) => None,
        }
    }
}
