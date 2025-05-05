mod app;

use app::AnnealApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    // native_options.initial_window_size = Some(vec2(
    //     (BOARD_SIZE * CELL_SIZE + 16) as f32,
    //     (BOARD_SIZE * CELL_SIZE + 16) as f32,
    // ));

    eframe::run_native(
        "rust-anneal",
        native_options,
        Box::new(|_cc| Ok(Box::new(AnnealApp::new()))),
    )
    .unwrap();
}
