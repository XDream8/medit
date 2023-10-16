#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use egui_file::FileDialog;

use std::fs;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Markdown Editor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct Tabs {
    name: String,
    text: String,
}

struct MyApp {
    open_file_dialog: Option<FileDialog>,
    selected_tab: usize,
    tabs: Vec<Tabs>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            open_file_dialog: None,
            selected_tab: 0,
            tabs: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(16.0);

            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Open file…").clicked() {
                        let mut dialog = FileDialog::open_file(None);
                        dialog.open();
                        self.open_file_dialog = Some(dialog);
                    }
                    if let Some(dialog) = &mut self.open_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                let text: String = cat(file).unwrap();
                                let file_name: String =
                                    file.file_name().unwrap().to_string_lossy().to_string();
                                self.tabs.push(Tabs {
                                    name: file_name,
                                    text,
                                });
                            }
                        }
                    }

                    for (tab_id, tab) in self.tabs.iter().enumerate() {
                        if ui.add(egui::Button::new(&tab.name)).clicked() {
                            self.selected_tab = tab_id;
                        }
                    }
                })
            });

            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    if !self.tabs.is_empty() {
                        egui::ScrollArea::both().show(ui, |ui| {
                            ui.add(egui::TextEdit::multiline(
                                &mut self.tabs[self.selected_tab].text,
                            ));
                        });
                    }
                })
            });

            ui.allocate_space(ui.available_size());
        });
    }
}

#[inline]
pub fn cat(path: &Path) -> Result<String, std::io::Error> {
    let file_bytes: Vec<u8> = fs::read(path)?;
    let buffer: String = String::from_utf8(file_bytes).unwrap_or(String::new());

    Ok(buffer)
}
