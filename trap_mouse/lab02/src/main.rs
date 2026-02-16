//pb1
fn add_chars_n(mut s: String, c: char, i: i32) -> String {
    let mut j: i32 = 0;
    let temp = String::from(c);
    while j < i {
        s += &temp;
        j += 1;
    }

    s
}

//pb2
fn add_chars_n2(ref_to_s: &mut String, c: char, i: i32) {
    let mut j: i32 = 0;
    while j < i {
        ref_to_s.push(c);
        j += 1;
    }
}

//pb3

fn add_space(mut s: String, n: i32) -> String {
    let mut j: i32 = 0;
    let temp = String::from(" ");

    while j < n {
        s += &temp;
        j += 1;
    }

    s
}

fn add_str(mut s: String, s2: String) -> String {
    s += &s2;
    s
}

fn add_integer(mut s: String,mut i: i32) -> String {
    let mut j: i32 = 1;
    let mut p: i32 = 1;
    let mut r: i32 = 1;

    while i / p >= 10 {
        p *= 10;
        r += 1;
    }

    r %= 3;

    if r == 0 {
        r = 3;
    }

    while i > 0 {
        let k: char = ((((i / p) % 10) as u8) + b'0') as char;

        s.push(k);
        
        if (j == r || (j > r && (j - r) % 3 == 0)) && (p > 1) {
            let temp: String = String::from("_");
            s += &temp;
        }
        i %= p;
        j += 1;
        p /= 10;
    }

    s
}

fn add_float(mut s: String, i: f64) -> String {
    let integer = i.trunc() as i32;
    let frac = i.fract();

    let mut j: i32 = 1;
    let mut p: i32 = 1;
    let mut r: i32 = 1;

    while integer / p >= 10 {
        p *= 10;
        r += 1;
    }

    r %= 3;
    if r == 0 {
        r = 3;
    }

    if integer == 0 {
        s.push('0');
    }

    while p > 0 {
        let k: char = ((((integer / p) % 10) as u8) + b'0') as char;
        s.push(k);

        if (j == r || (j > r && (j - r) % 3 == 0)) && (p > 1) {
            s.push('_');
        }

        j += 1;
        p /= 10;
    }

    s.push('.');

    let frac = (frac * 1000.0).round() as i32;
    
    let mut k = ((((frac / 100) % 10) as u8) + b'0') as char;
    s.push(k);
    k = ((((frac / 10) % 10) as u8) + b'0') as char;
    s.push(k);
    k = (((frac % 10) as u8) + b'0') as char;
    s.push(k);

    s
}


fn main() {
    let mut s = String::from("");
    let mut i = 0;

    //main pb1
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    //main pb2
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        let ref_to_s: &mut String = &mut s;
        add_chars_n2(ref_to_s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);

    //main pb3
    let mut s2 = String::from("");
    s2 = add_space(s2, 40);
    s2 = add_str(s2, String::from("I"));
    s2 = add_space(s2, 1);
    s2 = add_str(s2, String::from("💚"));
    s2 = add_space(s2, 1);
    s2 = add_str(s2, String::from("\n"));
    s2 = add_space(s2, 40);
    s2 = add_str(s2, String::from("RUST."));
    s2 = add_space(s2, 1);
    s2 = add_str(s2, String::from("\n"));
    s2 = add_space(s2, 4);
    s2 = add_str(s2, String::from("Most"));
    s2 = add_space(s2, 12);
    s2 = add_str(s2, String::from("crate"));
    s2 = add_space(s2, 6); 
    s2 = add_integer(s2,306437968);
    s2 = add_space(s2, 11);
    s2 = add_str(s2, String::from("and")); 
    s2 = add_space(s2, 5);
    s2 = add_str(s2, String::from("latest"));
    s2 = add_space(s2, 6);
    s2 = add_str(s2, String::from("is"));
    s2 = add_space(s2, 1);
    s2 = add_str(s2, String::from("\n"));
    s2 = add_space(s2, 9); 
    s2 = add_str(s2, String::from("downloaded"));
    s2 = add_space(s2, 8);
    s2 = add_str(s2, String::from("has")); 
    s2 = add_space(s2, 13);
    s2 = add_str(s2, String::from("downloads")); 
    s2 = add_space(s2, 5);
    s2 = add_str(s2, String::from("the"));
    s2 = add_space(s2, 9);
    s2 = add_str(s2, String::from("version"));
    s2 = add_space(s2, 4);
    s2 = add_float(s2, 2.038);
    s2 = add_str(s2, String::from("."));

    print!("{}", s2);
}
