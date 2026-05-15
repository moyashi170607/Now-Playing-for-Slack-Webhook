#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod debug;
mod fetch;

use crate::app::NowPlayingApp;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 250.0])
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Now Playing for Slack Webhook",
        native_options,
        Box::new(|cc| Ok(Box::new(NowPlayingApp::new(cc)))),
    )
    .unwrap();
}
