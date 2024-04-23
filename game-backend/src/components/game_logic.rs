use std::fmt;

mod difficulty {
    pub const EASY: u8 = 35;
    pub const MEDIUM: u8 = 45;
    pub const HARD: u8 = 55;
}

pub struct SudokuGame {
    pub grid: [[Option<u8>; 9]; 9],
    pub difficulty: u8,
}

impl SudokuGame {
    pub fn new(difficulty: u8) -> Self {
        SudokuGame {
            grid: [[None; 9]; 9],
            difficulty,
        }
    }

    pub fn generate_puzzle(&mut self) {
        // Generate a new Sudoku puzzle using the selected difficulty
        // The puzzle generation algorithm is not covered in this example
    }

    pub fn is_valid_input(&self, row: usize, col: usize, num: u8) -> bool {
        // Check if the given input is valid
        // This function should validate the input according to Sudoku rules

        // Check row
        for col_iter in 0..9 {
            if self.grid[row][col_iter].unwrap_or(0) == num {
                return false;
            }
        }

        // Check column
        for row_iter in 0..9 {
            if self.grid[row_iter][col].unwrap_or(0) == num {
                return false;
            }
        }

        // Check 3x3 grid
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for row_iter in start_row..start_row + 3 {
            for col_iter in start_col..start_col + 3 {
                if self.grid[row_iter][col_iter].unwrap_or(0) == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        // Check if the puzzle has been solved correctly
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col].is_none() {
                    return false;
                }
            }
        }

        for row in 0..9 {
            for col in 0..9 {
                if !self.is_valid_input(row, col, self.grid[row][col].unwrap()) {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Debug for SudokuGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(
                    f,
                    "{} ",
                    match *cell {
                        None => ".",
                        Some(n) => format!("{}", n),
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}use std::fmt;

mod difficulty {
    pub const EASY: u8 = 35;
    pub const MEDIUM: u8 = 45;
    pub const HARD: u8 = 55;
}

pub struct SudokuGame {
    pub grid: [[Option<u8>; 9]; 9],
    pub difficulty: u8,
}

impl SudokuGame {
    pub fn new(difficulty: u8) -> Self {
        SudokuGame {
            grid: [[None; 9]; 9],
            difficulty,
        }
    }

    pub fn generate_puzzle(&mut self) {
        // Generate a new Sudoku puzzle using the selected difficulty
        // The puzzle generation algorithm is not covered in this example
    }

    pub fn is_valid_input(&self, row: usize, col: usize, num: u8) -> bool {
        // Check if the given input is valid
        // This function should validate the input according to Sudoku rules

        // Check row
        for col_iter in 0..9 {
            if self.grid[row][col_iter].unwrap_or(0) == num {
                return false;
            }
        }

        // Check column
        for row_iter in 0..9 {
            if self.grid[row_iter][col].unwrap_or(0) == num {
                return false;
            }
        }

        // Check 3x3 grid
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for row_iter in start_row..start_row + 3 {
            for col_iter in start_col..start_col + 3 {
                if self.grid[row_iter][col_iter].unwrap_or(0) == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        // Check if the puzzle has been solved correctly
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col].is_none() {
                    return false;
                }
            }
        }

        for row in 0..9 {
            for col in 0..9 {
                if !self.is_valid_input(row, col, self.grid[row][col].unwrap()) {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Debug for SudokuGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(
                    f,
                    "{} ",
                    match *cell {
                        None => ".",
                        Some(n) => format!("{}", n),
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}