#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontId, FontFamily::Proportional, TextStyle::*, ProgressBar, Button};
use std::{path::PathBuf, thread, sync::mpsc::{Sender, Receiver, channel}};
mod utils;

struct MainApp {
    picked_paths: Vec<PathBuf>,
    failed_paths: Vec<PathBuf>,
    failed_error_messages: Vec<String>,
    rules: Option<serde_json::Value>,
    error_message: String,
    working: bool,
    progress: i32, // 0-100
    tx: Sender<i32>,
    rx: Receiver<i32>
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let mut style = (*ctx.style()).clone();

            style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(24.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(48.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
            ].into();

            ctx.set_style(style);

            if self.error_message.is_empty() {
                ui.vertical_centered(|ui| {
                    if ui.add_enabled(!self.working, egui::Button::new("Choose subtitles")).clicked() {
                        for picked_path in rfd::FileDialog::new().add_filter("subtitles", &["srt"]).pick_files().unwrap_or_else(|| vec![]) {
                            self.failed_paths.clear();
                            if !self.picked_paths.contains(&picked_path) {
                                self.picked_paths.push(picked_path);
                            }
                        }
                    }

                    ui.label(format!("Files selected: {}", self.picked_paths.len()));

                    if ui.add_enabled(!self.working, egui::Button::new("Format subtitles")).clicked() {
                        if self.picked_paths.len() > 0 {
                            self.working = true;

                            let rules = self.rules.clone().unwrap();
                            let picked_paths = self.picked_paths.clone();
                            let tx = self.tx.clone();
                            let divider: f32 = picked_paths.len() as f32;
                            thread::spawn(move || {
                                for (index, picked_path) in picked_paths.iter().enumerate() {
                                    if let Err(err) = utils::format_subtitle(rules.clone(), picked_path.clone()) {
                                        println!("{}", err.to_string())
                                    };
                                    let current_amount: f32 = (index+1) as f32;
                                    tx.send(((current_amount/divider)*100.0) as i32);
                                }
                                println!("done")
                            });
                            self.picked_paths.clear();
                        }
                    }

                    if self.progress == 100 {
                        ui.label("Done");
                        if self.failed_paths.len() > 0 {
                            for (index, failed_path) in self.failed_paths.clone().iter().enumerate() {
                                ui.small(format!("{} ({})", failed_path.file_name().unwrap().to_str().unwrap(), self.failed_error_messages[index]));
                            }
                        }
                    }

                    if self.working {
                        match self.rx.try_recv() {
                            Ok(amount_done) => {
                                self.progress = amount_done;
                            },
                            Err(_) => {}
                        }
                        ui.add(ProgressBar::new((self.progress as f32) / 100.0).desired_width(256.0));
                        if self.progress == 100 {
                            self.working = false;
                        }
                    }
                });
            } else {
                ui.heading(self.error_message.clone());
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let (tx, rx) = channel();

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
            progress: 0,
            tx,
            rx
        })),
    )
}