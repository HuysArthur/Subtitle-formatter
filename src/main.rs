#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontId, FontFamily::Proportional, TextStyle::*};
use std::{path::PathBuf, thread};
mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 240.0)),
        ..Default::default()
    };

    let error_message: String;
    let rules: Option<serde_json::Value>;

    if let Ok(rules_json) = utils::read_json_file(PathBuf::from("rules.json")) {
            rules = Some(rules_json);
            error_message = String::from("");
    } else {
        rules = None;
        error_message = String::from("rules.json file not in same folder as the application or json format not correct, fix these problems and reload");
    }

    eframe::run_native(
        "Subtitle formatter",
        options,
        Box::new(|_cc| Box::new(MainApp {
            picked_paths: vec![],
            failed_paths: vec![],
            failed_error_messages: vec![],
            rules,
            error_message,
            working: false,
            done: false
        })),
    )
}

#[derive(Default)]
struct MainApp {
    picked_paths: Vec<PathBuf>,
    failed_paths: Vec<PathBuf>,
    failed_error_messages: Vec<String>,
    rules: Option<serde_json::Value>,
    error_message: String,
    working: bool,
    done: bool
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("start");
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();

            // Redefine text_styles
            style.text_styles = [
            (Heading, FontId::new(48.0, Proportional)),
            (Name("Heading2".into()), FontId::new(36.0, Proportional)),
            (Name("Context".into()), FontId::new(24.0, Proportional)),
            (Body, FontId::new(36.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(48.0, Proportional)),
            (Small, FontId::new(16.0, Proportional)),
            ].into();

            // Mutate global style with above changes
            ctx.set_style(style);

            if self.error_message.is_empty() {
                ui.vertical_centered(|ui| {
                    if !self.working {
                        if ui.button("Choose subtitles").clicked() {
                            for picked_path in rfd::FileDialog::new().add_filter("subtitles", &["srt"]).pick_files().unwrap_or_else(|| vec![]) {
                                self.done = false;
                                self.failed_paths.clear();
                                if !self.picked_paths.contains(&picked_path) {
                                    self.picked_paths.push(picked_path);
                                }
                            }
                        }

                        ui.label(format!("Files selected: {}", self.picked_paths.len()));

                        if ui.button("Format subtitles").clicked() {
                            if self.picked_paths.len() > 0 {
                                self.working = true;
                            }
                        }
                        if self.done {
                            ui.label("Done");

                            if self.failed_paths.len() > 0 {
                                for (index, failed_path) in self.failed_paths.clone().iter().enumerate() {
                                    ui.small(format!("{} ({})", failed_path.file_name().unwrap().to_str().unwrap(), self.failed_error_messages[index]));
                                }
                            }
                        }
                    } else {
                        if self.picked_paths.len() > 0 {
                            ui.label(format!("Files todo: {}", self.picked_paths.len()));

                            let picked_path = self.picked_paths.pop().unwrap();

                            ui.label(format!("Current file: {}", picked_path.display().to_string()));

                            if let Err(err) = utils::format_subtitle(self.rules.clone().unwrap(), picked_path.clone()) {
                                self.failed_error_messages.push(err.to_string());
                                self.failed_paths.push(picked_path);
                            }
                        } else {
                            self.working = false;
                            self.done = true;
                        }
                    }
                });
            } else {
                ui.heading(self.error_message.clone());
            }
        });
        println!("stop");
    }
}
