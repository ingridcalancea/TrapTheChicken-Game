fn next_prime(x: u16) -> Option<u16> {
    let mut y = x + 1;
    while y < u16::MAX {
        let mut ok = 1;
        let mut d = 2;

        while d < y {
            if y.is_multiple_of(d) {
                ok = 0;
            }
            d += 1;
        }

        if ok == 1 {
            return Some(y);
        }
        y += 1;
    }
    None
}

fn addition(x: u32, y: u32) -> u32 {
    if (x as u64 + y as u64) < u32::MAX as u64 {
        x + y
    } else {
        panic!("!!!!the value doesnt fit in an u32!!!!");
    }
}

fn multiplication(x: u32, y: u32) -> u32 {
    if (x as u64 * y as u64) < u32::MAX as u64 {
        x * y
    } else {
        panic!("!!!!the value doesnt fit in an u32!!!!");
    }
}

fn addition_r(x: u32, y: u32) -> Result<u32, &'static str> {
    if (x as u64 + y as u64) < u32::MAX as u64 {
        Ok(x + y)
    } else {
        Err("!!!!the value doesnt fit in an u32!!!!")
    }
}

fn multiplication_r(x: u32, y: u32) -> Result<u32, &'static str> {
    if (x as u64 * y as u64) < u32::MAX as u64 {
        Ok(x * y)
    } else {
        Err("!!!!the value doesnt fit in an u32!!!!")
    }
}

enum ErrorChar {
    NonAscii,
    NonDigit,
    NonBase16Digit,
    NonLetter,
    NonPrintable,
}

impl ErrorChar {
    fn print_error(&self) {
        match self {
            ErrorChar::NonAscii => println!("char not Ascii"),
            ErrorChar::NonDigit => println!("char not digit"),
            ErrorChar::NonBase16Digit => println!("char not a base 16 digit"),
            ErrorChar::NonLetter => println!("char not letter"),
            ErrorChar::NonPrintable => println!("char not printable"),
        }
    }
}

fn to_uppercase(c: u8) -> Result<u8, ErrorChar> {
    if (c as char).is_ascii_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(ErrorChar::NonLetter)
    }
}

fn to_lowercase(c: u8) -> Result<u8, ErrorChar> {
    if (c as char).is_ascii_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(ErrorChar::NonLetter)
    }
}

fn print_char(c: u8) -> Result<(), ErrorChar> {
    if !((c as char).is_ascii_control()) {
        print!("{:?} {}", {}, c);
        Ok(())
    } else {
        Err(ErrorChar::NonPrintable)
    }
}

fn char_to_number(c: u8) -> Result<u8, ErrorChar> {
    if (c as char).is_ascii() {
        if (c as char).is_ascii_digit() {
            Ok(c - b'0')
        } else {
            Err(ErrorChar::NonDigit)
        }
    } else {
        Err(ErrorChar::NonAscii)
    }
}

fn char_to_number_hex(c: u8) -> Result<u8, ErrorChar> {
    if !c.is_ascii() {
        return Err(ErrorChar::NonAscii);
    }
    if (c as char).is_ascii_digit() {
        Ok(c - b'0')
    } else if (b'a'..=b'f').contains(&c) {
        Ok(10 + (c - b'a'))
    } else if (b'A'..=b'F').contains(&c) {
        Ok(10 + (c - b'A'))
    } else {
        Err(ErrorChar::NonBase16Digit)
    }
}

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 { None } else { Some(x / y) }
}

fn main() {
    //1
    let mut x = 62591;

    while let Some(y) = next_prime(x) {
        print!("{} ", y);
        x = y;
    }

    //3
    let a = 2000000000;
    let b = 1500000000;

    match addition_r(a, b) {
        Ok(result) => println!("Suma lui {} + {} = {}", a, b, result),
        Err(e) => println!("Eroare la adunare: {}", e),
    }

    let x = 50000;
    let y = 100000;

    match multiplication_r(x, y) {
        Ok(result) => println!("Produsul lui {} * {} = {}", x, y, result),
        Err(e) => println!("Eroare la înmulțire: {}", e),
    }

    let p = 90000000;
    let q = 90000000;

    match addition_r(p, q) {
        Ok(result) => println!("Suma lui {} + {} = {}", p, q, result),
        Err(e) => println!("Eroare la adunare: {}", e),
    }

    match multiplication_r(p, q) {
        Ok(result) => println!("Produsul lui {} * {} = {}", p, q, result),
        Err(e) => println!("Eroare la înmulțire: {}", e),
    }

    //4

    match to_uppercase(b'a') {
        Ok(ch) => println!("uppercase: {}", ch as char),
        Err(e) => e.print_error(),
    }

    match to_lowercase(b'G') {
        Ok(ch) => println!("lowercase: {}", ch as char),
        Err(e) => e.print_error(),
    }

    match print_char(b'\n') {
        Ok(_) => println!("printed"),
        Err(e) => e.print_error(),
    }

    match char_to_number(b'8') {
        Ok(n) => println!("number: {}", n),
        Err(e) => e.print_error(),
    }

    match char_to_number_hex(b'F') {
        Ok(n) => println!("hex number: {}", n),
        Err(e) => e.print_error(),
    }

    match char_to_number(b'x') {
        Ok(n) => println!("number: {}", n),
        Err(e) => e.print_error(),
    }

    //5

    let div1 = division(15.0, 4.25);
    let div2 = division(5.78, 0.0);

    if let Some(rez) = div1 {
        println!("rezultat impartire 1: {}", rez);
    } else {
        println!("impartire 1 invalida");
    }

    if let Some(rez) = div2 {
        println!("rezultat impartire 1: {}", rez);
    } else {
        println!("impartire 2 invalida");
    }

    //2

    let addit = addition(123, 345);
    println!("{}\n", addit);

    let multipl = multiplication(1234, 45679999);
    println! {"{}", multipl};
}
