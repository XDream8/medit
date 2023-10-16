#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use egui_file::FileDialog;

use medit::cat;

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

#[derive(Debug)]
enum EditMode {
    Insert,
    Normal,
}

struct Tabs {
    name: String,
    text: String,
    mode: EditMode,
}

#[derive(Default)]
struct MyApp {
    open_file_dialog: Option<FileDialog>,
    selected_tab: usize,
    tabs: Vec<Tabs>,
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
                                    mode: EditMode::Normal,
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
                        // TODO: editor modes
                        if ui.input(|key| key.key_pressed(egui::Key::I)) {
                            self.tabs[self.selected_tab].mode = EditMode::Insert;
                        }

                        egui::ScrollArea::both().show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::multiline(&mut self.tabs[self.selected_tab].text)
                                    .code_editor(),
                            );
                        });
                    }
                })
            });

            ui.allocate_space(ui.available_size());
        });
    }
}
