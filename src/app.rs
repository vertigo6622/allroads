use eframe::egui;
use chrono::Datelike;

mod models;
mod dialogs;
mod templates;

use models::*;
use dialogs::*;

pub struct RoadmapApp {
    quarters: Vec<Quarter>,
    current_file: Option<std::path::PathBuf>,
    status_text: String,
    dialog_state: DialogState,
}

#[derive(Default)]
enum DialogState {
    #[default]
    None,
    AddFeature {
        quarter_idx: usize,
    },
    EditFeature {
        quarter_idx: usize,
        feature_idx: usize,
    },
    EditStatus {
        quarter_idx: usize,
        feature_idx: usize,
    },
}

impl Default for RoadmapApp {
    fn default() -> Self {
        let mut app = Self {
            quarters: Vec::new(),
            current_file: None,
            status_text: "Ready".into(),
            dialog_state: DialogState::None,
        };
        app.initialize_quarters();
        app
    }
}

impl RoadmapApp {
    fn initialize_quarters(&mut self) {
        let now = chrono::Local::now();
        let current_year = now.year() as u32;
        let current_quarter = (now.month() - 1) / 3 + 1;

        for i in 0..4 {
            let mut q = current_quarter + i;
            let mut year = current_year;
            if q > 4 {
                q -= 4;
                year += 1;
            }
            self.quarters.push(Quarter::new(year, q));
        }
    }

    fn add_quarter(&mut self) {
        let (year, quarter) = if let Some(last) = self.quarters.last() {
            if last.quarter == 4 {
                (last.year + 1, 1)
            } else {
                (last.year, last.quarter + 1)
            }
        } else {
            let now = chrono::Local::now();
            (now.year() as u32, 1)
        };
        self.quarters.push(Quarter::new(year, quarter));
        self.status_text = format!("Added Q{} {}", quarter, year);
    }

    fn remove_quarter(&mut self) {
        if self.quarters.pop().is_some() {
            self.status_text = "Removed last quarter".into();
        }
    }

    fn clear_all(&mut self) {
        self.quarters.clear();
        self.status_text = "Cleared all quarters".into();
    }

    fn new_roadmap(&mut self) {
        self.quarters.clear();
        self.current_file = None;
        self.initialize_quarters();
        self.status_text = "New roadmap created".into();
    }

    fn open_roadmap(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(data) => match serde_json::from_str::<RoadmapFile>(&data) {
                    Ok(file) => {
                        self.quarters = file.quarters;
                        self.current_file = Some(path);
                        self.status_text = "Opened roadmap".into();
                    }
                    Err(e) => self.status_text = format!("Error parsing: {}", e),
                },
                Err(e) => self.status_text = format!("Error reading: {}", e),
            }
        }
    }

    fn save_roadmap(&mut self) {
        if let Some(ref path) = self.current_file {
            self.save_to_file(path.clone());
        } else {
            self.save_as_roadmap();
        }
    }

    fn save_as_roadmap(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .set_file_name("roadmap.json")
            .save_file()
        {
            self.save_to_file(path);
        }
    }

    fn save_to_file(&mut self, path: std::path::PathBuf) {
        let file = RoadmapFile {
            quarters: self.quarters.clone(),
        };
        match serde_json::to_string_pretty(&file) {
            Ok(json) => match std::fs::write(&path, json) {
                Ok(_) => {
                    self.current_file = Some(path);
                    self.status_text = "Saved roadmap".into();
                }
                Err(e) => self.status_text = format!("Error saving: {}", e),
            },
            Err(e) => self.status_text = format!("Error serializing: {}", e),
        }
    }

    fn load_template(&mut self, template_type: &str) {
        self.quarters.clear();
        let now = chrono::Local::now();
        let year = now.year() as u32;
        let features = templates::get_template(template_type);
        let per_q = 2;
        let num_q = (features.len() + per_q - 1) / per_q;

        for i in 0..num_q {
            let mut q = Quarter::new(year, (i as u32) + 1);
            let start = i * per_q;
            let end = std::cmp::min(start + per_q, features.len());
            for (j, title) in features[start..end].iter().enumerate() {
                let colors = ["#4CAF50", "#2196F3", "#FF9800", "#9C27B0"];
                q.features.push(Feature {
                    id: format!("feature_{}_{}_{}", template_type, i, j),
                    title: title.to_string(),
                    description: format!("Implementation of {}", title),
                    completed: false,
                    status: "Planned".into(),
                    color: colors[i % colors.len()].into(),
                });
            }
            self.quarters.push(q);
        }
        self.status_text = format!("Loaded {} template", template_type);
    }
}

impl eframe::App for RoadmapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_roadmap();
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        self.open_roadmap();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save_roadmap();
                        ui.close_menu();
                    }
                    if ui.button("Save As").clicked() {
                        self.save_as_roadmap();
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Templates", |ui| {
                    if ui.button("Web Application").clicked() {
                        self.load_template("web");
                        ui.close_menu();
                    }
                    if ui.button("Mobile App").clicked() {
                        self.load_template("mobile");
                        ui.close_menu();
                    }
                    if ui.button("API Development").clicked() {
                        self.load_template("api");
                        ui.close_menu();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label(&self.status_text);
        });

        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Timeline");
                ui.add_space(10.0);
                if ui.button("Add Quarter").clicked() {
                    self.add_quarter();
                }
                if ui.button("Remove Quarter").clicked() {
                    self.remove_quarter();
                }
                if ui.button("Clear All").clicked() {
                    self.clear_all();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut dialog_action: Option<DialogAction> = None;

                for (qi, quarter) in &mut self.quarters.iter_mut().enumerate() {
                    egui::Frame::group(ui.style())
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180)))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.heading(quarter.name());
                                ui.label(quarter.date_range());
                                ui.separator();

                                if ui.button("+ Add Feature").clicked() {
                                    dialog_action = Some(DialogAction::OpenAddFeature(qi));
                                }

                                ui.add_space(4.0);

                                let mut remove_idx = None;
                                let mut move_up = None;
                                let mut move_down = None;

                                for (fi, feature) in quarter.features.iter().enumerate() {
                                    egui::Frame::none()
                                        .stroke(egui::Stroke::new(0.5, egui::Color32::from_rgb(200, 200, 200)))
                                        .inner_margin(4.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                let color = parse_color(&feature.color);
                                                let (rect, _) = ui.allocate_exact_size(
                                                    egui::vec2(6.0, 16.0),
                                                    egui::Sense::hover(),
                                                );
                                                ui.painter().rect_filled(rect, 0.0, color);

                                                if feature.completed {
                                                    ui.colored_label(egui::Color32::GRAY, &feature.title);
                                                } else {
                                                    ui.label(&feature.title);
                                                }

                                                let status_color = if feature.completed {
                                                    egui::Color32::GREEN
                                                } else {
                                                    egui::Color32::from_rgb(100, 100, 100)
                                                };
                                                ui.colored_label(status_color, format!("[{}]", feature.status));

                                                if ui.small_button("Edit").clicked() {
                                                    dialog_action = Some(DialogAction::OpenEditFeature(qi, fi));
                                                }
                                                if ui.small_button("Delete").clicked() {
                                                    remove_idx = Some(fi);
                                                }
                                                if ui.small_button("\u{2191}").clicked() {
                                                    move_up = Some(fi);
                                                }
                                                if ui.small_button("\u{2193}").clicked() {
                                                    move_down = Some(fi);
                                                }
                                            });
                                            ui.horizontal(|ui| {
                                                ui.add_space(18.0);
                                                ui.colored_label(egui::Color32::GRAY, &feature.description);
                                            });
                                        });
                                }

                                if let Some(fi) = remove_idx {
                                    quarter.features.remove(fi);
                                }
                                if let Some(fi) = move_up {
                                    if fi > 0 {
                                        quarter.features.swap(fi, fi - 1);
                                    } else if qi > 0 {
                                        let feature = quarter.features.remove(fi);
                                        self.quarters[qi - 1].features.push(feature);
                                    }
                                }
                                if let Some(fi) = move_down {
                                    if fi < quarter.features.len() - 1 {
                                        quarter.features.swap(fi, fi + 1);
                                    } else if qi < self.quarters.len() - 1 {
                                        let feature = quarter.features.remove(fi);
                                        self.quarters[qi + 1].features.insert(0, feature);
                                    }
                                }
                            });
                        });
                    ui.add_space(8.0);
                }

                if let Some(action) = dialog_action {
                    match action {
                        DialogAction::OpenAddFeature(qi) => {
                            self.dialog_state = DialogState::AddFeature { quarter_idx: qi };
                        }
                        DialogAction::OpenEditFeature(qi, fi) => {
                            self.dialog_state = DialogState::EditFeature {
                                quarter_idx: qi,
                                feature_idx: fi,
                            };
                        }
                    }
                }
            });
        });

        match &self.dialog_state {
            DialogState::AddFeature { quarter_idx } => {
                let qi = *quarter_idx;
                let mut dialog = FeatureDialogState::default();
                egui::Window::new("Add Feature")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        if dialog.show(ui) {
                            if let Some(feat) = dialog.to_feature(&format!("feature_{}", self.quarters[qi].features.len())) {
                                self.quarters[qi].features.push(feat);
                                self.status_text = format!("Added feature: {}", self.quarters[qi].features.last().unwrap().title);
                            }
                            self.dialog_state = DialogState::None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.dialog_state = DialogState::None;
                        }
                    });
            }
            DialogState::EditFeature { quarter_idx, feature_idx } => {
                let (qi, fi) = (*quarter_idx, *feature_idx);
                let existing = self.quarters[qi].features[fi].clone();
                let mut dialog = FeatureDialogState::from_feature(&existing);
                egui::Window::new("Edit Feature")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        if dialog.show(ui) {
                            if let Some(feat) = dialog.to_feature(&existing.id) {
                                self.quarters[qi].features[fi] = feat;
                                self.status_text = format!("Updated feature: {}", self.quarters[qi].features[fi].title);
                            }
                            self.dialog_state = DialogState::None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.dialog_state = DialogState::None;
                        }
                    });
            }
            DialogState::EditStatus { quarter_idx, feature_idx } => {
                let (qi, fi) = (*quarter_idx, *feature_idx);
                let mut status = self.quarters[qi].features[fi].status.clone();
                egui::Window::new("Edit Status")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            for s in &["Planned", "Developing", "Testing", "Completed"] {
                                if ui.radio(status == *s, *s).clicked() {
                                    status = s.to_string();
                                }
                            }
                            ui.horizontal(|ui| {
                                if ui.button("OK").clicked() {
                                    self.quarters[qi].features[fi].status = status.clone();
                                    self.quarters[qi].features[fi].completed = status == "Completed";
                                    self.status_text = format!("Updated status to: {}", status);
                                    self.dialog_state = DialogState::None;
                                }
                                if ui.button("Cancel").clicked() {
                                    self.dialog_state = DialogState::None;
                                }
                            });
                        });
                    });
            }
            DialogState::None => {}
        }
    }
}

enum DialogAction {
    OpenAddFeature(usize),
    OpenEditFeature(usize, usize),
}

fn parse_color(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let Ok(r) = u8::from_str_radix(&hex[0..2], 16) {
            if let Ok(g) = u8::from_str_radix(&hex[2..4], 16) {
                if let Ok(b) = u8::from_str_radix(&hex[4..6], 16) {
                    return egui::Color32::from_rgb(r, g, b);
                }
            }
        }
    }
    egui::Color32::from_rgb(33, 150, 243)
}
