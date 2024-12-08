use core::panic;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Res {
    Win,
    Draw,
    Lose,
}

impl Shape {
    const SHAPES: [Shape; 3] = [Shape::Rock, Shape::Paper, Shape::Scissors];
    fn offset(&self, offset: usize) -> Shape {
        let mut iter = Shape::SHAPES.into_iter().cycle();

        let offset = offset % Shape::SHAPES.len();
        if offset == 0 {
            return *self;
        }

        while *self != iter.next().unwrap() {}

        for _ in 0..offset - 1 {
            iter.next();
        }
        iter.next().unwrap()
    }
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

fn into_res(res: &str) -> Res {
    match res {
        "X" => Res::Lose,
        "Y" => Res::Draw,
        "Z" => Res::Win,
        _ => panic!("No way!"),
    }
}

fn guess_shape(other: &Shape, res: &Res) -> Shape {
    match (res, other) {
        (Res::Draw, &shape) => shape,
        (Res::Win, _) => other.offset(1),
        _ => other.offset(2),
    }
}

pub fn solve(input: &str) -> i32 {
    let rounds = input.lines();
    let mut total_score = 0;
    for round in rounds {
        let (other, res) = round.split_once(" ").expect("Nooooo way");
        let their_shape = into_shape(other);
        let res = into_res(res);
        let my_shape = guess_shape(&their_shape, &res);
        println!("mine: {my_shape:?}, theirs: {their_shape:?}, res: {res:?}");
        total_score += get_score(&my_shape, &res);
    }
    total_score
}
