fn prime(a: i32) -> bool {
    let mut b = 2;
    if a == 0 || a == 1 {
        return false;
    }
    while b <= a / 2 {
        if a % b == 0 {
            return false;
        }
        b = b + 1;
    }
    return true;
}

fn coprime(a: i32, b: i32) -> bool {
    let mut a1: i32 = a;
    let mut b1: i32 = b;

    while a1 != b1 {
        if a1 > b1 {
            a1 = a1 - b1;
        } else {
            b1 = b1 - a1;
        }
    }

    if a1 == 1 {
        return true;
    } else {
        return false;
    }
}

fn bottlesofbeer() {
    let mut i: i32 = 99;

    while i >= 1 {
        if i != 1 {
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottles of beer,", i);
            println!("Take one down, pass it around,");
            println!("{} bottles of beer on the wall.", i - 1);
            println!(" ");
        } else {
            println!("1 bottle of beer on the wall,");
            println!("1 bottle of beer,");
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.");
            println!(" ");
        }
        i = i - 1;
    }
}

fn main() {
    let mut i: i32 = 0;

    while i <= 100 {
        if prime(i) == true {
            println!("{}", i);
        }
        i = i + 1;
    }

    let mut i1: i32 = 1;
    let mut j: i32 = 1;

    while i1 <= 100 {
        let mut j = 1;

        while j <= 100 {
            if coprime(i1, j) == true {
                println!("{} {}", i1, j);
            }
            j = j + 1;
        }
        i1 = i1 + 1;
    }

    bottlesofbeer();
}
