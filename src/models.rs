use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub completed: bool,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "default_color")]
    pub color: String,
}

fn default_status() -> String {
    "Planned".into()
}

fn default_color() -> String {
    "#2196F3".into()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quarter {
    pub year: u32,
    pub quarter: u32,
    #[serde(default)]
    pub features: Vec<Feature>,
}

impl Quarter {
    pub fn new(year: u32, quarter: u32) -> Self {
        Self {
            year,
            quarter,
            features: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        format!("Q{} {}", self.quarter, self.year)
    }

    pub fn date_range(&self) -> String {
        let months: [(u32, u32); 4] = [(1, 3), (4, 6), (7, 9), (10, 12)];
        let (start_month, end_month) = months[(self.quarter - 1) as usize];
        let start = chrono::NaiveDate::from_ymd_opt(self.year as i32, start_month, 1).unwrap();
        let end = if end_month == 12 {
            chrono::NaiveDate::from_ymd_opt(self.year as i32, 12, 31).unwrap()
        } else {
            chrono::NaiveDate::from_ymd_opt(self.year as i32, end_month + 1, 1).unwrap()
                - chrono::Duration::days(1)
        };
        format!(
            "{} - {}",
            start.format("%b %d"),
            end.format("%b %d")
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct RoadmapFile {
    pub quarters: Vec<Quarter>,
}
