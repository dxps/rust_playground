mod headlines;

use eframe::{
    egui::{CentralPanel, CtxRef, Hyperlink, Label, ScrollArea, TopBottomPanel, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};
use headlines::Headlines;

fn main() {
    tracing_subscriber::fmt().init();

    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600.0, 800.0));
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
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_inited {
            self.render_config(ctx);
        } else {
            self.render_top_panel(ctx, frame);
            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| self.render_news_cards(ui));
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
