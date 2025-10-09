//pb1
fn add_chars_n(mut s: String, c: char, i: i32) -> String {
    let mut j: i32 = 0;
    let temp = String::from(c);
    while j < i {
        s += &temp;
        j = j + 1;
    }

    return s;
}

//pb2
fn add_chars_n2(ref_to_s: &mut String, c: char, i: i32) {
    let mut j: i32 = 0;
    while j < i {
        ref_to_s.push(c);
        j = j + 1;
    }
}

//pb3

fn add_space(mut s: String, n: i32) -> String {
    let mut j: i32 = 0;
    let temp = String::from(" ");

    while j < n {
        s += &temp;
        j = j + 1;
    }

    return s;
}

fn add_str(mut s: String, s2: String) -> String {
    s += &s2;
    return s;
}

fn add_integer(mut s: String, i: i32) -> String {
    let mut j: i32 = 1;
    let mut p: i32 = 1;

    while i / p > 10 {
        p = p * 10;
    }
    while i > 0 {
        let k: char = (((i / p) as u8) - ('0' as u8)) as char;

        s.push(k);

        if (j == 3) && (i / p > 9) {
            let temp: String = String::from("_");
            s += &temp;
            j = 0;
        }
        j = j + 1;
        p = p / 10;
    }

    return s;
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
    s2 = add_space(s2, 4);
    s2 = add_str(s2, String::from("miau"));
    s2 = add_integer(s2, 123456);

    print!("{}", s2);
}
