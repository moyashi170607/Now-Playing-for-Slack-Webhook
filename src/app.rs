use crate::fetch::MusicMetadata;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NowPlayingApp {
    metadatas: Option<Vec<MusicMetadata>>,
    slack_webhook: Option<String>,
}

impl Default for NowPlayingApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            metadatas: None,
            slack_webhook: None,
        }
    }
}

impl NowPlayingApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_sans_jp".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../assets/font/NotoSansJP-VariableFont_wght.ttf"
            ))
            .into(),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "noto_sans_jp".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = egui::Color32::from_rgb(255, 255, 255);

        cc.egui_ctx.set_visuals(visuals);

        //ストレージから復元
        if let Some(storage) = cc.storage {
            storage
                .get_string(eframe::APP_KEY)
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn post(&mut self) {}

    pub fn render(&mut self, ui: &mut egui::Ui) {
        let music_text = match &self.metadatas {
            Some(list) if !list.is_empty() => {
                format!("{} - {} / {}", list[0].title, list[0].artist, list[0].album)
            }
            _ => "再生中の曲なし".to_string(),
        };
        ui.label(music_text);

        ui.separator();

        ui.label("Slack Webhook URL:");
        ui.scope(|ui| {
            ui.visuals_mut().widgets.inactive.bg_stroke =
                egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 100, 100));
            ui.text_edit_singleline(self.slack_webhook.get_or_insert_with(String::new));
        });

        ui.separator();

        if ui.button("投稿").clicked() {}
    }
}

impl eframe::App for NowPlayingApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.render(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
