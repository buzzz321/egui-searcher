use std::{
    fs,
    io::{BufReader, Read},
};

use eframe::egui;
use egui::{Label, Sense};
use egui_extras::{Column, TableBuilder};

#[derive(Debug)]
struct Matches {
    _col: usize,
    row: usize,
}

impl Matches {
    fn new(source_matches_col: usize, source_matches_row: usize) -> Self {
        Self {
            _col: source_matches_col,
            row: source_matches_row,
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

    fn get_line(&self, lineno: usize) -> Option<String> {
        let bytebuff = self.source_text.as_bytes();

        if lineno > bytebuff.len() {
            return None;
        }

        let mut curr_line_end: usize = 0;
        let mut curr_line_start: usize;
        let mut curr_line_no: usize = 0;
        for (index, item) in bytebuff.iter().enumerate() {
            if *item == b'\n' {
                curr_line_start = curr_line_end;
                curr_line_end = index;

                if curr_line_no == lineno {
                    return Some(
                        std::str::from_utf8(&bytebuff[curr_line_start + 1..curr_line_end])
                            .unwrap()
                            .to_string(),
                    );
                }

                curr_line_no += 1;
            }
        }
        None
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
                let myedit = egui::TextEdit::multiline(&mut self.source_text).cursor_at_end(false);
                let myscroll = egui::ScrollArea::vertical().max_height(height * 0.7);
                ui.push_id(1, |ui| {
                    myscroll.show(ui, |ui| {
                        self.font_height = ui.style().text_styles()[0].resolve(ui.style()).size;

                        ui.add_sized([ui.available_width(), height * 0.7], myedit);
                    });
                });
                TableBuilder::new(ui)
                    .column(Column::remainder().at_least(100.0))
                    .column(Column::exact(40.0))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading(self.searchkey.to_string());
                        });
                    })
                    .body(|body| {

                        body.rows(15.0, self.source_matches.len(), |mut row| {
                            let row_index = row.index();
                            row.col(|ui| {
                                let resp = ui.add(
                                    Label::new(
                                        self.get_line(self.source_matches[row_index].row).unwrap(),
                                    )
                                    .sense(Sense::click()),
                                );

                                if resp.clicked() {
                                    println!(
                                        "row {} text {}",
                                        row_index,
                                        self.get_line(self.source_matches[row_index].row).unwrap()
                                    );
                                }
                            });
                        });
                    });
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "Searcher",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}
