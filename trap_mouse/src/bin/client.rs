use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::{str, thread};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9090")?;

    println!("--- JOC SOARECELE SI CAPCANA ---");
    println!("Alege modul de joc:\n1 - Player\n2 - Computer");
    print!("Optiune (apasa 1 sau 2 urmat de ENTER): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    stream.write_all(choice.as_bytes())?;

    let mut stream_read = stream.try_clone()?;
    thread::spawn(move || {
        let mut buffer = [0; 4096];
        loop {
            match stream_read.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let received = str::from_utf8(&buffer[0..n]).unwrap_or("");
                    print!("{}", received);
                    print!("Comanda (MOVE x y): ");
                    let _ = io::stdout().flush();
                }
                _ => {
                    println!("\nConexiune pierduta.");
                    std::process::exit(0);
                }
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "exit" {
            break;
        }
        if stream.write_all(input.as_bytes()).is_err() {
            break;
        }
    }
    Ok(())
}
