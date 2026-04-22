#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod editor;
mod export;
mod project;
mod ui;

rust_i18n::i18n!("locales");
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tlatolli", 
        options,
        Box::new(|_cc| Ok(Box::new(app::TlatolliApp::default()))),
    )
}