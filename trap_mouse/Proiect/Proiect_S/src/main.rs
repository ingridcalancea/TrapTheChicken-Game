use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Pvp,
    Strategy,
}

type Board = [[Cell; 7]; 7];

struct Room {
    mode: Mode,
    players: Vec<TcpStream>,
    board: Board,
    mouse_pos: (usize, usize),
    turn: Role,
}

fn initial_board() -> Board {
    let mut board = [[Cell::Empty; 7]; 7];
    board[3][3] = Cell::Mouse;
    board
}

fn serialize_board(board: &Board) -> String {
    board.iter().flat_map(|row| row.iter()).map(|cell| {
        match cell {
            Cell::Empty => '.',
            Cell::Trap => '#',
            Cell::Mouse => 'M',
        }
    }).collect()
}

fn parse_coords(parts: &[&str]) -> Option<(usize, usize)> {
    if parts.len() != 3 {
        return None;
    }

    let x = parts[1].parse::<usize>().ok()?;
    let y = parts[2].parse::<usize>().ok()?;

    if x < 7 && y < 7 {
        Some((x, y))
    } else {
        None
    }
}

fn strategy_mouse_move(board: &Board, pos: (usize, usize)) -> Option<(usize, usize)> {
    let dirs = [(-1,0), (0,1), (1,0), (0,-1)];

    for (dx, dy) in dirs {
        let nx = pos.0 as isize + dx;
        let ny = pos.1 as isize + dy;

        if nx >= 0 && ny >= 0 && nx < 7 && ny < 7 {
            let (nx, ny) = (nx as usize, ny as usize);
            if board[nx][ny] == Cell::Empty {
                return Some((nx, ny));
            }
        }
    }
    None
}

fn broadcast(room: &mut Room) {
    let board = serialize_board(&room.board);
    let msg = format!("BOARD {}\nTURN {:?}\n", board, room.turn);

    room.players.retain_mut(|p| {
        p.write_all(msg.as_bytes()).is_ok()
    });
}

fn handle_client(
    mut stream: TcpStream,
    rooms: Arc<Mutex<Vec<Room>>>,
) -> io::Result<()> {

    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }

    let input = match std::str::from_utf8(&buf[..n]) {
        Ok(s) => s.trim(),
        Err(_) => {
            stream.write_all(b"ERROR invalid utf8\n")?;
            return Ok(());
        }
    };

    let mode = if input == "JOIN PVP" {
        Mode::Pvp
    } else if input == "JOIN STRATEGY" {
        Mode::Strategy
    } else {
        stream.write_all(b"ERROR invalid JOIN\n")?;
        return Ok(());
    };

    let mut rooms = rooms.lock().map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "Mutex poisoned")
    })?;

    match mode {
        Mode::Pvp => {
            if let Some(room) = rooms.iter_mut().find(|r| r.mode == Mode::Pvp && r.players.len() == 1) {
                room.players.push(stream.try_clone()?);
                room.players[0].write_all(b"START TRAPPER\n")?;
                room.players[1].write_all(b"START MOUSE\n")?;
                broadcast(room);
            } else {
                let mut room = Room {
                    mode,
                    players: vec![stream.try_clone()?],
                    board: initial_board(),
                    mouse_pos: (3,3),
                    turn: Role::Trapper,
                };
                stream.write_all(b"WAIT\n")?;
                rooms.push(room);
            }
        }

        Mode::Strategy => {
            let mut room = Room {
                mode,
                players: vec![stream.try_clone()?],
                board: initial_board(),
                mouse_pos: (3,3),
                turn: Role::Trapper,
            };
            stream.write_all(b"START TRAPPER\n")?;
            broadcast(&mut room);
            rooms.push(room);
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    let rooms: Arc<Mutex<Vec<Room>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let rooms = rooms.clone();
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, rooms) {
                        eprintln!("Client error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
