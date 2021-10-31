use std::borrow::Cow;

use eframe::{
    egui::{
        self, Align2, Button, Color32, CtxRef, Direction, FontDefinitions, FontFamily, Key, Label,
        Layout, TopBottomPanel, Vec2, Window,
    },
    epi,
};
use serde::{Deserialize, Serialize};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(Clone, Serialize, Deserialize)]
pub struct HeadlinesConfig {
    pub api_key: String,
    pub dark_mode: bool,
}

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self {
            api_key: String::default(),
            dark_mode: true,
        }
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub api_key_inited: bool,
}

impl Headlines {
    pub fn new() -> Self {
        // temporary sample
        let iter = (0..10).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("desc {}", a),
            url: format!("https://example.com/{}", a),
        });
        let config: HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines {
            articles: Vec::from_iter(iter),
            api_key_inited: !config.api_key.is_empty(),
            config,
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "Hasklig".to_string(),
            Cow::Borrowed(include_bytes!("../assets/Hasklig_Regular.otf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 32.0),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.0),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Hasklig".to_string());
        ctx.set_fonts(font_def);
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &CtxRef, frame: &mut epi::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.);
            egui::menu::bar(ui, |ui| {
                // The logo.
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📓").text_style(eframe::egui::TextStyle::Heading));
                });
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| ui.heading("Headlines"),
                );
                // The controls.
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add_space(2. * PADDING);
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    let _refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let theme_btn = ui.add(
                        Button::new({
                            if self.config.dark_mode {
                                "🌞"
                            } else {
                                "🌙"
                            }
                        })
                        .text_style(egui::TextStyle::Body),
                    );
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });
            });
            ui.add_space(6.);
        });
    }

    pub(crate) fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        let total = &self.articles.len();
        for (i, a) in (&self.articles).into_iter().enumerate() {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(egui::Hyperlink::new(&a.url).text("more ⤴"));
            });
            ui.add_space(PADDING);
            if i < total - 1 {
                ui.add(egui::Separator::default());
            }
        }
        ui.add_space(10. * PADDING);
    }

    pub fn render_config(&mut self, ctx: &CtxRef) {
        Window::new("Configuration")
            .collapsible(false)
            .anchor(Align2::CENTER_TOP, Vec2::new(0., 40.))
            .show(ctx, |ui| {
                ui.label("Enter your API Key from newsapi.org");
                let text_input = ui.text_edit_singleline(&mut self.config.api_key);
                text_input.request_focus();
                if text_input.lost_focus() || ui.input().key_pressed(Key::Enter) {
                    if let Err(e) = confy::store("headlines", self.config.clone()) {
                        tracing::error!("Failed saving config: {}", e);
                    }
                    tracing::info!("Saved config with API Key {}", self.config.api_key);
                    self.api_key_inited = true;
                }
                ui.label("For getting your API Key head over to");
                ui.hyperlink("https://newsapi.org");
            });
    }
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}
