use rand::seq::SliceRandom;
use rocket::tokio::time::Instant;
use serde_json::{json, Value};
use std::{collections::HashMap, f64::consts::E};

use log::info;

use crate::{
    eval::{self, manhattan},
    types::{Position, Snake},
    Battlesnake, Board, Coord, Game,
};

pub fn get_info() -> Value {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "BrokenKeyboard",
        "color": "#54764B",
        "head": "default",
        "tail": "default",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, you: &Battlesnake) -> &'static str {
    let possible_moves = vec!["up", "down", "left", "right"];
    let possible_head_locations = vec![
        you.head + Coord::new(0, 1),
        you.head + Coord::new(0, -1),
        you.head + Coord::new(-1, 0),
        you.head + Coord::new(1, 0),
    ];

    let mut values = vec![];
    let weights = [
        0.02887396891287721,
        -0.024065560310748118,
        -0.00024165487017368143,
        0.038723174814708515,
        0.00470536898375267,
        0.0006659711268774564,
        0.0,
    ];
    let other = if board.snakes[0].id != you.id {
        board.snakes[0].clone()
    } else {
        board.snakes[1].clone()
    };
    for head in possible_head_locations {
        let mut pos = Position {
            my_health: you.health as u8,
            their_health: other.health as u8,
            board: crate::types::Board {
                snakes: vec![],
                food: vec![],
            },
            all_bb: 0,
        };
        let t0 = Instant::now();
        let output = eval::score(&pos);
        let score: f64 = output
            .iter()
            .enumerate()
            .map(|(idx, x)| x * weights[idx])
            .sum();
        println!("Time taken for 1 eval: {:?}", Instant::now() - t0);
        values.push(better_sigmoid(score));
    }

    let mut highest_idx = 0;
    let mut highest_score = 0.0;
    for (idx, val) in values.iter().enumerate() {
        if *val > highest_score {
            highest_idx = idx;
            highest_score = *val;
        }
    }
    let chosen = actual_moves[highest_idx];
    info!("{} MOVE {} SCORE {}", game.id, chosen, highest_score);

    chosen
}
fn better_sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + E.powf(-value))
}
