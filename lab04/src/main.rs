use std::{fs, io};

fn pb1(fisier: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(fisier)?;

    let mut line_maxb = "";
    let mut line_maxc = "";

    let mut maxb = 0;
    let mut maxc = 0;

    for line in s.lines() {
        if line.len() > maxb {
            maxb = line.len();
            line_maxb = line;
        }

        if line.chars().count() > maxc {
            maxc = line.chars().count();
            line_maxc = line;
        }
    }
    println!("longest byte line: {} with {} bytes", line_maxb, maxb);
    println!("longest char line: {} with {} chars", line_maxc, maxc);

    Ok(())
}

fn pb2(word: &str) -> Result<String, char> {
    let mut coded = String::new();

    for c in word.chars() {
        let new_c = if c.is_ascii_uppercase() {
            (((c as u8 - b'A' + 13) % 26) + b'A') as char
        } else if c.is_ascii_lowercase() {
            (((c as u8 - b'a' + 13) % 26) + b'a') as char
        } else {
            return Err(c);
        };
        coded.push(new_c);
    }

    Ok(coded)
}

fn pb3(fisier: &str) -> Result<(), io::Error> {
    let phrase = fs::read_to_string(fisier)?;

    let phrase = phrase.replace(" pt ", " pentru ");
    let phrase = phrase.replace(" ptr ", " pentru ");
    let phrase = phrase.replace(" dl ", " domnul ");
    let phrase = phrase.replace(" dna ", " doamna ");

    println!("correct phrase: {}", phrase);
    Ok(())
}

fn pb4(fisier: &str) -> Result<(), io::Error> {
    let info = fs::read_to_string(fisier)?;

    for line in info.lines() {
        if !line.starts_with('#') {
            let mut v1 = "";
            let mut v2 = "";

            for (k, v) in line.split_whitespace().enumerate() {
                if k == 0 {
                    v1 = v;
                } else if k == 1 {
                    v2 = v;
                }

                if k > 2 {
                    break;
                }
            }
            println!("{} => {}", v1, v2);
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = pb1("fisier.txt") {
        println!("error: {} ", e);
    }

    match pb2("ProgramareRust") {
        Ok(word) => println!("{}", word),
        Err(c) => println!("char invalid: {}", c),
    }

    if let Err(e) = pb3("fisier2.txt") {
        println!("error: {} ", e);
    }

    if let Err(e) = pb4("fisier3.txt") {
        println!("error: {} ", e);
    }
}
