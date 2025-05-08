mod app;

use app::{AREA_HEIGHT, AREA_MARGIN, AREA_WIDTH, AnnealApp, BOTTOM_PLOT_HEIGHT, SIDE_PANEL_WIDTH};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size((
        AREA_WIDTH + AREA_MARGIN * 2. + SIDE_PANEL_WIDTH,
        AREA_HEIGHT + AREA_MARGIN * 2. + BOTTOM_PLOT_HEIGHT,
    ));

    eframe::run_native(
        "rust-anneal",
        native_options,
        Box::new(|_cc| Ok(Box::new(AnnealApp::new()))),
    )
    .unwrap();
}
