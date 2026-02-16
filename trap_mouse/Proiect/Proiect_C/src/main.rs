use eframe::egui;
use std::io::{Read, Write};
use std::net::TcpStream;

struct App {
    stream: TcpStream,
    board: String,
    role: String,
    my_turn: bool,
    status: String,
}

impl App {
    fn send(&mut self, msg: &str) {
        let _ = self.stream.write_all(format!("{msg}\n").as_bytes());
    }

    fn handle_server_message(&mut self, msg: &str) {
        for line in msg.lines() {
            if let Some(rest) = line.strip_prefix("BOARD ") {
                self.board = rest.to_string();
            } else if let Some(rest) = line.strip_prefix("START ") {
                self.role = rest.to_string();
            } else if let Some(rest) = line.strip_prefix("TURN ") {
                self.my_turn = rest == self.role;
            } else if line.starts_with("WIN") {
                self.status = line.to_string();
                self.my_turn = false;
            }
        }
    }

    fn read_server(&mut self) {
        let mut buf = [0u8; 256];

        match self.stream.read(&mut buf) {
            Ok(0) => {
                self.status = "Disconnected from server".into();
            }
            Ok(n) => {
                if let Ok(msg) = std::str::from_utf8(&buf[..n]) {
                    self.handle_server_message(msg);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // normal pentru non-blocking
            }
            Err(_) => {
                self.status = "Network error".into();
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.read_server();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐭 Trap The Mouse");
            ui.label(format!("Role: {}", self.role));
            ui.label(format!("Status: {}", self.status));
            ui.separator();

            egui::Grid::new("board")
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for i in 0..7 {
                        for j in 0..7 {
                            let idx = i * 7 + j;
                            let c = self.board.chars().nth(idx).unwrap_or('.');

                            let label = match c {
                                '.' => "⬜",
                                '#' => "⬛",
                                'M' => "🐭",
                                _ => "?",
                            };

                            let clicked = ui.button(label).clicked();

                            if clicked && self.my_turn {
                                if self.role == "TRAPPER" && c == '.' {
                                    self.send(&format!("TRAP {i} {j}"));
                                }

                                if self.role == "MOUSE" && c == '.' {
                                    self.send(&format!("MOVE {i} {j}"));
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let mut stream = match TcpStream::connect("127.0.0.1:9000") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Cannot connect to server: {}", e);
            return Ok(());
        }
    };

    if stream.set_nonblocking(true).is_err() {
        eprintln!("Cannot set non-blocking mode");
        return Ok(());
    }

    println!("Choose mode:");
    println!("1 = Player vs Player");
    println!("2 = Player vs Strategy");

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return Ok(());
    }

    let join_msg = if input.trim() == "1" {
        "JOIN PVP\n"
    } else {
        "JOIN STRATEGY\n"
    };

    if stream.write_all(join_msg.as_bytes()).is_err() {
        eprintln!("Cannot send JOIN");
        return Ok(());
    }

    eframe::run_native(
        "Trap The Mouse",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            Box::new(App {
                stream,
                board: ".".repeat(49),
                role: "UNKNOWN".into(),
                my_turn: false,
                status: "Connected".into(),
            })
        }),
    )
}
