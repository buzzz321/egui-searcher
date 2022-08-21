use eframe::egui;
use egui_extras::{Size, TableBuilder};

#[derive(Default)]
struct MyEguiApp {
    my_string: String,
    source_text: String,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let height = ui.available_height();
            ui.vertical(|ui| {
                ui.horizontal(|ui| {                   
                    ui.add(egui::TextEdit::singleline(&mut self.my_string));
                   
                    ui.add_space(ui.available_width()*0.85);
                    ui.label("--");
                    let search_button = ui.add(egui::Button::new("Search"));
                    if search_button.clicked() {
                        println!("{}", self.my_string);
                    }                  
                });
                
                ui.add_sized([ui.available_width(),height*0.7], egui::TextEdit::multiline(&mut self.source_text));
                   
                TableBuilder::new(ui)
                    .column(Size::remainder().at_least(100.0))
                    .column(Size::exact(40.0))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Growing");
                        });                       
                    })
                    .body(|mut body| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label("first row growing cell");
                            });                            
                        });
                    });
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Searcher",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}
