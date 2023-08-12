/// The name of the application.
pub const APP_NAME: &str = "Sudoku Resolver";
/// The version of the application.
pub const APP_VERSION: &str = "V0.1.0";
/// The size of the Sudoku board (number of rows and columns).
pub const BOARD_SIZE: usize = 9;
/// The size of each button in the user interface.
pub const BUTTON_SIZE: f32 = 80.;
/// The size of the main window.
pub const WINDOW_SIZE: f32 = BUTTON_SIZE * 10.;
/// The size of the popup windows.
pub const POPUP_SIZE: f32 = 200.;
/// The first background color used for the cells in the Sudoku board.
pub const COLOR1: egui::Color32 = egui::Color32::from_rgb(100, 100, 100);
/// The second background color used for the cells in the Sudoku board.
pub const COLOR2: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);
/// The background color used for the selected cell in the Sudoku board.
pub const COLOR_SELECTED: egui::Color32 = egui::Color32::from_rgb(0, 0, 150);
