#[derive(Clone, Debug, Copy)]
pub enum Colors {
    Blue,
    Green,
    Yellow
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    numbers: [u8; 3],
    position: u8
}

impl Piece {
    pub fn get_number_of_color(self, color: Colors) -> u8 {
        self.numbers[((color as u8) % 3) as usize]
    }

    pub fn get_right_side_number_of_base_color(&self, color: &Colors) -> u8 {
        self.numbers[((color.clone() as u8 + 2) % 3) as usize]
    }

    pub fn get_left_side_number_of_base_color(&self, color: &Colors) -> u8 {
        self.numbers[((color.clone() as u8 + 1) % 3) as usize]
    }
}

#[derive(Debug)]
pub struct GameArea {
    orientation: Colors, // color of lower/upper part of pieces
    pieces: [Piece; 36],
    next_piece_index: u8
}

fn generate_below_of_next_piece_index_rec(prev_base: &u8, curr_base: &u8, index: &u8) -> u8 {
    return if index <= curr_base {
        let r = *index - (*curr_base - *prev_base) + 1;
        r
    } else {
        generate_below_of_next_piece_index_rec(curr_base, &(curr_base + (curr_base - prev_base - 2)), index)
    }
}

impl GameArea {
    pub fn new(orientation: Colors) -> Self {
        Self {
            orientation,
            pieces: Self::get_new_pieces(),
            next_piece_index: 1
        }
    }

    pub fn get_next_piece_index(&self) -> &u8 {
        &self.next_piece_index
    }



    pub fn get_below_of_next_piece_index(&self) -> u8 {
        generate_below_of_next_piece_index_rec(&(0 as u8), &(11 as u8), self.get_next_piece_index())
    }

    pub fn get_orientation(&self) -> &Colors {
        &self.orientation
    }

    pub fn get_new_pieces() -> [Piece; 36] {
        [
            Piece{ numbers: [1,1,4], position: 0},
            Piece{ numbers: [1,2,3], position: 0},
            Piece{ numbers: [1,3,2], position: 0},
            Piece{ numbers: [1,4,1], position: 0},
            Piece{ numbers: [1,5,6], position: 0},
            Piece{ numbers: [1,6,5], position: 0},

            Piece{ numbers: [2,1,3], position: 0},
            Piece{ numbers: [2,2,2], position: 0},
            Piece{ numbers: [2,3,1], position: 0},
            Piece{ numbers: [2,4,6], position: 0},
            Piece{ numbers: [2,5,5], position: 0},
            Piece{ numbers: [2,6,4], position: 0},

            Piece{ numbers: [3,1,2], position: 0},
            Piece{ numbers: [3,2,1], position: 0},
            Piece{ numbers: [3,3,6], position: 0},
            Piece{ numbers: [3,4,5], position: 0},
            Piece{ numbers: [3,5,4], position: 0},
            Piece{ numbers: [3,6,3], position: 0},

            Piece{ numbers: [4,1,1], position: 0},
            Piece{ numbers: [4,2,6], position: 0},
            Piece{ numbers: [4,3,5], position: 0},
            Piece{ numbers: [4,4,4], position: 0},
            Piece{ numbers: [4,5,3], position: 0},
            Piece{ numbers: [4,6,2], position: 0},

            Piece{ numbers: [5,1,6], position: 0},
            Piece{ numbers: [5,2,5], position: 0},
            Piece{ numbers: [5,3,4], position: 0},
            Piece{ numbers: [5,4,3], position: 0},
            Piece{ numbers: [5,5,2], position: 0},
            Piece{ numbers: [5,6,1], position: 0},

            Piece{ numbers: [6,1,5], position: 0},
            Piece{ numbers: [6,2,4], position: 0},
            Piece{ numbers: [6,3,3], position: 0},
            Piece{ numbers: [6,4,2], position: 0},
            Piece{ numbers: [6,5,1], position: 0},
            Piece{ numbers: [6,6,6], position: 0},
        ]
    }

    pub fn set_next_piece(&mut self, new_piece: Piece) {
        let npi = self.get_next_piece_index().clone();
        for p in self.pieces.iter_mut() {
            if *p == new_piece {
                p.position = npi
            }
        }
        self.next_piece_index += 1;
    }

    pub fn get_piece_candidates(&self) -> Vec<Piece> {
        match self.next_piece_index {
            1  => {
                self.pieces.iter().cloned().collect::<Vec<Piece>>()
            },
            3 | 5 | 7 | 9 | 11 => {
                self.get_piece_candidates_match_left_side()
            },
            12 | 21 | 28 | 33 | 36 => {
                self.pieces.iter().cloned().collect::<Vec<Piece>>()
            },
            14 | 16 | 18 | 20 | 23 | 25 | 27 | 30 | 32 | 35 => {
                self.pieces.iter().cloned().collect::<Vec<Piece>>()
            },
            _ => {
                self.get_piece_candidates_match_left_side_upside_down()
            }
        }
    }

    fn get_previous_piece_number_right_side(&self) -> u8 {
        self.pieces
            .iter()
            .filter(|p| p.position == self.get_next_piece_index()-1)
            .cloned()
            .next()
            .unwrap()
            .get_right_side_number_of_base_color(self.get_orientation())
    }

    fn get_previous_piece_number_left_side(&self) -> u8 {
        self.pieces
            .iter()
            .filter(|p| p.position == self.get_next_piece_index()-1)
            .cloned()
            .next()
            .unwrap()
            .get_left_side_number_of_base_color(self.get_orientation())
    }

    pub fn get_piece_candidates_match_left_side_upside_down(&self) -> Vec<Piece> {
        // let c = ;
        self.pieces
            .iter()
            .filter(|p| //p.numbers[c as usize] == 0 &&
                p.position == 0 &&
                p.get_right_side_number_of_base_color(self.get_orientation()) == self.get_previous_piece_number_right_side()
            )
            .cloned()
            .collect::<Vec<Piece>>()
    }

    pub fn get_piece_candidates_match_left_side(&self) -> Vec<Piece> {
        // let c = ;
        self.pieces
            .iter()
            .filter(|p| //p.numbers[c as usize] == 0 &&
                p.position == 0 &&
                p.get_right_side_number_of_base_color(self.get_orientation()) == self.get_previous_piece_number_left_side()
            )
            .cloned()
            .collect::<Vec<Piece>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pieces_have_numbers_between_1_and_6() {
        let g = GameArea::new(Colors::Blue);
        for piece in g.pieces {
            for number in piece.numbers {
                assert!(number >= 1 && number <= 6);
            }
        }
    }

    #[test]
    fn pieces_are_all_unique() {
        let g = GameArea::new(Colors::Blue);
        for piece in &g.pieces {
            assert!(g.pieces.iter().filter(|&p| p.numbers == piece.numbers).count() == 1)
        }
    }

    #[test]
    fn pieces_have_same_number_on_same_position_exactly_six_times() {
        let g = GameArea::new(Colors::Blue);
        for number in 1..=6 {
            format!("number: {}", number);
            for position in 0..=2 {
                assert!(g.pieces.iter().filter(|&p| p.numbers[position] == number).count() == 6)
            }
        }
    }

    #[test]
    fn get_candidates_in_position_two_and_three() {
        let mut g = GameArea::new(Colors::Blue);
        // test position 2
        let mut p = g.pieces[0].clone();
        g.set_next_piece(p);
        let mut c = g.get_piece_candidates();
        assert_eq!(c.iter().count(), 5);

        // test position 3
        p = c[0].clone();
        g.set_next_piece(p);
        c = g.get_piece_candidates();
        assert_eq!(c.iter().count(), 6);
    }

    #[test]
    fn get_candidates_in_position_twelve() {
        let mut g = GameArea::new(Colors::Blue);
        // [1,2,3]
        g.pieces[1].position = 2;
        g.next_piece_index = 12;



    }

    #[test]
    fn get_below_of_next_piece_index_all_lines() {
        let mut g = GameArea::new(Colors::Blue);

        g.next_piece_index = 2;
        assert_eq!(g.get_below_of_next_piece_index(), 0 as u8);

        g.next_piece_index = 11;
        assert_eq!(g.get_below_of_next_piece_index(), 0 as u8);

        g.next_piece_index = 12;
        assert_eq!(g.get_below_of_next_piece_index(), 2 as u8);

        g.next_piece_index = 20;
        assert_eq!(g.get_below_of_next_piece_index(), 10 as u8);

        g.next_piece_index = 21;
        assert_eq!(g.get_below_of_next_piece_index(), 13 as u8);

        g.next_piece_index = 27;
        assert_eq!(g.get_below_of_next_piece_index(), 19 as u8);

        g.next_piece_index = 28;
        assert_eq!(g.get_below_of_next_piece_index(), 22 as u8);

        g.next_piece_index = 32;
        assert_eq!(g.get_below_of_next_piece_index(), 26 as u8);

        g.next_piece_index = 33;
        assert_eq!(g.get_below_of_next_piece_index(), 29 as u8);

        g.next_piece_index = 35;
        assert_eq!(g.get_below_of_next_piece_index(), 31 as u8);

        g.next_piece_index = 36;
        assert_eq!(g.get_below_of_next_piece_index(), 34 as u8);
    }

    #[test]
    fn piece_get_number_of_specific_color() {
        let p = Piece{numbers: [0, 1, 2], position: 0};
        assert!(p.get_right_side_number_of_base_color(&Colors::Blue) == 2);
        assert!(p.get_right_side_number_of_base_color(&Colors::Green) == 0);
        assert!(p.get_right_side_number_of_base_color(&Colors::Yellow) == 1);
    }
}