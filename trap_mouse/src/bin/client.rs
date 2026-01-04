use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9090")?;
    
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let mesaj = input.trim();
        if mesaj == "exit" {
            break;
        }

        stream.write_all(mesaj.as_bytes())?;

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                println!("{}", str::from_utf8(&buffer[0..n]).unwrap_or("Err"));
            }
            _ => break,
        }
    }
    Ok(())
}