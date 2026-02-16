use rand::Rng;
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

type Board = [[Cell; 11]; 11];

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
    let mut board = [[Cell::Empty; 11]; 11];
    let mut rng = rand::thread_rng();

    board[5][5] = Cell::Mouse;

    let walls = rng.gen_range(5..=12);
    let mut placed = 0;

    while placed < walls {
        let x = rng.gen_range(0..11);
        let y = rng.gen_range(0..11);

        if board[x][y] == Cell::Empty {
            board[x][y] = Cell::Trap;
            placed += 1;
        }
    }

    board
}

fn parse_coords(parts: &[&str]) -> Option<(usize, usize)> {
    if parts.len() != 3 {
        return None;
    }
    let x = parts[1].parse::<usize>().ok()?;
    let y = parts[2].parse::<usize>().ok()?;
    if x < 11 && y < 11 { Some((x, y)) } else { None }
}

fn mouse_can_move(board: &Board, pos: (usize, usize)) -> bool {
    let dirs = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    dirs.iter().any(|(dx, dy)| {
        let nx = pos.0 as isize + dx;
        let ny = pos.1 as isize + dy;
        nx >= 0 && ny >= 0 && nx < 11 && ny < 11 && board[nx as usize][ny as usize] == Cell::Empty
    })
}

fn computer_mouse_move(board: &Board, pos: (usize, usize)) -> Option<(usize, usize)> {
    let dirs = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut best = None;
    let mut best_score = i32::MIN;

    for (dx, dy) in dirs {
        let nx = pos.0 as isize + dx;
        let ny = pos.1 as isize + dy;
        if nx < 0 || ny < 0 || nx >= 11 || ny >= 11 {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if board[nx][ny] != Cell::Empty {
            continue;
        }
        let dist = std::cmp::min(std::cmp::min(nx, 10 - nx), std::cmp::min(ny, 10 - ny)) as i32;
        let free = dirs
            .iter()
            .filter(|(ddx, ddy)| {
                let xx = nx as isize + ddx;
                let yy = ny as isize + ddy;
                xx >= 0
                    && yy >= 0
                    && xx < 11
                    && yy < 11
                    && board[xx as usize][yy as usize] == Cell::Empty
            })
            .count() as i32;

        let score = -10 * dist + 2 * free;
        if score > best_score {
            best_score = score;
            best = Some((nx, ny));
        }
    }
    best
}
fn serialize_board(board: &Board) -> String {
    let mut s = String::from("BOARD_START\n");
    for row in board.iter() {
        for cell in row.iter() {
            s.push(match cell {
                Cell::Empty => '.',
                Cell::Trap => '#',
                Cell::Mouse => 'M',
            });
        }
        s.push('\n');
    }
    s.push_str("BOARD_END\n");
    s
}

fn mesaje(room: &mut Room) {
    let board_str = serialize_board(&room.board);
    let mouse_won = room.mouse_pos.0 == 0
        || room.mouse_pos.0 == 10
        || room.mouse_pos.1 == 0
        || room.mouse_pos.1 == 10;
    let mouse_lost = !mouse_can_move(&room.board, room.mouse_pos);

    for (i, p) in room.players.iter_mut().enumerate() {
        let player_role = if room.mode == Mode::Computer {
            if i == 0 { Role::Trapper } else { Role::Mouse }
        } else if i == 0 {
            Role::Trapper
        } else {
            Role::Mouse
        };
        let game_over_msg = if mouse_won {
            if player_role == Role::Mouse {
                "GAME_OVER: WIN\n"
            } else {
                "GAME_OVER: LOSE\n"
            }
        } else if mouse_lost {
            if player_role == Role::Mouse {
                "GAME_OVER: LOSE\n"
            } else {
                "GAME_OVER: WIN\n"
            }
        } else {
            ""
        };
        let msg = format!(
            "{}{}YOUR_ROLE: {:?}\nTURN: {:?}\nMODE: {:?}\n",
            board_str, game_over_msg, player_role, room.turn, room.mode
        );
        let _ = p.write_all(msg.as_bytes());
        let _ = p.flush();
    }
}

fn computer_move_if_needed(room: &mut Room) {
    if room.mode == Mode::Computer && room.turn == Role::Mouse {
        if let Some(new_pos) = computer_mouse_move(&room.board, room.mouse_pos) {
            room.board[room.mouse_pos.0][room.mouse_pos.1] = Cell::Empty;
            room.board[new_pos.0][new_pos.1] = Cell::Mouse;
            room.mouse_pos = new_pos;
        }
        room.turn = Role::Trapper;
        mesaje(room);
    }
}

fn handle_client(
    mut stream: TcpStream,
    state: Arc<Mutex<ServerState>>,
    room_id: usize,
    player_idx: usize,
) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let msg = String::from_utf8_lossy(&buffer[0..n]);
                let parts: Vec<&str> = msg.split_whitespace().collect();
                let mut state_lock = match state.lock() {
                    Ok(g) => g,
                    Err(poison) => poison.into_inner(),
                };
                let room = match state_lock.rooms.get_mut(room_id) {
                    Some(r) => r,
                    None => break,
                };
                let my_role = if room.mode == Mode::Computer
                    || (room.mode == Mode::Human && player_idx == 0)
                {
                    Role::Trapper
                } else {
                    Role::Mouse
                };

                if let (Some(&"MOVE"), Some((x, y))) = (parts.first(), parse_coords(&parts))
                    && room.turn == my_role
                {
                    match (my_role, room.board[x][y]) {
                        (Role::Trapper, Cell::Empty) => {
                            room.board[x][y] = Cell::Trap;
                            room.turn = Role::Mouse;
                            computer_move_if_needed(room);
                        }
                        (Role::Mouse, Cell::Empty) => {
                            let dx = (x as isize - room.mouse_pos.0 as isize).abs();
                            let dy = (y as isize - room.mouse_pos.1 as isize).abs();
                            if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) || (dx == 1 && dy == 1)
                            {
                                room.board[room.mouse_pos.0][room.mouse_pos.1] = Cell::Empty;
                                room.board[x][y] = Cell::Mouse;
                                room.mouse_pos = (x, y);
                                room.turn = Role::Trapper;
                            }
                        }
                        _ => {}
                    }
                    mesaje(room);
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
            eprintln!("Nu se poate deschide listener pe {}: {}", addr, e);
            return;
        }
    };

    let state = Arc::new(Mutex::new(ServerState { rooms: Vec::new() }));

    println!("Server pornit pe {}", addr);
    for stream_result in listener.incoming() {
        let stream = match stream_result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("eroare la acceptare client: {}", e);
                continue;
            }
        };

        let state_clone = Arc::clone(&state);
        thread::spawn(move || {
            let cloned_stream = match stream.try_clone() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("eroare la clonare steam: {}", e);
                    return;
                }
            };

            let mut reader = BufReader::new(cloned_stream);
            let mut choice = String::new();
            if reader.read_line(&mut choice).is_err() {
                eprintln!("eroare la decizia clientului");
                return;
            }

            let mode = match choice.trim() {
                "2" => Mode::Computer,
                _ => Mode::Human,
            };

            let mut state_lock = match state_clone.lock() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("eroare la lockul starii serverului: {}", e);
                    return;
                }
            };

            let (final_id, player_idx) = if mode == Mode::Human {
                if let Some(id) = state_lock
                    .rooms
                    .iter()
                    .position(|r| r.mode == Mode::Human && r.players.len() < 3)
                {
                    match stream.try_clone() {
                        Ok(s_clone) => state_lock.rooms[id].players.push(s_clone),
                        Err(e) => {
                            eprintln!("eroare streamul pentru al doilea jucator: {}", e);
                            return;
                        }
                    }
                    (id, 1)
                } else {
                    let new_id = state_lock.rooms.len();
                    match stream.try_clone() {
                        Ok(s_clone) => state_lock.rooms.push(Room {
                            mode,
                            players: vec![s_clone],
                            board: initial_board(),
                            mouse_pos: (5, 5),
                            turn: Role::Trapper,
                        }),
                        Err(e) => {
                            eprintln!("eroare streamul pentru primul jucator: {}", e);
                            return;
                        }
                    }
                    (new_id, 0)
                }
            } else {
                let new_id = state_lock.rooms.len();
                match stream.try_clone() {
                    Ok(s_clone) => state_lock.rooms.push(Room {
                        mode,
                        players: vec![s_clone],
                        board: initial_board(),
                        mouse_pos: (5, 5),
                        turn: Role::Trapper,
                    }),
                    Err(e) => {
                        eprintln!("eroare stream-ul pentru jucatorul uman: {}", e);
                        return;
                    }
                }
                (new_id, 0)
            };

            mesaje(&mut state_lock.rooms[final_id]);
            drop(state_lock);

            handle_client(stream, state_clone, final_id, player_idx);
        });
    }
}
