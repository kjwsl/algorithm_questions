use core::panic;

#[derive(Debug, PartialEq, PartialOrd)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Res {
    Win,
    Draw,
    Lose,
}

fn get_score(shape: &Shape, result: &Res) -> i32 {
    let shape_point = match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let score = match result {
        Res::Win => 6,
        Res::Draw => 3,
        Res::Lose => 0,
    };
    score + shape_point
}

fn into_shape(shape: &str) -> Shape {
    match shape {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("No way!"),
    }
}

fn evaluate_round(me: &Shape, other: &Shape) -> Res {
    match (me, other) {
        (Shape::Scissors, Shape::Paper)
        | (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock) => Res::Win,
        _ => {
            if me == other {
                Res::Draw
            } else {
                Res::Lose
            }
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let rounds = input.lines();
    let mut total_score = 0;
    for round in rounds {
        let (other, me) = round.split_once(" ").expect("Nooooo way");
        let my_shape = into_shape(me);
        let their_shape = into_shape(other);

        let res = evaluate_round(&my_shape, &their_shape);
        total_score += get_score(&my_shape, &res);
    }
    total_score
}
