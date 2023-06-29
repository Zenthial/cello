use bnum::BInt;
use std::ops::{Add, Div, Mul, Sub};

pub struct CobolInt {
    int: BInt<144>,
}

impl CobolInt {
    fn new() -> Self {
        Self { int: 0.into() }
    }
}

impl From<isize> for CobolInt {
    fn from(int: isize) -> Self {
        Self { int: int.into() }
    }
}

pub struct CobolNumber {
    underlying_number: Box<[u8]>,
    width: usize,
}

impl CobolNumber {
    fn new(number: isize) -> Self {
        let bytes = number.to_ne_bytes();
        CobolNumber {
            underlying_number: Box::from(bytes),
            width: bytes.len(),
        }
    }

    fn from_width(number: usize) -> Self {
        let bytes = 0_i128.to_ne_bytes();
        CobolNumber {
            underlying_number: Box::from(bytes),
            width: number,
        }
    }

    fn to_sized_bytes(&self) -> [u8; 16] {
        // this is incredibly ugly
        let mut bytes: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for (i, byte) in self.underlying_number.iter().enumerate() {
            bytes[i] = *byte;
            if i > 15 {
                break;
            }
        }

        return bytes;
    }

    pub fn to_num(&self) -> i128 {
        return i128::from_ne_bytes(self.to_sized_bytes());
    }
}

impl ToString for CobolNumber {
    fn to_string(&self) -> String {
        let num = self.to_num();
        let width_i128 = i128::try_from(self.width).expect("failed to convert width to i128");
        let packed_zeros = width_i128 - num;
        let mut zeros_string = String::new();
        for _ in 0..packed_zeros {
            zeros_string += "0"
        }

        format!("{zeros_string}{}", num)
    }
}

impl Add for CobolNumber {
    type Output = i128;

    fn add(self, rhs: Self) -> Self::Output {
        self.to_num() + rhs.to_num()
    }
}

impl Sub for CobolNumber {
    type Output = i128;

    fn sub(self, rhs: Self) -> Self::Output {
        self.to_num() - rhs.to_num()
    }
}

impl Mul for CobolNumber {
    type Output = i128;

    fn mul(self, rhs: Self) -> Self::Output {
        self.to_num() * rhs.to_num()
    }
}

impl Div for CobolNumber {
    type Output = i128;

    fn div(self, rhs: Self) -> Self::Output {
        self.to_num() / rhs.to_num()
    }
}
