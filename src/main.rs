mod queen_placer {

    #[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
    pub struct Row(pub usize);

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub struct Column(pub usize);

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Square {
        pub row: Row,
        pub column: Column,
    }

    impl Square {
        fn new(row: Row, column: Column) -> Self {
            Square { row, column }
        }
    }

    mod queen_placer_impl {
        use super::*;

        pub struct QueenPlacer {
            queen_positions: Vec<Square>,
            size: usize,
        }

        impl QueenPlacer {
            pub fn new(size: usize) -> Self {
                QueenPlacer {
                    queen_positions: Vec::new(),
                    size,
                }
            }

            pub fn place_queens(mut self) -> Vec<Vec<Square>> {
                let mut solutions = Vec::new();
                let mut first_possible_column = Column(0);
                loop {
                    if let Some(row) = self.next_row_for_placement() {
                        if self.attempt_to_place_queen_on_row(row, first_possible_column) {
                            first_possible_column = Column(0);
                            continue;
                        }
                    }
                    if self.found_solution() {
                        self.record_solution(&mut solutions);
                    }
                    match self.backtrack() {
                        Some(column) => first_possible_column = column,
                        None => break,
                    }
                }
                solutions
            }

            fn record_solution(&self, solutions: &mut Vec<Vec<Square>>) {
                assert_eq!(self.queen_positions.len(), self.size);
                solutions.push(self.queen_positions.clone());
            }

            fn found_solution(&self) -> bool {
                self.queen_positions.len() == self.size
            }

            fn next_row_for_placement(&self) -> Option<Row> {
                let rows_filled = self.queen_positions.len();
                if rows_filled == self.size {
                    None
                } else {
                    Some(Row(self.queen_positions.len()))
                }
            }

            fn attempt_to_place_queen_on_row(
                &mut self,
                row: Row,
                first_possible_column: Column,
            ) -> bool {
                for column in first_possible_column.0..self.size {
                    let square = Square::new(row, Column(column));
                    if self.can_place_queen_at(&square) {
                        self.place_queen(&square);
                        return true;
                    }
                }
                false
            }

            fn place_queen(&mut self, square: &Square) {
                assert!(square.row == Row(self.queen_positions.len()));
                assert!(self.can_place_queen_at(square));
                self.queen_positions.push(*square);
            }

            fn can_place_queen_at(&self, square: &Square) -> bool {
                !self
                    .queen_positions
                    .iter()
                    .any(|existing_queen| queens_attack(existing_queen, square))
            }

            fn backtrack(&mut self) -> Option<Column> {
                let last_queen = self.queen_positions.pop()?;
                if last_queen.column == Column(self.size - 1) {
                    self.backtrack()
                } else {
                    Some(Column(last_queen.column.0 + 1))
                }
            }
        }
    }

    fn queens_attack(queen_a: &Square, queen_b: &Square) -> bool {
        if queen_a.row == queen_b.row || queen_a.column == queen_b.column {
            return true;
        }
        (queen_a.row.0 as i32 - queen_b.row.0 as i32).abs()
            == (queen_a.column.0 as i32 - queen_b.column.0 as i32).abs()
    }

    pub fn place_queens(size: usize) -> Vec<Vec<Square>> {
        let placer = queen_placer_impl::QueenPlacer::new(size);
        placer.place_queens()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_queens_attack() {
            let queen_a = Square::new(Row(0), Column(1));
            let queen_b = Square::new(Row(2), Column(3));
            assert!(queens_attack(&queen_a, &queen_b));
        }

        #[test]
        fn test_placing_four_queens() {
            let queen_positions = place_queens(4);
            println!("{queen_positions:#?}");
            assert_eq!(queen_positions.len(), 2);
        }
    }
}

mod leetcode {
    use crate::queen_placer;

    pub fn place_queens(size: i32) -> i32 {
        queen_placer::place_queens(size as usize).len() as i32
    }
}

fn main() {
    let queen_positions = leetcode::place_queens(4);
    println!("{queen_positions:#?}")
}
