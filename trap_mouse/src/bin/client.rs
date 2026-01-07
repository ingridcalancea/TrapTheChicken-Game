use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily, FontId, TextureHandle};
use egui::Color32;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::thread;

enum ServerEvent {
    BoardUpdate(String),
    Disconnected(String),
}

enum AppState {
    ChooseMode,
    InGame,
    GameOver(GameResult),
}

enum GameResult {
    Win,
    Lose,
}

struct TrapApp {
    state: AppState,
    chosen_mode: Option<String>,
    stream: Option<TcpStream>,
    receiver: Option<Receiver<ServerEvent>>,
    board_raw: String,
    status_msg: String,

    tex_mouse: Option<TextureHandle>,
    tex_wall: Option<TextureHandle>,
    tex_empty: Option<TextureHandle>,

    bg_menu: Option<TextureHandle>,
    bg_game: Option<TextureHandle>,
    bg_gameover: Option<TextureHandle>,
}

impl TrapApp {
    fn connect_and_send_mode(mode: &str) -> (Option<TcpStream>, Receiver<ServerEvent>, String) {
        let (tx, rx) = mpsc::channel();
        let addr = "127.0.0.1:9090";

        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                let _ = stream.write_all(mode.as_bytes());

                let mut read_stream = stream.try_clone().unwrap();
                let tx_clone = tx.clone();

                thread::spawn(move || {
                    let mut buffer = [0; 4096];
                    loop {
                        match read_stream.read(&mut buffer) {
                            Ok(n) if n > 0 => {
                                let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
                                let _ = tx_clone.send(ServerEvent::BoardUpdate(msg));
                            }
                            _ => {
                                let _ = tx_clone.send(ServerEvent::Disconnected(
                                    "Conexiune închisă".to_string(),
                                ));
                                break;
                            }
                        }
                    }
                });

                (Some(stream), rx, "Conectat!".to_string())
            }
            Err(e) => (None, rx, e.to_string()),
        }
    }
}

fn load_texture(ctx: &egui::Context, path: &str, name: &str) -> TextureHandle {
    let img = image::open(path).expect(path).to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR)
}

fn extract_value(raw: &str, key: &str) -> Option<String> {
    raw.lines()
        .find(|l| l.starts_with(key))
        .map(|l| l.replace(key, "").trim().to_string())
}

fn draw_background(ui: &egui::Ui, tex: &TextureHandle) {
    let rect = ui.max_rect();
    ui.painter().image(
        tex.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

impl eframe::App for TrapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.tex_mouse.is_none() {
            self.tex_mouse = Some(load_texture(ctx, "blocuri/mouse.png", "mouse"));
            self.tex_wall = Some(load_texture(ctx, "blocuri/wall.png", "wall"));
            self.tex_empty = Some(load_texture(ctx, "blocuri/empty.png", "empty"));

            self.bg_menu = Some(load_texture(ctx, "blocuri/bg_menu.png", "bg_menu"));
            self.bg_game = Some(load_texture(ctx, "blocuri/bg_game.png", "bg_game"));
            self.bg_gameover = Some(load_texture(ctx, "blocuri/bg_gameover.png", "bg_gameover"));
        }

        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "ComicSans".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../blocuri/ComicSansMS.ttf"
            ))),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "ComicSans".to_owned());

        ctx.set_fonts(fonts);

        if let Some(rx) = &self.receiver {
            let mut events = vec![];
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }

            for event in events {
                match event {
                    ServerEvent::BoardUpdate(msg) => {
                        if let Some(r) = extract_value(&msg, "GAME_OVER: ") {
                            self.state = match r.as_str() {
                                "WIN" => AppState::GameOver(GameResult::Win),
                                "LOSE" => AppState::GameOver(GameResult::Lose),
                                _ => AppState::InGame,
                            };
                        } else {
                            self.board_raw = msg;
                            self.state = AppState::InGame;
                        }
                    }
                    ServerEvent::Disconnected(err) => {
                        self.status_msg = err;
                        self.stream = None;
                        self.receiver = None;
                        self.state = AppState::ChooseMode;
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui|  {
            if matches!(self.state, AppState::ChooseMode) {
                draw_background(ui, self.bg_menu.as_ref().unwrap());

                ui.vertical_centered(|ui| {
                    ui.add_space(140.0);

                    ui.heading(egui::RichText::new(" ").size(48.0));

                    ui.add_space(325.0);

                    

                    if ui
                        .add_sized(
                            [260.0, 60.0],
                            egui::Button::new(
                                egui::RichText::new("Player vs Player")
                                    .font(FontId::new(24.0, FontFamily::Proportional))
                                    .color(Color32::WHITE),
                            )
                            .fill(Color32::from_rgb(0, 180, 0)),
                        )
                        .clicked()
                    {
                        self.chosen_mode = Some("1\n".to_string());
                    }

                    ui.add_space(20.0);

                    if ui
                        .add_sized(
                            [260.0, 60.0],
                            egui::Button::new(
                                egui::RichText::new("Player vs Computer")
                                    .font(FontId::new(24.0, FontFamily::Proportional))
                                    .color(Color32::WHITE),
                            )
                            .fill(Color32::from_rgb(0, 180, 0)),
                        )
                        .clicked()
                    {
                        self.chosen_mode = Some("2\n".to_string());
                    }

                    ui.add_space(30.0);

                    if let Some(mode) = &self.chosen_mode && ui
                            .add_sized(
                                [200.0, 55.0],
                                egui::Button::new(
                                    egui::RichText::new("START")
                                        .font(FontId::new(24.0, FontFamily::Proportional))
                                        .color(Color32::WHITE),
                                )
                                .fill(Color32::from_rgb(0, 180, 0)),
                            )
                            .clicked()
                        {
                            let (s, rx, msg) = TrapApp::connect_and_send_mode(mode);
                            self.stream = s;
                            self.chosen_mode = None;
                            self.receiver = Some(rx);
                            self.status_msg = msg;
                        }
                    

                    ui.add_space(20.0);
                    ui.label(&self.status_msg);
                });

                return;
            }

            if let AppState::GameOver(result) = &self.state {
                if let Some(bg) = self.bg_gameover.as_ref() {
                    draw_background(ui, bg);
                }

                ui.vertical_centered(|ui| {
                    ui.add_space(220.0);

                    match result {
                        GameResult::Win => {
                            ui.label(
                                egui::RichText::new("AI CÂȘTIGAT:)")
                                    .font(egui::FontId::new(50.0, egui::FontFamily::Proportional))
                                    .color(egui::Color32::DARK_BLUE),
                            );
                        }
                        GameResult::Lose => {
                            ui.label(
                                egui::RichText::new("AI PIERDUT:(")
                                    .font(egui::FontId::new(50.0, egui::FontFamily::Proportional))
                                    .color(egui::Color32::DARK_RED),
                            );
                        }
                    }

                    ui.add_space(30.0);
                    ui.label(
                        egui::RichText::new("Click oriunde pentru a reveni la meniu")
                            .font(egui::FontId::new(25.0, egui::FontFamily::Proportional))
                            .color(egui::Color32::DARK_GRAY)
                            .strong(),
                    );
                });

                if ctx.input(|i| i.pointer.any_click()) {
                    self.state = AppState::ChooseMode;
                    self.stream = None;
                    self.receiver = None;
                    self.board_raw.clear();
                    self.status_msg.clear();
                }
                return;
            }

            draw_background(ui, self.bg_game.as_ref().unwrap());

            ui.heading("Trap the chicken");
            ui.separator();

            let board_lines: Vec<&str> = self
                .board_raw
                .split("BOARD_START\n")
                .nth(1)
                .unwrap_or("")
                .split("BOARD_END")
                .next()
                .unwrap_or("")
                .lines()
                .collect();

            let your_role =
                extract_value(&self.board_raw, "YOUR_ROLE: ").unwrap_or("Unknown".to_string());
            let turn = extract_value(&self.board_raw, "TURN: ").unwrap_or("Unknown".to_string());

            ui.label(format!("Your role: {}", your_role));
            ui.label(format!("Current turn: {}", turn));
            ui.add_space(10.0);

            egui::Grid::new("board").spacing([4.0, 4.0]).show(ui, |ui| {
                for x in 0..11 {
                    for y in 0..11 {
                        let c = board_lines
                            .get(x)
                            .and_then(|l| l.chars().nth(y))
                            .unwrap_or('.');
                        let tex = match c {
                            'M' => &self.tex_mouse,
                            '#' => &self.tex_wall,
                            _ => &self.tex_empty,
                        };

                        let resp = if let Some(tex) = tex.as_ref() {
                            ui.add(
                                egui::Button::image(tex)
                                    .min_size(egui::vec2(48.0, 48.0))
                                    .frame(false),
                            )
                        } else {
                            return;
                        };

                        if resp.hovered() {
                            ui.painter().rect_filled(
                                resp.rect,
                                0.0,
                                egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100),
                            );
                        }

                        if resp.clicked() 
                            && let Some(s) = &mut self.stream {
                                let _ = s.write_all(format!("MOVE {} {}\n", x, y).as_bytes());
                            }
                        
                    }
                    ui.end_row();
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([760.0, 850.0])
            .with_min_inner_size([500.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Trap the chicken",
        options,
        Box::new(|_cc| {
            Ok(Box::new(TrapApp {
                state: AppState::ChooseMode,
                chosen_mode: None,
                stream: None,
                receiver: None,
                board_raw: String::new(),
                status_msg: String::new(),
                tex_mouse: None,
                tex_wall: None,
                tex_empty: None,
                bg_menu: None,
                bg_game: None,
                bg_gameover: None,
            }))
        }),
    )
}
