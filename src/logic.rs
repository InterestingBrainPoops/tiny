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
        (you.head + Coord::new(0, 1)).wrap(board.width, board.height),
        (you.head + Coord::new(0, -1)).wrap(board.width, board.height),
        (you.head + Coord::new(-1, 0)).wrap(board.width, board.height),
        (you.head + Coord::new(1, 0)).wrap(board.width, board.height),
    ];

    let mut actual_moves = vec![];
    let mut actual_head_locations = vec![];
    for (idx, head) in possible_head_locations.iter().enumerate() {
        let mut death = false;
        for snake in &board.snakes {
            if snake.body[..(snake.body.len() - 1)].contains(head) {
                death = true;
                println!(
                    "Move {} would have lead to ramming death",
                    possible_moves[idx]
                );
                break;
            }
            if death {
                break;
            }
        }
        if death {
            continue;
        }
        if (head.x < 0 || head.y < 0 || head.y >= board.height || head.x >= board.width)
            || board.hazards.contains(head)
        {
            continue;
        }
        if board.snakes.iter().any(|snake| {
            snake.id != you.id && manhattan(&snake.head, head) == 1 && snake.length >= you.length
        }) {
            continue;
        }
        actual_moves.push(possible_moves[idx]);
        actual_head_locations.push(*head);
    }
    if actual_moves.len() == 1 {
        return actual_moves[0];
    } else if actual_moves.is_empty() {
        return "up";
    }
    println!("Moves :{:?}", actual_moves);
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
    let mut others = vec![];
    for (idx, snake) in board.snakes.iter().enumerate() {
        if snake.id != you.id {
            others.push(idx);
        }
    }
    let mut other_idx = others[0];
    let mut other_distance = 1000;
    for idx in others {
        if manhattan(&board.snakes[idx].head, &you.head) < other_distance {
            other_idx = idx;
            other_distance = manhattan(&board.snakes[idx].head, &you.head);
        }
    }
    let other = board.snakes[other_idx].clone();
    for head in actual_head_locations {
        let mut pos = Position {
            my_health: you.health as u8,
            their_health: other.health as u8,
            board: crate::types::Board {
                snakes: vec![],
                food: vec![],
            },
            all_bb: 0,
        };
        let mut you_body = you.body.clone();
        let mut food = board.food.clone();

        you_body.pop();
        you_body.insert(0, head);
        if food.contains(&you_body[0]) {
            food.remove(food.iter().position(|&x| x == you_body[0]).unwrap());
            pos.my_health = 100;
        }
        pos.board.snakes.push(Snake {
            body: you_body.clone(),
        });
        pos.board.snakes.push(Snake {
            body: other.body.clone(),
        });
        pos.board.food = board.food.clone();
        for snake in &pos.board.snakes {
            if snake.body[0] != you_body[0] {
                for body in &snake.body[..(snake.body.len() - 1)] {
                    pos.all_bb |= u128::from(*body);
                }
            } else {
                for body in &snake.body {
                    pos.all_bb |= u128::from(*body);
                }
            }
        }

        for hazard in &board.hazards {
            pos.all_bb |= u128::from(*hazard);
        }

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
    info!(
        "GAME ID : {} MOVE: {} SCORE: {}",
        game.id, chosen, highest_score
    );

    chosen
}
fn better_sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + E.powf(-value))
}
