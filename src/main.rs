enum Direction {
    Left,
    Right,
    Up,
    Down
}


struct Arrow {
    position: (u8, u8),
    direction: Direction
}


struct GameBoard {
    board: Vec<Vec<char>>,
    arrows: Vec<Arrow>,
}

impl GameBoard {
    pub fn new(width: Option<usize>, height: Option<usize>) -> Self {

        let mut arrows: Vec<Arrow> = Vec::new();
        arrows.push(Arrow{ position: (0, 0), direction: Direction::Right});
        return GameBoard { board: vec![["."; height.unwrap_or(10)]; width.unwrap_or(10)], arrows: Vec::new()};
    }

    pub fn next(self: &mut Self) {
        for arrow in self.arrows {

            
        }

    }

    fn create_arrow(self: &mut Self, position: (u8, u8), direction: Direction) -> Arrow {
        let new_arrow = Arrow{position, direction};
        return new_arrow;
    }

    pub fn redirect_arrow(arrow: &mut Arrow, direction: Direction) {
        arrow.direction = direction;
    }

}

fn main() {
    let board = GameBoard::new();
}
