#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..8, 0..8) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || (self.position.rank - other.position.rank).abs()
                == (self.position.file - other.position.file).abs()
        // if self.position.rank == other.position.rank || self.position.file == other.position.file {
        //     return true;
        // }
        // let directions: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
        // for dir in directions {
        //     let mut d = dir;
        //     while ChessPosition::new(d.0 + other.0, d.1 + other.1).is_some() {
        //         if (self.0, self.1) == (other.0 + d.0, other.1 + d.1) {
        //             return true;
        //         }
        //         d = (d.0 + dir.0, d.1 + dir.1);
        //     }
        // }
        // false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn queen_with_a_valid_position() {
        let chess_position = ChessPosition::new(2, 2);
        assert!(chess_position.is_some());
    }

    #[test]
    fn queen_must_have_positive_row() {
        let chess_position = ChessPosition::new(-2, 2);
        assert!(chess_position.is_none());
    }

    #[test]
    fn queen_must_have_row_on_board() {
        let chess_position = ChessPosition::new(8, 4);
        assert!(chess_position.is_none());
    }

    #[test]
    fn queen_must_have_positive_column() {
        let chess_position = ChessPosition::new(2, -2);
        assert!(chess_position.is_none());
    }

    #[test]
    fn queen_must_have_column_on_board() {
        let chess_position = ChessPosition::new(4, 8);
        assert!(chess_position.is_none());
    }

    #[test]
    fn cannot_attack() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
        assert!(!white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_same_row() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_same_column() {
        let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_first_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_second_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(3, 1).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_third_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_fourth_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(1, 7).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 6).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn cannot() {
        let white_queen = Queen::new(ChessPosition::new(4, 1).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
        assert!(!white_queen.can_attack(&black_queen));
    }
}
