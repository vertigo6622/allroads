use crate::models::Feature;

pub struct FeatureDialogState {
    title: String,
    description: String,
    status: String,
    color: String,
}

impl Default for FeatureDialogState {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            status: "Planned".into(),
            color: "#2196F3".into(),
        }
    }
}

impl FeatureDialogState {
    pub fn from_feature(f: &Feature) -> Self {
        Self {
            title: f.title.clone(),
            description: f.description.clone(),
            status: f.status.clone(),
            color: f.color.clone(),
        }
    }

    pub fn show(&mut self, ui: &mut eframe::egui::Ui) -> bool {
        let mut ok = false;
        ui.vertical(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.title);
            ui.add_space(4.0);

            ui.label("Description:");
            ui.text_edit_multiline(&mut self.description, 6);
            ui.add_space(4.0);

            ui.label("Status:");
            ui.horizontal(|ui| {
                for s in &["Planned", "Developing", "Testing", "Completed"] {
                    if ui.radio(self.status == *s, *s).clicked() {
                        self.status = s.to_string();
                    }
                }
            });
            ui.add_space(4.0);

            ui.label("Color:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.color);
                if ui.button("Choose Color").clicked() {
                    if let Some(hex) = rfd::ColorPicker::new()
                        .pick_color()
                        .map(|c| format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b))
                    {
                        self.color = hex;
                    }
                }
            });
            ui.add_space(8.0);

            if ui.button("OK").clicked() {
                ok = true;
            }
        });
        ok
    }

    pub fn to_feature(&self, id: &str) -> Option<Feature> {
        if self.title.trim().is_empty() {
            return None;
        }
        Some(Feature {
            id: id.to_string(),
            title: self.title.trim().to_string(),
            description: self.description.trim().to_string(),
            completed: self.status == "Completed",
            status: self.status.clone(),
            color: self.color.clone(),
        })
    }
}
