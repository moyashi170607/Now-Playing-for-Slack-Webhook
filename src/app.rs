use crate::fetch::{MusicMetadata, get_music_metadata};
use std::time::{Duration, Instant};

const FETCH_INTERVAL: Duration = Duration::from_secs(1);

const STORAGE_KEY: &str = "NowPlayingForSlackWebhook";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MusicEntry {
    pub metadata: MusicMetadata,
    pub selected: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NowPlayingApp {
    user_name: Option<String>,
    slack_webhook: Option<String>,

    #[serde(skip)]
    entries: Vec<MusicEntry>,
    #[serde(skip)]
    last_fetch: Option<Instant>,
}

impl Default for NowPlayingApp {
    fn default() -> Self {
        Self {
            user_name: Some(String::from("ユーザー")),
            slack_webhook: None,
            entries: Vec::new(),
            last_fetch: None,
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
            eframe::get_value(storage, STORAGE_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn fetch_metadata(&mut self) {
        let new_list = get_music_metadata().unwrap_or_default();
        self.entries = new_list
            .into_iter()
            .map(|metadata| {
                let selected = self
                    .entries
                    .iter()
                    .find(|e| {
                        e.metadata.title == metadata.title && e.metadata.artist == metadata.artist
                    })
                    .map(|e| e.selected)
                    .unwrap_or(false);
                MusicEntry { metadata, selected }
            })
            .collect();
        self.last_fetch = Some(Instant::now());
    }

    pub fn post(&mut self, webhook: Option<String>) {
        let Some(url) = webhook else { return };

        let selected: Vec<&MusicEntry> = self.entries.iter().filter(|e| e.selected).collect();
        if selected.is_empty() {
            return;
        }

        let text = selected
            .iter()
            .map(|e| {
                if e.metadata.album.is_empty() {
                    format!("{} - {}", e.metadata.title, e.metadata.artist)
                } else {
                    format!(
                        "{} - {} /{} 収録",
                        e.metadata.title, e.metadata.artist, e.metadata.album
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let username = format!(
            "{}の再生中の曲",
            self.user_name.as_deref().unwrap_or("ユーザー")
        );

        let body = serde_json::json!({"username":username ,"text": text });
        let _ = ureq::post(&url).send_json(body);
    }

    pub fn left_render(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("画像");
        });
    }

    pub fn center_render(&mut self, ui: &mut egui::Ui) {
        if self.entries.is_empty() {
            ui.label("再生中の曲なし");
        } else {
            for entry in &mut self.entries {
                let label = if entry.metadata.album.is_empty() {
                    format!("{} - {}", entry.metadata.title, entry.metadata.artist)
                } else {
                    format!(
                        "{} - {} / {} 収録",
                        entry.metadata.title, entry.metadata.artist, entry.metadata.album
                    )
                };
                ui.checkbox(&mut entry.selected, label);
            }
        }

        ui.separator();

        ui.label("ユーザー名:");
        ui.scope(|ui| {
            ui.visuals_mut().widgets.inactive.bg_stroke =
                egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 119, 255));
            ui.text_edit_singleline(self.user_name.get_or_insert_with(String::new));
        });

        ui.label("Slack Webhook URL:");
        ui.scope(|ui| {
            ui.visuals_mut().widgets.inactive.bg_stroke =
                egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 119, 255));
            ui.text_edit_singleline(self.slack_webhook.get_or_insert_with(String::new));
        });

        ui.separator();

        if ui.button("投稿").clicked() {
            self.post(self.slack_webhook.clone());
        }
    }
}

impl eframe::App for NowPlayingApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let should_fetch = self
            .last_fetch
            .map(|t| t.elapsed() >= FETCH_INTERVAL)
            .unwrap_or(true);

        if should_fetch {
            self.fetch_metadata();
        }

        ui.ctx().request_repaint_after(FETCH_INTERVAL);

        // egui::Panel::left("image_panel")
        //     .exact_size(100.0)
        //     .resizable(false)
        //     .show_inside(ui, |ui| {
        //         self.left_render(ui);
        //     });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.center_render(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, STORAGE_KEY, self);
    }
}
