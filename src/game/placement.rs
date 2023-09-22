use prettytable::{Cell, Row, Table};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::network::models::{Direction, Placement, Position, Ship};

impl Placement {
    pub fn new() -> Placement {
        return Placement { ships: vec![] };
    }

    pub fn is_occupied(&self, cell: &Position) -> bool {
        return self.ships.iter().any(|ship| ship.collides_with(cell));
    }

    pub fn get_missing_ship_sizes(&self) -> Vec<i8> {
        let mut missing_ship_sizes = vec![5, 4, 3, 3, 2];

        for ship in &self.ships {
            if let Some(index) = missing_ship_sizes
                .iter()
                .position(|size| *size == ship.size)
            {
                missing_ship_sizes.remove(index);
            }
        }

        return missing_ship_sizes;
    }

    pub fn add_ship(&mut self, ship: Ship) -> Result<(), &str> {
        if self.ships.len() >= 5 {
            return Err("Cannot add more than 5 ships");
        }

        if !self.get_missing_ship_sizes().contains(&ship.size) {
            return Err("Invalid ship size");
        }

        if self.ships.iter().any(|other| other.collides_with(&ship)) {
            return Err("Ship collides with other ship");
        }

        if ship.start.x < 0
            || ship.start.x > 9
            || ship.start.y < 0
            || ship.start.y > 9
            || ship.get_end().x < 0
            || ship.get_end().x > 9
            || ship.get_end().y < 0
            || ship.get_end().y > 9
        {
            return Err("Ship is out of bounds");
        }

        self.ships.push(ship);

        return Ok(());
    }

    pub fn all_ships_placed(&self) -> bool {
        return self.ships.len() == 5;
    }

    pub fn print(&self) {
        let mut board = vec![vec![" "; 10]; 10];

        for ship in &self.ships {
            for cell in ship.get_occupied_cells() {
                board[cell.x as usize][cell.y as usize] = "O";
            }
        }

        let mut table = Table::new();

        for row in board {
            table.add_row(Row::new(row.iter().map(|cell| Cell::new(cell)).collect()));
        }

        table.printstd();
    }
}

impl PartialEq for Placement {
    fn eq(&self, other: &Self) -> bool {
        return self
            .ships
            .iter()
            .all(|ship| other.ships.iter().any(|other_ship| ship == other_ship));
    }
}

impl Distribution<Placement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Placement {
        let mut placement = Placement::new();
        let mut sizes = vec![5, 4, 3, 3, 2];

        while !sizes.is_empty() {
            let size = sizes[0];
            let start: Position = rand::random();
            let direction: Direction = rand::random();

            let ship = Ship {
                start,
                size,
                direction,
            };

            if placement.add_ship(ship).is_ok() {
                sizes.remove(0);
            }
        }

        return placement;
    }
}
