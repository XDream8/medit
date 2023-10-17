#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
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

            Box::<Medit>::default()
        }),
    )
}

#[derive(Clone, Debug, PartialEq)]
enum EditMode {
    Insert,
    Normal,
}

#[derive(Clone, PartialEq)]
struct Tab {
    name: String,
    text: String,
    mode: EditMode,
    markdown_view: bool,
}

#[derive(Default)]
struct Medit {
    open_file_dialog: Option<FileDialog>,
    selected_tab: usize,
    tabs: Vec<Tab>,
}

impl eframe::App for Medit {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(24.0);

            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Markdown View").clicked() && !self.tabs.is_empty() {
                        self.tabs[self.selected_tab].markdown_view =
                            !self.tabs[self.selected_tab].markdown_view;
                    }
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
                                let tab: Tab = Tab {
                                    name: file_name,
                                    text,
                                    mode: EditMode::Normal,
                                    markdown_view: false,
                                };
                                // open tab
                                new_tab(tab, self);
                            }
                        }
                    }

                    // Handle tabs
                    for (tab_id, mut tab) in self.tabs.clone().iter().enumerate() {
                        ui.group(|ui| {
                            let button = ui.button(&tab.name);
                            let close_button = ui.button("X");
                            // close tab
                            if button.clicked() {
                                self.selected_tab = tab_id;
                            } else if button.secondary_clicked() || close_button.clicked() {
                                close_tab(&mut tab, self);
                            }
                        });
                    }
                })
            });

            // text edit
            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    if !self.tabs.is_empty() {
                        let tab = &mut self.tabs[self.selected_tab];
                        // TODO: editor modes
                        if ui.input(|key| key.key_pressed(egui::Key::I)) {
                            tab.mode = EditMode::Insert;
                        }

                        CodeEditor::default()
                            .id_source("code editor")
                            .with_rows(12)
                            .with_fontsize(14.0)
                            .with_theme(ColorTheme::GRUVBOX)
                            .with_syntax(Syntax::rust())
                            .with_numlines(true)
                            .show(ui, &mut tab.text);

                        if tab.markdown_view {
                            // Convert Markdown to HTML
                            let parser =
                                pulldown_cmark::Parser::new(&self.tabs[self.selected_tab].text);
                            let mut html = String::new();
                            pulldown_cmark::html::push_html(&mut html, parser);
                            ui.add(egui::Label::new(&html));
                        }
                    };
                })
            });

            ui.allocate_space(ui.available_size());
        });
    }
}

fn new_tab(tab: Tab, info: &mut Medit) {
    info.tabs.push(tab);
}

fn close_tab(tab: &Tab, info: &mut Medit) {
    if info.tabs.len() > 1 && info.selected_tab != 0 {
        info.selected_tab -= 1;
    } else {
        info.selected_tab = 0;
    }
    info.tabs.retain(|x| x != tab)
}
