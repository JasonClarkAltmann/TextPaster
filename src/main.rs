#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use enigo::{Enigo, KeyboardControllable};
use std::{thread, time};

struct TextPaster {
    text: String,
    enigo: Enigo,
}

impl TextPaster {
    fn new() -> Self {
        Self {
            text: String::new(),
            enigo: Enigo::new(),
        }
    }
}

impl App for TextPaster {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.2);
        let dark_visuals = egui::Visuals::dark();
        ctx.set_visuals(dark_visuals);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Paster");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.text).desired_width(350.0));
                    ui.add_space(10.0);

                    if ui.button("Paste").clicked() {
                        thread::sleep(time::Duration::from_secs(3));
                        self.enigo.key_sequence(&self.text);
                    }
                    ui.add_space(10.0);
                });
            });

            egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Jason Clark Altmann");
                    ui.add_space(5.0);
                    ui.label("-");
                    ui.add_space(5.0);

                    use egui::special_emojis::GITHUB;
                    ui.hyperlink_to(
                        format!("{GITHUB} GitHub"),
                        "https://github.com/JasonClarkAltmann/text-paster",
                    );
                });
            });
        });
    }
}

fn main() {
    let app = TextPaster::new();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 200.0)),
        ..Default::default()
    };
    let result = eframe::run_native(
        "Text Paster",
        native_options,
        Box::new(move |_ctx: &CreationContext<'_>| Box::new(app)),
    );

    result.unwrap_or_else(|e| eprintln!("Fehler beim Ausführen der Anwendung: {:?}", e));
}
