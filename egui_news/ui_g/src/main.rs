mod headlines;

use std::{
    sync::mpsc::{channel, sync_channel, Sender},
    thread,
};

use api_news::NewsAPI;
use eframe::{
    egui::{CentralPanel, CtxRef, Hyperlink, Label, ScrollArea, TopBottomPanel, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};
use headlines::{Headlines, NewsCardData};

use crate::headlines::AppMsg;

fn main() {
    tracing_subscriber::fmt().init();

    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(660.0, 700.0));
    run_native(Box::new(app), win_options)
}

impl App for Headlines {
    // A one-time lifecycle method called during initial init.
    fn setup(
        &mut self,
        ctx: &CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        let (app_tx, app_rx) = sync_channel(1);
        self.app_tx = Some(app_tx);

        let (mut news_tx, news_rx) = channel();
        self.news_rx = Some(news_rx);

        let api_key = self.config.api_key.to_string();

        thread::spawn(move || {
            if !api_key.is_empty() {
                fetch_news(&api_key, &mut news_tx);
            } else {
                loop {
                    match app_rx.recv() {
                        Ok(AppMsg::ApiKeyProvided(api_key)) => fetch_news(&api_key, &mut news_tx),
                        Err(e) => tracing::error!("Failed receiving news msg: {}", e),
                    }
                }
            }
        });

        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_inited {
            self.render_config(ctx);
        } else {
            self.preload_articles();

            self.render_top_panel(ctx, frame);
            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| self.render_news_cards(ui));
                render_footer(ctx);
            });
        }
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org").monospace());
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
                    .text("Made with egui")
                    .text_style(eframe::egui::TextStyle::Monospace),
            );
            ui.add_space(10.);
        });
    });
}

fn fetch_news(api_key: &str, news_tx: &mut Sender<NewsCardData>) {
    if let Ok(response) = NewsAPI::new(&api_key).fetch() {
        let articles = response.articles();
        for a in articles.iter() {
            let news = NewsCardData {
                title: a.title().to_string(),
                desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string()),
                url: a.url().to_string(),
            };
            if let Err(e) = news_tx.send(news) {
                tracing::error!("Failed tx news data: {}", e);
            }
        }
    } else {
        tracing::error!("Failed fetching news");
    }
}
