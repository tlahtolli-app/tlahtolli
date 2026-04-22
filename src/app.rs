use eframe::egui;

#[derive(Default)]
pub struct TlatolliApp;

impl eframe::App for TlatolliApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tlatolli")
        });        
    }
}