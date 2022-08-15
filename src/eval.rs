use pathfinding::prelude::astar;

use crate::{
    types::{Board, Position, Snake, NUM_PARAMS},
    Battlesnake, Coord,
};

// this is always from the perspective of the first snake (hacky fix, but it works)
pub fn score(position: &Position) -> [f64; NUM_PARAMS] {
    // me
    let me = position.board.snakes[0].clone();
    // them
    let other = position.board.snakes[1].clone();
    // the length difference between me and them
    let length_difference = (me.body.len() - other.body.len()) as i32;
    // my distance to center - their distance to center
    let distance_to_center = distance_to_center(me, other);
    // my heatlh - their health
    let health_diff = (position.my_health - position.their_health) as i32;
    // my nearest food
    let mut my_nearest = 0;
    // their nearest food
    let mut their_nearest = 0;
    food_vornoi(position, &mut my_nearest, &mut their_nearest);
    // my_nearest foods - their_nearest-foods
    let food_ownership_difference = (my_nearest - their_nearest);
    // my owned squares
    let mut my_squares = 0;
    // their owned squares
    let mut their_squares = 0;
    // calcualte ownership using vornoi
    square_voronoi(position, &mut my_squares, &mut their_squares);
    // the difference between the owned squares
    let square_ownership_difference = (my_squares - their_squares);

    let target_distance = target_finder(length_difference, position);

    let mut distance_to_other = // my path to the food
    {let my_path = astar(
        &position.board.snakes[0].body[0],
        |p| successors(p, &position.board, position.all_bb),
        |p| manhattan(p, &position.board.snakes[1].body[0]),
        |p| *p == position.board.snakes[1].body[0],
    );

    // my distance to the food
    match my_path {
        None => 0,                  // if i have no path, set the path length to 1k
        Some((path, _)) => path.len(), // otherwise set it to the length of the path
    }} as i32;

    distance_to_other *= if length_difference > 0 { 1 } else { -1 };
    [
        length_difference as f64,
        distance_to_center as f64,
        health_diff as f64,
        food_ownership_difference as f64,
        square_ownership_difference as f64,
        target_distance as f64,
        distance_to_other as f64,
    ]
}

fn target_finder(length_difference: i32, position: &Position) -> usize {
    if length_difference > 0 {
        // my path to the food
        let my_path = astar(
            &position.board.snakes[0].body[0],
            |p| successors(p, &position.board, position.all_bb),
            |p| manhattan(p, &position.board.snakes[1].body[0]),
            |p| *p == position.board.snakes[1].body[0],
        );

        // my distance to the food
        match my_path {
            None => 1000,                  // if i have no path, set the path length to 1k
            Some((path, _)) => path.len(), // otherwise set it to the length of the path
        }
    } else {
        let mut nearest_food_distance = 500;
        let mut nearest_food = Coord::new(1000, 100);
        for food in &position.board.food {
            // my path to the food
            let my_path = astar(
                &position.board.snakes[0].body[0],
                |p| successors(p, &position.board, position.all_bb),
                |p| manhattan(p, food),
                |p| *p == *food,
            );

            // my distance to the food
            let my_dist = match my_path {
                None => 1000,                  // if i have no path, set the path length to 1k
                Some((path, _)) => path.len(), // otherwise set it to the length of the path
            };

            if my_dist < nearest_food_distance {
                nearest_food_distance = my_dist;
                nearest_food = *food;
            }
        }
        nearest_food_distance
    }
}

fn distance_to_center(me: Snake, other: Snake) -> i32 {
    (manhattan(&me.body[0], &Coord::new(6, 6)) - manhattan(&other.body[0], &Coord::new(6, 6)))
}

fn food_vornoi(position: &Position, my_nearest: &mut i32, their_nearest: &mut i32) {
    for food in &position.board.food {
        // my path to the food
        let my_path = astar(
            &position.board.snakes[0].body[0],
            |p| successors(p, &position.board, position.all_bb),
            |p| manhattan(p, food),
            |p| *p == *food,
        );
        // their path to the same food
        let their_path = astar(
            &position.board.snakes[1].body[0],
            |p| successors(p, &position.board, position.all_bb),
            |p| manhattan(p, food),
            |p| *p == *food,
        );
        // my distance to the food
        let my_dist = match my_path {
            None => 1000,                  // if i have no path, set the path length to 1k
            Some((path, _)) => path.len(), // otherwise set it to the length of the path
        };
        // their distance to the food
        let their_dist = match their_path {
            None => 1000,                  // if they have no path, set the path length to 1k
            Some((path, _)) => path.len(), // otherwise set it to the length of the path
        };
        // give credit based on whose path is shorter
        if my_dist < their_dist {
            // if my path is shorter, then credit me
            *my_nearest += 1;
        } else {
            // if their path is shorter, then credit them
            *their_nearest += 1;
        }
    }
}

fn square_voronoi(position: &Position, my_squares: &mut i32, their_squares: &mut i32) {
    for x in 0..11 {
        for y in 0..11 {
            // the curent Coord
            let thing = &Coord::new(x, y);
            // if the square is in either persons body, ignore it
            if position.board.snakes[0].body.contains(thing)
                || position.board.snakes[1].body.contains(thing)
            {
                continue;
            }

            // my path to the square
            let my_path = astar(
                &position.board.snakes[0].body[0],
                |p| successors(p, &position.board, position.all_bb),
                |p| manhattan(p, thing),
                |p| *p == *thing,
            );

            // their path to the square
            let their_path = astar(
                &position.board.snakes[1].body[0],
                |p| successors(p, &position.board, position.all_bb),
                |p| manhattan(p, thing),
                |p| *p == *thing,
            );
            // my distance to the square
            let my_dist = match my_path {
                None => 1000,                  // if i dont have a path, set it to 1k
                Some((path, _)) => path.len(), // if i do, set it to the length of the path
            };
            let their_dist = match their_path {
                None => 1000,                  // if they dont have a path, set it to 1k
                Some((path, _)) => path.len(), // if they do, set to the length of the path
            };
            // credit based on who is closer
            if my_dist < their_dist {
                *my_squares += 1;
            } else {
                *their_squares += 1;
            }
        }
    }
}

// successors for a given Coord
fn successors(coord: &Coord, board: &Board, bb: u128) -> Vec<(Coord, i32)> {
    // possible successors
    let possible = [
        Coord::new(0, 1),
        Coord::new(0, -1),
        Coord::new(-1, 0),
        Coord::new(1, 0),
    ];

    // possible ending squares
    let mut new_possible = vec![];
    for thing in &possible {
        new_possible.push(*thing + *coord);
    }

    let mut out = vec![];
    // go through each possible end square
    for thing in &new_possible {
        // if im oob, dont include it
        if thing.x < 0 || thing.x > 10 || thing.y < 0 || thing.y > 10 {
            continue;
        }
        // if the bitboard has this, then dont include it
        if bb & u128::from(*thing) != 0 {
            continue;
        }
        // add it to the out vector
        out.push(*thing);
    }
    // add a weight to each one
    out.iter().map(|p| (*p, 1)).collect()
}

// manhattan distance between two Coords
pub fn manhattan(c1: &Coord, c2: &Coord) -> i32 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}
