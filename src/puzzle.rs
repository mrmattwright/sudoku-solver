pub struct Puzzle {
    grid: [[i32; 9]; 9], // The Sudoku grid is a 9x9 array of u32 values
}

impl Puzzle {
    // Constructor to initialize a new Puzzle instance
    pub fn new(grid: [[i32; 9]; 9]) -> Self {
        Puzzle { grid }
    }

    pub fn is_valid(&self) -> bool {
        return true;
    }

    pub fn validate_line(numbers: [i32; 9]) -> Result<(), String> {
        use std::collections::HashSet;

        let mut seen: HashSet<i32> = HashSet::new();

        for &num in numbers.iter() {
            if num < 0 || num > 9 {
                return Err(format!("Number out of Range 0-9: {}", num));
            }

            if num != 0 {
                if seen.contains(&num) {
                    return Err(format!("Number: {} appears more than once", num));
                }
                seen.insert(num);
            }
        }

        Ok(())
    }

    // Additional methods can be added here, such as methods to solve the puzzle, check validity, etc.
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_row() {
        let row: [i32; 9] = [0, 7, 0, 0, 0, 0, 4, 3, 0];
        let result = Puzzle::validate_line(row);
        assert_eq!(result.is_ok(), true);

        let row: [i32; 9] = [7, 7, 0, 0, 0, 0, 4, 3, 0];
        let result = Puzzle::validate_line(row);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), "Number: 7 appears more than once");
    }

    #[test]
    fn initialize_grid() {
        let grid: [[i32; 9]; 9] = [
            [0, 7, 0, 0, 0, 0, 4, 3, 0],
            [4, 0, 0, 0, 0, 9, 6, 1, 0],
            [8, 0, 0, 6, 3, 4, 9, 0, 0],
            [0, 9, 4, 0, 5, 2, 0, 0, 0],
            [3, 5, 8, 4, 6, 0, 0, 2, 0],
            [0, 0, 0, 8, 0, 0, 5, 3, 0],
            [0, 8, 0, 0, 7, 0, 0, 9, 1],
            [9, 0, 2, 1, 0, 0, 0, 0, 5],
            [0, 0, 7, 0, 4, 0, 8, 0, 2],
        ];

        // initalize the puzzle class
        let puzzle = Puzzle::new(grid);

        // assert the puzzle is valid
        assert_eq!(puzzle.is_valid(), true);
    }
}
