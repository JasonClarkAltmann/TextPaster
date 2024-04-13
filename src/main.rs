#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use enigo::{Enigo, KeyboardControllable};
use std::{thread, time};
use clipboard::{ClipboardContext, ClipboardProvider};

use egui::special_emojis::GITHUB;

struct TextPaster {
    text: String,
    enigo: Enigo,
    dark_mode: bool,
    clipboard_texts: Vec<String>,
}

impl TextPaster {
    fn new() -> Self {
        Self {
            text: String::new(),
            enigo: Enigo::new(),
            dark_mode: true,
            clipboard_texts: Vec::new(),
        }
    }

    fn get_clipboard_text(&mut self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        match ctx.get_contents() {
            Ok(contents) => {
                if !self.clipboard_texts.contains(&contents) {
                    self.clipboard_texts.insert(0, contents);
                }
            },
            Err(_) => (),
        }
    }
}

impl App for TextPaster {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.2);

        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TextPaster");

                ui.with_layout(
                    egui::Layout::from_main_dir_and_cross_align(
                        egui::Direction::RightToLeft,
                        egui::Align::RIGHT,
                    ),
                    |ui| {
                        let button_text = if self.dark_mode { "🌙" } else { "🌞" };
                        if ui.button(button_text).clicked() {
                            self.dark_mode = !self.dark_mode;
                        }
                    },
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.text).desired_width(350.0));
                    ui.add_space(10.0);
        
                    if ui.button("Paste").clicked() {
                        thread::sleep(time::Duration::from_secs(3));
                        self.enigo.key_sequence(&self.text);
                    }
                    ui.add_space(10.0);

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.get_clipboard_text();
                        for (i, text) in self.clipboard_texts.iter().enumerate() {
                            if ui.selectable_label(false, text).clicked() {
                                self.text = text.clone();
                            }
                            if i < self.clipboard_texts.len() - 1 {
                                ui.separator();
                            }
                        }
                    });
                });
            });
        
            egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("@JasonClarkAltmann");
                    
                    ui.with_layout(
                        egui::Layout::from_main_dir_and_cross_align(
                            egui::Direction::RightToLeft,
                            egui::Align::RIGHT,
                        ),
                        |ui| {
                            ui.hyperlink_to(
                                format!("{GITHUB} GitHub"),
                                "https://github.com/JasonClarkAltmann/TextPaster",
                            );
                        },
                    );
                });
            });
        });
        
    }
}

fn main() {
    let app = TextPaster::new();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        ..Default::default()
    };
    let result = eframe::run_native(
        "TextPaster",
        native_options,
        Box::new(move |_ctx: &CreationContext<'_>| Box::new(app)),
    );

    result.unwrap_or_else(|e| eprintln!("Fehler beim Ausführen der Anwendung: {:?}", e));
}
