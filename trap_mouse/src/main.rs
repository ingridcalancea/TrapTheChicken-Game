use std::{
    env,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Trap,
    Mouse,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Role {
    Trapper,
    Mouse,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Human,
    Computer,
}

type Board = [[Cell; 7]; 7];

struct Room {
    mode: Mode,
    players: Vec<TcpStream>,
    board: Board,
    mouse_pos: (usize, usize),
    turn: Role,
}

struct ServerState {
    rooms: Vec<Room>,
}

fn initial_board() -> Board {
    let mut board = [[Cell::Empty; 7]; 7];
    board[3][3] = Cell::Mouse;
    board
}

fn parse_coords(parts: &[&str]) -> Option<(usize, usize)> {
    if parts.len() != 3 {
        return None;
    }
    let x = parts[1].parse::<usize>().ok()?;
    let y = parts[2].parse::<usize>().ok()?;
    if x < 7 && y < 7 { Some((x, y)) } else { None }
}

fn computer_mouse_move(board: &Board, pos: (usize, usize)) -> Option<(usize, usize)> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for (dx, dy) in dirs {
        let nx = pos.0 as isize + dx -1;
        let ny = pos.1 as isize + dy -1;
        if nx >= 0 && ny >= 0 && nx < 7 && ny < 7 {
            let (nx, ny) = (nx as usize, ny as usize);
            if board[nx][ny] == Cell::Empty {
                return Some((nx, ny));
            }
        }
    }
    None
}


fn serialize_board(board: &Board) -> String {
    let mut s = String::from("BOARD_START\n");
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Empty => s.push('.'),
                Cell::Trap => s.push('#'),
                Cell::Mouse => s.push('M'),
            }
        }
        s.push('\n');
    }
    s.push_str("BOARD_END\n");
    s
}

fn mesaje(room: &mut Room) {
    let board_str = serialize_board(&room.board);
    for (i, p) in room.players.iter_mut().enumerate() {
        let player_role = if room.mode == Mode::Computer {
            Role::Trapper
        } else if i == 0 {
            Role::Mouse
        } else {
            Role::Trapper
        };
        // Markerii ajută clientul GUI să ignore restul textului
        let msg = format!(
            "{}YOUR_ROLE: {:?}\nTURN: {:?}\nMODE: {:?}\n",
            board_str, player_role, room.turn, room.mode
        );
        let _ = p.write_all(msg.as_bytes());
        let _ = p.flush();
    }
}

fn handle_client(mut stream: TcpStream, state: Arc<Mutex<ServerState>>, room_id: usize, player_idx: usize) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let msg = String::from_utf8_lossy(&buffer[0..n]);
                let parts: Vec<&str> = msg.split_whitespace().collect();
                let mut state_lock = state.lock().unwrap();
                let room = match state_lock.rooms.get_mut(room_id) {
                    Some(r) => r,
                    None => break,
                };

                let my_role = if room.mode == Mode::Computer { Role::Trapper } 
                             else if player_idx == 0 { Role::Mouse } 
                             else { Role::Trapper };

                if let (Some(&"MOVE"), Some((x, y))) = (parts.get(0), parse_coords(&parts)) {
                    if room.turn == my_role {
                        let mut move_made = false;
                        match (my_role, room.board[x][y]) {
                            (Role::Trapper, Cell::Empty) => {
                                room.board[x][y] = Cell::Trap;
                                room.turn = Role::Mouse;
                                move_made = true;
                                if room.mode == Mode::Computer {
                                    if let Some(new_pos) = computer_mouse_move(&room.board, room.mouse_pos) {
                                        room.board[room.mouse_pos.0][room.mouse_pos.1] = Cell::Empty;
                                        room.board[new_pos.0][new_pos.1] = Cell::Mouse;
                                        room.mouse_pos = new_pos;
                                    }
                                    room.turn = Role::Trapper;
                                }
                            }
                            (Role::Mouse, Cell::Empty) => {
                                let dx = (x as isize - room.mouse_pos.0 as isize).abs();
                                let dy = (y as isize - room.mouse_pos.1 as isize).abs();
                                if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) || (dx == 1 && dy == 1){
                                    room.board[room.mouse_pos.0][room.mouse_pos.1] = Cell::Empty;
                                    room.board[x][y] = Cell::Mouse;
                                    room.mouse_pos = (x, y);
                                    room.turn = Role::Trapper;
                                    move_made = true;
                                }
                            }
                            _ => {}
                        }
                        if move_made {
                            mesaje(room); // Trimite update imediat ambilor jucători
                        }
                    }
                }
            }
            _ => break,
        }
    }
}
fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9090".to_string());

    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Eroare bind: {}", e);
            return;
        }
    };

    let state = Arc::new(Mutex::new(ServerState { rooms: Vec::new() }));

    println!("Server pornit pe {}", addr);

    for stream in listener.incoming() {
        if let Ok(s) = stream {
            let state_clone = Arc::clone(&state);

            thread::spawn(move || {
                let stream_for_reader = match s.try_clone() {
                    Ok(cloned) => cloned,
                    Err(_) => return,
                };

                let mut reader = BufReader::new(stream_for_reader);
                let mut choice = String::new();

                if reader.read_line(&mut choice).is_ok() {
                    let mode = match choice.trim() {
                        "2" => Mode::Computer,
                        _ => Mode::Human,
                    };

                    let mut state_lock = match state_clone.lock() {
                        Ok(lock) => lock,
                        Err(poisoned) => poisoned.into_inner(),
                    };

                    let (final_id, player_idx) = if mode == Mode::Human {
                        if let Some(id) = state_lock
                            .rooms
                            .iter()
                            .position(|r| r.mode == Mode::Human && r.players.len() < 2)
                        {
                            if let Ok(cloned_s) = s.try_clone() {
                                state_lock.rooms[id].players.push(cloned_s);
                                (id, 1)
                            } else {
                                return;
                            }
                        } else {
                            let new_id = state_lock.rooms.len();
                            if let Ok(cloned_s) = s.try_clone() {
                                state_lock.rooms.push(Room {
                                    mode,
                                    players: vec![cloned_s],
                                    board: initial_board(),
                                    mouse_pos: (3, 3),
                                    turn: Role::Trapper,
                                });
                                (new_id, 0)
                            } else {
                                return;
                            }
                        }
                    } else {
                        let new_id = state_lock.rooms.len();
                        if let Ok(cloned_s) = s.try_clone() {
                            state_lock.rooms.push(Room {
                                mode,
                                players: vec![cloned_s],
                                board: initial_board(),
                                mouse_pos: (3, 3),
                                turn: Role::Trapper,
                            });
                            (new_id, 0)
                        } else {
                            return;
                        }
                    };

                    if let Some(room) = state_lock.rooms.get_mut(final_id) {
                        mesaje(room);
                    }

                    drop(state_lock);
                    handle_client(s, state_clone, final_id, player_idx);
                }
            });
        }
    }
}
