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

#[derive(Debug)]
enum ErrorOp {
    Addit,
    Mutipl,
}

fn addition_r(x: u32, y: u32) -> Result<u32, ErrorOp> {
    if (x as u64 + y as u64) < u32::MAX as u64 {
        Ok(x + y)
    } else {
        Err(ErrorOp::Addit)
    }
}

fn multiplication_r(x: u32, y: u32) -> Result<u32, ErrorOp> {
    if (x as u64 * y as u64) < u32::MAX as u64 {
        Ok(x * y)
    } else {
        Err(ErrorOp::Mutipl)
    }
}

fn compute_op(a :u32, b:u32) -> Result<(u32, u32), ErrorOp>
{
    let add = addition_r(a, b)?;
    let mult = multiplication_r(a,b)?;
    Ok((add, mult))
}

fn op()
{
    match compute_op(1234, 45678){
        Ok((a,b)) => println!("sum: {}, multpl: {}", a, b),
       Err(ErrorOp::Addit)=>println!("eraore la adunare"),
       Err(ErrorOp::Mutipl)=>println!("eroare la inmultire"),
    }
    match compute_op(1234564, 45345678){
        Ok((a,b)) => println!("sum: {}, multpl: {}", a, b),
       Err(ErrorOp::Addit)=>println!("eraore la adunare"),
       Err(ErrorOp::Mutipl)=>println!("eroare la inmultire"),
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
    op();

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
