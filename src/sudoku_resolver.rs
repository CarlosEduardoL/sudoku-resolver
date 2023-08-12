use crate::constants::BOARD_SIZE;

/// A struct representing a Sudoku puzzle and its state.
pub struct SudokuResolver {
    /// The current state of the Sudoku puzzle.
    pub sudoku: [[u8; BOARD_SIZE]; BOARD_SIZE],
    /// The currently selected cell in the puzzle.
    pub selected: (usize, usize),
    /// Whether or not to show an invalid move message.
    pub show_invalid_state: bool,
    /// The value of the invalid move.
    pub invalid_move_value: u8,
}

impl Default for SudokuResolver {
    /// Creates a new `SudokuResolver` with default values.
    fn default() -> Self {
        Self {
            sudoku: [[0; BOARD_SIZE]; BOARD_SIZE],
            selected: (usize::MAX, usize::MAX),
            show_invalid_state: false,
            invalid_move_value: 0,
        }
    }
}

impl SudokuResolver {
    /// Solves the Sudoku puzzle using a backtracking algorithm.
    ///
    /// Returns `Some(())` if the puzzle was solved successfully, or `None` if the puzzle could not be solved.
    pub fn solve(&mut self) -> Option<()> {
        // Save the current selected cell
        let selected = self.selected;
        // Find all the cells that need to be filled
        let to_fill = self.sudoku.iter().enumerate().fold(Vec::new(), |mut acc, (x, row)| {
            row.iter().enumerate().for_each(|(y, v)| {
                if *v == 0 {
                    acc.push((x, y));
                }
            });
            acc
        });
        let mut index = 0;
        // Iterate over all the cells that need to be filled
        'out: while index < to_fill.len() {
            // Select the current cell
            self.selected = to_fill[index];
            // Try all possible values for the current cell
            for possibility in (self.get()+1)..=9 {
                // Check if the current possibility is valid
                if self.is_valid_move(possibility) {
                    // Set the value of the current cell
                    self.set(possibility);
                    // Move to the next cell
                    index += 1;
                    continue 'out
                }
            }
            // No valid value was found for the current cell, backtrack
            self.set(0);
            if index == 0 {
                return None
            }
            index -= 1;
        }
        // Restore the original selected cell
        self.selected = selected;
        Some(())
    }

    /// Sets the value of the currently selected cell in the puzzle.
    pub fn set(&mut self, value: u8) {
        self.sudoku[self.selected.0][self.selected.1] = value
    }

    /// Gets the value of the currently selected cell in the puzzle.
    pub fn get(&self) -> u8 {
        self.sudoku[self.selected.0][self.selected.1]
    }

    /// Checks if a given move is valid according to the rules of Sudoku.
    ///
    /// Returns `true` if the move is valid, or `false` otherwise.
    pub fn is_valid_move(&self, value: u8) -> bool {
        let (x, y) = self.selected;
        // Check row
        for i in 0..BOARD_SIZE {
            if self.sudoku[x][i] == value && (x, i) != self.selected {
                return false;
            }
        }
        // Check column
        for i in 0..BOARD_SIZE {
            if self.sudoku[i][y] == value && (i, y) != self.selected {
                return false;
            }
        }
        // Check subgrid
        let subgrid_x = x / 3 * 3;
        let subgrid_y = y / 3 * 3;
        for i in subgrid_x..subgrid_x + 3 {
            for j in subgrid_y..subgrid_y + 3 {
                if self.sudoku[i][j] == value && (i, j) != self.selected {
                    return false;
                }
            }
        }
        true
    }
}