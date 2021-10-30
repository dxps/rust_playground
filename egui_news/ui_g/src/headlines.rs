use std::borrow::Cow;

use eframe::egui::{
    self, Button, Color32, CtxRef, Direction, FontDefinitions, FontFamily, Label, Layout,
    TopBottomPanel,
};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    pub fn new() -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("desc {}", a),
            url: format!("url {}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
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

    pub(crate) fn render_top_panel(&self, ctx: &CtxRef) {
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
                    ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
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
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
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
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}
