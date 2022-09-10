use std::{
    fs,
    io::{BufReader, Read},
};

use eframe::egui;
use egui_extras::{Size, TableBuilder};

#[derive(Debug)]
struct Matches {
    source_matches_col: usize,
    source_matches_row: usize,
}

impl Matches {
    fn new(source_matches_col: usize, source_matches_row: usize) -> Self {
        Self {
            source_matches_col,
            source_matches_row,
        }
    }
}

#[derive(Default)]
struct MyEguiApp {
    searchkey: String,
    source_text: String,
    file_path: Option<String>,
    source_matches: Vec<Matches>,
    font_height: f32,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn finder(&mut self) {
        let mut lineno: usize = 0;
        for line in self.source_text.lines() {
            if let Some(index) = line.find(&self.searchkey) {
                self.source_matches.push(Matches::new(index, lineno));
            }
            lineno += 1;
        }
        println!("matches on lines {:?} ", self.source_matches);
    }

    fn open_file(&mut self, filename: String) {
        let path = fs::File::open(&filename);
        match path {
            Ok(infile) => {
                let mut f = BufReader::new(infile);
                let metadata = fs::metadata(&filename).unwrap();
                self.source_text.reserve(metadata.len() as usize);
                if let Ok(_) = f.read_to_string(&mut self.source_text) {
                    return;
                }
            }
            Err(e) => println!("Cant open file due to: {}", e),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.file_path = Some(path.display().to_string());
                            //self.source_text = fs::read_to_string(self.file_path.clone().unwrap())
                            //    .expect("Should have been able to read the file");
                            self.open_file(self.file_path.as_ref().unwrap().clone());
                        }
                    }

                    ui.add(
                        egui::TextEdit::singleline(&mut self.searchkey)
                            .desired_width(ui.available_width() * 0.85),
                    );

                    //ui.add_space(ui.available_width() * 0.85);
                    ui.label("--");
                    let search_button = ui.add(egui::Button::new("Search"));
                    if search_button.clicked() {
                        println!("{}", self.searchkey);
                        self.finder();
                    }
                });
                let height = ui.available_height();

                // # egui::__run_test_ui(|ui| {
                // # let mut my_string = String::new();
                // # use egui::{ Color32, FontId };
                // let text_edit = egui::TextEdit::multiline(&mut my_string)
                //     .desired_width(f32::INFINITY);
                // let output = text_edit.show(ui);
                // let painter = ui.painter_at(output.response.rect);
                // let galley = painter.layout(
                //     String::from("Enter text"),
                //     FontId::default(),
                //     Color32::from_rgba_premultiplied(100, 100, 100, 100),
                //     f32::INFINITY
                // );
                // painter.galley(output.text_draw_pos, galley);
                // # });
                ui.push_id(1, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(height * 0.7)
                        .show(ui, |ui| {
                            self.font_height = ui.style().text_styles()[0].resolve(ui.style()).size;
                            ui.add_sized(
                                [ui.available_width(), height * 0.7],
                                egui::TextEdit::multiline(&mut self.source_text)
                                    .cursor_at_end(false),
                            );
                        });
                });
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
