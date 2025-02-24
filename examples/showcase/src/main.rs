use eframe::egui::{CentralPanel, ViewportBuilder};
use egui_theme_switcher::{Dimension, theme_switcher_with_dimension};

fn main() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        viewport: ViewportBuilder::default(),
        ..Default::default()
    };

    let app = App {};

    eframe::run_native(
        "Show Case Switcher",
        opts,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Selected: {:?}", ctx.theme()));
            ui.horizontal(|ui| {
                ui.label("S");
                ui.add(theme_switcher_with_dimension(Dimension::S));
            });
            ui.horizontal(|ui| {
                ui.label("M");
                ui.add(theme_switcher_with_dimension(Dimension::M));
            });
            ui.horizontal(|ui| {
                ui.label("L");
                ui.add(theme_switcher_with_dimension(Dimension::L));
            });
            ui.horizontal(|ui| {
                ui.label("XL");
                ui.add(theme_switcher_with_dimension(Dimension::XL));
            });
        });
    }
}
