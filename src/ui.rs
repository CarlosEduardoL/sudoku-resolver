use eframe::{egui, Frame};
use egui::{Context, vec2, Widget};
use crate::constants::{APP_NAME, APP_VERSION, BOARD_SIZE, BUTTON_SIZE, COLOR1, COLOR2, COLOR_SELECTED, POPUP_SIZE};
use crate::sudoku_resolver::SudokuResolver;

impl eframe::App for SudokuResolver {
    /// Updates the state of the Sudoku puzzle and its user interface.
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Show the header panel
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.set_enabled(!self.show_invalid_state);
            ui.heading(format!("{} {}", APP_NAME, APP_VERSION));
        });
        // Show the central panel with the Sudoku board
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!self.show_invalid_state);
            egui::Grid::new("board").show(ui, |ui| {
                for x in 0..self.sudoku.len() {
                    for y in 0..self.sudoku[x].len() {
                        let color = if self.selected == (x, y) { COLOR_SELECTED } else if (x / 3 + y / 3) % 2 == 0 { COLOR1 } else { COLOR2 };
                        let button = egui::Button::new(egui::RichText::new(
                            if self.sudoku[x][y] != 0 { format!("{}", self.sudoku[x][y]) } else { "".to_string() }).size(24.)
                        )
                            .min_size(vec2(BUTTON_SIZE, BUTTON_SIZE))
                            .fill(color);
                        if button.ui(ui).clicked() {
                            self.selected = (x, y);
                        }
                    }
                    ui.end_row();
                }
            });
        });
        // Show the footer panel with the control buttons
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.set_enabled(!self.show_invalid_state);
            ui.horizontal(|ui| {
                if ui.button("Solve!").clicked() {
                    if self.solve().is_none() {
                        egui::Window::new("Impossible sudoku")
                            .fixed_size(vec2(POPUP_SIZE, POPUP_SIZE))
                            .show(ctx, |ui| {
                                ui.label(egui::RichText::new("The provided Sudoku is impossible to solve".to_string()).size(24.));
                                if ui.button("OK").clicked() {
                                    self.show_invalid_state = false;
                                }
                            });
                    }
                }
                if ui.button("Delete").clicked() {
                    self.set(0);
                }
                if ui.button("Clear!").clicked() {
                    self.sudoku = [[0; BOARD_SIZE]; BOARD_SIZE];
                }
            });
        });
        // Handle keyboard input
        if let Some(value) = get_pressed_number(ctx) {
            if self.is_valid_move(value) {
                self.set(value);
            } else {
                self.show_invalid_state = true;
                self.invalid_move_value = value;
            }
        }
        // Show the invalid move popup window
        if self.show_invalid_state {
            let (x, y) = self.selected;
            egui::Window::new("Invalid move")
                .fixed_size(vec2(POPUP_SIZE, POPUP_SIZE))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(format!(
                        "Invalid move: {} at ({}, {})",
                        self.invalid_move_value,
                        x + 1,
                        y + 1
                    )).size(24.));
                    if ui.button("OK").clicked() {
                        self.show_invalid_state = false;
                    }
                });
        }
    }
}

/// Returns the number pressed on the keyboard as an `Option<u8>`.
///
/// If no number key is pressed, returns `None`.
fn get_pressed_number(ctx: &Context) -> Option<u8> {
    ctx.input(|is| {
        if is.keys_down.contains(&egui::Key::Num1) {
            Some(1)
        } else if is.keys_down.contains(&egui::Key::Num2) {
            Some(2)
        } else if is.keys_down.contains(&egui::Key::Num3) {
            Some(3)
        } else if is.keys_down.contains(&egui::Key::Num4) {
            Some(4)
        } else if is.keys_down.contains(&egui::Key::Num5) {
            Some(5)
        } else if is.keys_down.contains(&egui::Key::Num6) {
            Some(6)
        } else if is.keys_down.contains(&egui::Key::Num7) {
            Some(7)
        } else if is.keys_down.contains(&egui::Key::Num8) {
            Some(8)
        } else if is.keys_down.contains(&egui::Key::Num9) {
            Some(9)
        } else {
            None
        }
    })
}
