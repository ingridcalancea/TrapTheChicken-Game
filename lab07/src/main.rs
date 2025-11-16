fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

use std::fmt;
use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new<A: Into<f64>, B: Into<f64>>(a: A, b: B) -> Self {
        Complex {
            real: a.into(),
            imag: b.into(),
        }
    }

    pub fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl From<i32> for Complex {
    fn from(n: i32) -> Self {
        Complex {
            real: n as f64,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(n: f64) -> Self {
        Complex {
            real: n,
            imag: 0.0,
        }
    }
}


impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        if (self.real - other.real).abs() < 0.001 && (self.imag - other.imag).abs() < 0.001
        {
            true
        }
        else {
            false
        }
    }
}

impl<T: Into<Complex>> Add<T> for Complex {
    
    type Output = Complex;
    fn add(self, other: T) -> Self::Output{
        let r = other.into();
        Complex {
            real: self.real + r.real,
            imag: self.imag + r.imag,
        }
    }
}

impl<T: Into<Complex>> Sub<T> for Complex {
    
    type Output = Complex;
    fn sub(self, other: T) -> Self::Output{
        let r = other.into();
        Complex {
            real: self.real - r.real,
            imag: self.imag - r.imag,
        }
    }
}


impl<T: Into<Complex>> Mul<T> for Complex {
    type Output = Complex;

    fn mul(self, other: T) -> Self::Output {
        let r = other.into();
        Complex {
            real: self.real * r.real - self.imag * r.imag,
            imag: self.real * r.imag + self.imag * r.real,
        }
    }
}


impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}


impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.real, self.imag) {
            (0.0, 0.0) => write!(f, "0"),
            (0.0, i) => write!(f, "{}i", i),
            (r, 0.0) => write!(f, "{}", r),
            (r, i) if i > 0.0 => write!(f, "{}+{}i", r, i),
            (r, i) => write!(f, "{}{}i", r, i),
        }
    }
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}

