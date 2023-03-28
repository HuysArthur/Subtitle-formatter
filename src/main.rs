#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontId, FontFamily::Proportional, TextStyle::*};
use std::{path::PathBuf, io};
mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 240.0)),
        ..Default::default()
    };

    let error_message: String;
    let rules: Option<serde_json::Value>;

    if let Ok(content) = &utils::read_content_file(PathBuf::from("rules.json")) {
        if let Ok(rules_json) = serde_json::from_str(&content) {
            rules = rules_json;
            error_message = String::from("");
        } else {
            rules = None;
            error_message = String::from("rules.json format is not correct (no valid json)");
        }
    } else {
        rules = None;
        error_message = String::from("rules.json file not found, place this file in the same folder as the application and reload");
    }

    eframe::run_native(
        "Subtitle formatter",
        options,
        Box::new(|_cc| Box::new(MainApp {
            picked_paths: vec![],
            rules,
            error_message,
            done: false
        })),
    )

    
}

#[derive(Default)]
struct MainApp {
    picked_paths: Vec<PathBuf>,
    rules: Option<serde_json::Value>,
    error_message: String,
    done: bool
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();

            // Redefine text_styles
            style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(36.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(48.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
            ].into();

            // Mutate global style with above changes
            ctx.set_style(style);

            if self.error_message.is_empty() {
                ui.vertical_centered(|ui| {
                    if ui.button("Choose subtitles").clicked() {
                        for picked_path in rfd::FileDialog::new().add_filter("subtitles", &["srt"]).pick_files().unwrap_or_else(|| vec![]) {
                            self.done = false;
                            if !self.picked_paths.contains(&picked_path) {
                                self.picked_paths.push(picked_path);
                            }
                        }
                    }
    
                    ui.label(format!("Files selected: {}", self.picked_paths.len()));
    
                    if ui.button("Format subtitles").clicked() {
                        while self.picked_paths.len() > 0 {                        
                            let picked_path = self.picked_paths.pop().unwrap();
        
                            if format_subtitle(self.rules.clone().unwrap(), picked_path.clone()).is_err() {
                                self.picked_paths.push(picked_path.clone());
                            }
                        }
                        self.done = true;
                    }
                    if self.done {
                        ui.label("Done");
                    }
                });
            } else {
                ui.heading(self.error_message.clone());
            }
        });
    }
}

fn format_subtitle(rules: serde_json::Value, path: PathBuf) -> Result<(), io::Error> {
    let mut content = utils::read_content_file(path.clone())?;
    for (key, value) in rules.as_object().unwrap() {
        content = content.replace(key, value.as_str().unwrap());
    }
    
    utils::write_content_file(content, path)
}
