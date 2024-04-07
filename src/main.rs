#![windows_subsystem = "windows"]


use eframe::{egui, App, Frame, CreationContext};
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
    
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Paster");
            ui.separator();
    
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.text));
                    ui.add_space(10.0);

                    if ui.button("Paste").clicked() {
                        thread::sleep(time::Duration::from_secs(3));
                        self.enigo.key_sequence(&self.text);
                    }
                });
            });
        });
    }
}

fn main() {
    let app = TextPaster::new();
    let native_options = eframe::NativeOptions::default();
    let result = eframe::run_native("Text Paster", native_options, Box::new(move |_ctx: &CreationContext<'_>| Box::new(app)));

    result.unwrap_or_else(|e| eprintln!("Fehler beim Ausführen der Anwendung: {:?}", e));
}
