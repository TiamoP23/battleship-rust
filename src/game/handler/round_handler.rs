use itertools::Itertools;
use log::debug;

use crate::{
    application::database::DB_POOL,
    game::logic::heatmap::Heatmap,
    network::models::{Board, FieldState, GameRoundEvent, Position},
};

pub async fn round_handler(event: GameRoundEvent) -> Position {
    let pool = DB_POOL.get().unwrap();
    let opponent = event.details.get_opponent();

    for round in event
        .details
        .log
        .iter()
        .rev()
        .take_while(|round| round.player == opponent.id)
    {
        let move_x = match round.game_move {
            Some(position) => Some(position.x as i16),
            None => None,
        };

        let move_y = match round.game_move {
            Some(position) => Some(position.y as i16),
            None => None,
        };

        sqlx::query!(
            r#"INSERT INTO round (game_id, opponent_move, move_x, move_y) VALUES ($1, $2, $3, $4)"#,
            event.details.id,
            true,
            move_x,
            move_y,
        )
        .execute(pool)
        .await
        .expect("Failed to insert round in database");
    }

    let strategies = vec![next_attack, second_attack, first_attack];

    let opponent_board = event.get_opponent_board();
    let heatmap = Heatmap::from_board(opponent_board);

    debug!("Heatmap for game {}: {:#?}", event.details.id, heatmap);

    let position = strategies
        .iter()
        .enumerate()
        .find_map(|(index, strategy)| {
            let position = strategy(&opponent_board)
                .into_iter()
                .sorted_by(|a, b| {
                    let a_heat = heatmap.fields[a.x as usize][a.y as usize];
                    let b_heat = heatmap.fields[b.x as usize][b.y as usize];

                    b_heat.cmp(&a_heat)
                })
                .next()?;

            debug!(
                "Using strategy {} to attack at {:?} in game {}",
                index + 1,
                position,
                event.details.id
            );

            Some(position)
        })
        .expect("No strategy returned a position");

    let pool = DB_POOL.get().unwrap();

    sqlx::query!(
        r#"INSERT INTO round (game_id, opponent_move, move_x, move_y) VALUES ($1, $2, $3, $4)"#,
        event.details.id,
        false,
        position.x as i16,
        position.y as i16,
    )
    .execute(pool)
    .await
    .expect("Failed to insert round in database");

    position
}

fn first_attack(board: &Board) -> Vec<Position> {
    board.find_fields(vec![FieldState::Unknown], |position| {
        position.x % 2 == position.y % 2 && !board.is_occupied(*position)
    })
}

fn second_attack(board: &Board) -> Vec<Position> {
    let damaged_field = match board.find_field(vec![FieldState::Damaged], |_| true) {
        Some(field) => field,
        None => return Vec::new(),
    };

    let neighbor_fields = vec![
        Position {
            x: damaged_field.x - 1,
            y: damaged_field.y,
        },
        Position {
            x: damaged_field.x + 1,
            y: damaged_field.y,
        },
        Position {
            x: damaged_field.x,
            y: damaged_field.y - 1,
        },
        Position {
            x: damaged_field.x,
            y: damaged_field.y + 1,
        },
    ];

    neighbor_fields
        .into_iter()
        .filter(|&position| board.check_field(position, vec![FieldState::Unknown]))
        .collect_vec()
}

fn next_attack(board: &Board) -> Vec<Position> {
    let damaged_fields = board.find_fields(vec![FieldState::Damaged], |_| true);

    if damaged_fields.len() == 1 {
        return Vec::new();
    }

    let first_damaged_field = match damaged_fields.first() {
        Some(field) => field,
        None => return Vec::new(),
    };

    let last_damaged_field = match damaged_fields.last() {
        Some(field) => field,
        None => return Vec::new(),
    };

    if first_damaged_field.x != last_damaged_field.x {
        let neighbor_fields = vec![
            Position {
                x: first_damaged_field.x - 1,
                y: first_damaged_field.y,
            },
            Position {
                x: last_damaged_field.x + 1,
                y: last_damaged_field.y,
            },
        ];

        neighbor_fields
            .into_iter()
            .filter(|&position| board.check_field(position, vec![FieldState::Unknown]))
            .collect_vec()
    } else {
        let neighbor_fields = vec![
            Position {
                x: first_damaged_field.x,
                y: first_damaged_field.y - 1,
            },
            Position {
                x: last_damaged_field.x,
                y: last_damaged_field.y + 1,
            },
        ];

        neighbor_fields
            .into_iter()
            .filter(|&position| board.check_field(position, vec![FieldState::Unknown]))
            .collect_vec()
    }
}
