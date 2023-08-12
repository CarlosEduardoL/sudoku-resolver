use egui::vec2;
use crate::constants::{APP_NAME, WINDOW_SIZE};
use crate::sudoku_resolver::SudokuResolver;

mod constants;
mod sudoku_resolver;
mod ui;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(WINDOW_SIZE, WINDOW_SIZE)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_| Box::<SudokuResolver>::default()),
    )
}
