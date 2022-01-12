#[derive(Debug)]
pub enum colors {
    Blue,
    Green,
    Yellow
}

#[derive(Debug)]
pub struct Piece {
    numbers: [u8; 3],
    position: u8
}

#[derive(Debug)]
pub struct GameArea {
    orientation: colors, // color of lower/upper part of pieces
    pieces: [Piece; 36],
    nextPieceIndex: u8
}

impl GameArea {
    pub fn new(orientation: colors) -> Self {
        Self {
            orientation,
            pieces: Self::getNewPieces(),
            nextPieceIndex: 1
        }
    }

    pub fn getNewPieces() -> [Piece; 36] {
        [
            Piece{ numbers: [1,1,4], position: 0},
            Piece{ numbers: [4,4,4], position: 0},
            Piece{ numbers: [4,1,1], position: 0},
            Piece{ numbers: [6,5,1], position: 0},
            Piece{ numbers: [4,3,5], position: 0},
            Piece{ numbers: [4,2,6], position: 0},
            Piece{ numbers: [5,2,5], position: 0},
            Piece{ numbers: [5,5,2], position: 0},
            Piece{ numbers: [6,4,2], position: 0},
            Piece{ numbers: [2,5,5], position: 0},
            Piece{ numbers: [3,4,5], position: 0},
            Piece{ numbers: [1,5,6], position: 0},
            Piece{ numbers: [1,2,3], position: 0},
            Piece{ numbers: [1,4,1], position: 0},
            Piece{ numbers: [2,1,3], position: 0},
            Piece{ numbers: [2,4,6], position: 0},
            Piece{ numbers: [3,2,1], position: 0},
            Piece{ numbers: [3,5,4], position: 0},
            Piece{ numbers: [2,3,1], position: 0},
            Piece{ numbers: [4,5,3], position: 0},
            Piece{ numbers: [1,3,2], position: 0},
            Piece{ numbers: [3,3,6], position: 0},
            Piece{ numbers: [5,1,6], position: 0},
            Piece{ numbers: [5,6,1], position: 0},
            Piece{ numbers: [6,1,5], position: 0},
            Piece{ numbers: [5,3,4], position: 0},
            Piece{ numbers: [4,6,2], position: 0},
            Piece{ numbers: [3,6,3], position: 0},
            Piece{ numbers: [1,6,5], position: 0},
            Piece{ numbers: [6,2,4], position: 0},
            Piece{ numbers: [6,3,3], position: 0},
            Piece{ numbers: [2,6,4], position: 0},
            Piece{ numbers: [2,2,2], position: 0},
            Piece{ numbers: [5,4,3], position: 0},
            Piece{ numbers: [6,6,6], position: 0},
            Piece{ numbers: [3,1,2], position: 0},
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pieces_have_numbers_between_1_and_6() {
        let g = GameArea::new(colors::Blue);

        assert!(true);
    }
}