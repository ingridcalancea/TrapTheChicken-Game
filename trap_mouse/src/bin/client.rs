use eframe::egui;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver};
use std::thread;

enum ServerEvent {
    BoardUpdate(String),
    Disconnected(String),
}

enum AppState {
    ChooseMode,
    InGame,
}

struct TrapApp {
    state: AppState,
    chosen_mode: Option<String>,
    stream: Option<TcpStream>,
    receiver: Option<Receiver<ServerEvent>>,
    board_raw: String,
    status_msg: String,
}

impl TrapApp {
    fn connect_and_send_mode(mode: &str) -> (Option<TcpStream>, Receiver<ServerEvent>, String) {
        let (tx, rx) = mpsc::channel();
        let addr = "127.0.0.1:9090";

        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                let _ = stream.write_all(mode.as_bytes());

        let mut read_stream = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                let _ = tx.send(ServerEvent::Disconnected(format!(
                "Eroare clonare stream: {}",
                e
        )));
        return (None, rx, format!("Eroare clonare stream: {}", e));
            }
        };

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
            Err(e) => (None, rx, format!("Server offline: {}", e)),
        }
    }
}

fn extract_value(raw: &str, key: &str) -> Option<String> {
    raw.lines()
        .find(|l| l.starts_with(key))
        .map(|l| l.replace(key, "").trim().to_string())
}

impl eframe::App for TrapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut receiver = self.receiver.take();
        let mut disconnected = None;

        if let Some(rx) = receiver.as_ref() {
            while let Ok(event) = rx.try_recv() {
                match event {
                    ServerEvent::BoardUpdate(msg) => {
                        self.board_raw = msg;
                        self.state = AppState::InGame;
                    }
                    ServerEvent::Disconnected(err) => {
                        disconnected = Some(err);
                        break;
                    }
                }
                ctx.request_repaint();
            }
        }

        if let Some(err) = disconnected {
            self.status_msg = err;
            self.stream = None;
            receiver = None;
            self.state = AppState::ChooseMode;
        }

        self.receiver = receiver;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐭 Șoarecele și Capcana 🧱");
            ui.separator();

            if matches!(self.state, AppState::ChooseMode) {
                ui.label("Alege modul de joc:");

                if ui.button("👤 Player vs Player").clicked() {
                    self.chosen_mode = Some("1\n".to_string());
                }
                if ui.button("🤖 Player vs Computer").clicked() {
                    self.chosen_mode = Some("2\n".to_string());
                }

                if let Some(mode) = &self.chosen_mode {
                    if ui.button("🔌 Conectează-te").clicked() {
                        let (s, rx, msg) = TrapApp::connect_and_send_mode(mode);
                        self.stream = s;
                        self.receiver = Some(rx);
                        self.status_msg = msg;
                    }
                }

                ui.label(&self.status_msg);
                return;
            }

            let my_role = extract_value(&self.board_raw, "YOUR_ROLE: ");
            let turn = extract_value(&self.board_raw, "TURN: ");
            let is_my_turn = my_role.as_deref() == turn.as_deref();

            ui.label(format!(
                "Rol: {} {}",
                my_role.unwrap_or("?".into()),
                if is_my_turn { "– RÂNDUL TĂU" } else { "" }
            ));

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

            egui::Grid::new("board").show(ui, |ui| {
                for x in 0..7 {
                    for y in 0..7 {
                        let c = board_lines
                            .get(x)
                            .and_then(|l| l.chars().nth(y))
                            .unwrap_or('.');

                        let label = match c {
                            'M' => "🐭",
                            '#' => "🧱",
                            _ => "·",
                        };

                        if ui.button(label).clicked() && is_my_turn {
                            if let Some(ref mut s) = self.stream {
                                let _ = s.write_all(
                                    format!("MOVE {} {}\n", x, y).as_bytes(),
                                );
                            }
                        }
                    }
                    ui.end_row();
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Client Capcana",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(TrapApp {
                state: AppState::ChooseMode,
                chosen_mode: None,
                stream: None,
                receiver: None,
                board_raw: String::new(),
                status_msg: String::new(),
            }))
        }),
    )
}
