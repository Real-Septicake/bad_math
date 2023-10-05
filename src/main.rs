mod bad_math{
    use std::{ops, fmt::Display};
    pub struct BadInt{
        data: i64
    }

    impl Display for BadInt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.data)
        }
    }

    impl Clone for BadInt {
        fn clone(&self) -> Self {
            Self { data: self.data }
        }
    }

    impl ops::Add<BadInt> for BadInt {
        type Output = BadInt;
        fn add(self, rhs: BadInt) -> Self::Output {
            let mut r_val: i64 = self.data;
            let mut c_val: i64 = rhs.data;
            while c_val > 0 {
                r_val += 1;
                c_val -=1;
            }
            Self { data: r_val }
        }
    }

    impl ops::AddAssign<BadInt> for BadInt {
        fn add_assign(&mut self, rhs: BadInt) {
            let mut r_val: i64 = self.data;
            let mut c_val: i64 = rhs.data;
            while c_val > 0 {
                r_val += 1;
                c_val -=1;
            }
            *self = BadInt{ data: r_val }
        }
    }

    impl ops::Sub<BadInt> for BadInt {
        type Output = BadInt;
        fn sub(self, rhs: BadInt) -> Self::Output {
            let mut r_val: i64 = self.data;
            let mut c_val: i64 = rhs.data;
            while c_val > 0 {
                r_val -= 1;
                c_val -=1;
            }
            Self { data: r_val }
        }
    }

    impl ops::SubAssign<BadInt> for BadInt {
        fn sub_assign(&mut self, rhs: BadInt) {
            let mut r_val: i64 = self.data;
            let mut c_val: i64 = rhs.data;
            while c_val > 0 {
                r_val -= 1;
                c_val -=1;
            }
            *self = BadInt{ data: r_val }
        }
    }

    impl ops::Mul<BadInt> for BadInt {
        type Output = BadInt;
        fn mul(self, rhs: BadInt) -> Self::Output {
            let mut r_val: i64 = self.data;
            let o_val: i64 = self.data;
            let mut c_val: i64 = rhs.data;
            if rhs.data == 0 {
                return BadInt{data:0}
            } else {
                while c_val > 1 {
                    c_val -= 1;
                    for _i in 0..o_val {
                        r_val += 1
                    }
                }
            }
            Self { data: r_val }
        }
    }

    impl ops::MulAssign<BadInt> for BadInt {
        fn mul_assign(&mut self, rhs: BadInt) {
            let mut r_val: i64 = self.data;
            let o_val: i64 = self.data;
            let mut c_val: i64 = rhs.data;
            if rhs.data == 0 {
                *self = BadInt{data:0}
            } else {
                while c_val > 1 {
                    c_val -= 1;
                    for _i in 0..o_val {
                        r_val += 1
                    }
                }
            }
            *self = BadInt{ data: r_val }
        }
    }

    impl ops::Div<BadInt> for BadInt {
        type Output = BadInt;
        fn div(self, rhs: BadInt) -> Self::Output {
            if rhs.data == 0 {
                panic!("Division by Zero")
            }else {
                let mut r_val: i64 = 0;
                let mut o_val: i64 = self.data;
                let c_val: i64 = rhs.data;
                while o_val >= c_val {
                    for _i in 0..c_val {
                        o_val -= 1
                    }
                    r_val += 1
                }

                Self { data: r_val }
            }
        }
    }

    impl ops::DivAssign<BadInt> for BadInt {
        fn div_assign(&mut self, rhs: BadInt) {
            if rhs.data == 0 {
                panic!("Division by Zero")
            }else {
                let mut r_val: i64 = 0;
                let mut o_val: i64 = self.data;
                let c_val: i64 = rhs.data;
                while o_val >= c_val {
                    for _i in 0..c_val {
                        o_val -= 1
                    }
                    r_val += 1
                }

                *self = BadInt{ data: r_val }
            }
        }
    }

    impl ops::Neg for BadInt {
        type Output = BadInt;
        fn neg(self) -> Self::Output {
            Self { data: -self.data}
        }
    }

    impl BadInt {
        pub fn new(val: i64) -> BadInt {
            BadInt{ data: val }
        }
    }
    
    // Bad Double

    pub struct BadDouble{
        int_data: i64,
        decimal: i16
    }

    impl BadDouble {
        fn trunc(val: f64) -> i16{
            ((val*10000.0) as i64 - (val as i64) * 10000) as i16
        }

        pub fn new(val: f64) -> BadDouble {
            BadDouble { int_data: val as i64, decimal: Self::trunc(val) }
        }
    }

    impl Display for BadDouble {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}.{:04}",self.int_data, self.decimal)
        }
    }

    impl Clone for BadDouble {
        fn clone(&self) -> Self {
            Self { int_data: self.int_data, decimal: self.decimal }
        }
    }

    impl ops::Add<BadDouble> for BadDouble {
        type Output = BadDouble;
        fn add(self, rhs: BadDouble) -> Self::Output {
            let mut r_int_data: i64 = self.int_data; let mut r_decimal: i16 = self.decimal;
            let mut c_int_data: i64 = rhs.int_data; let mut c_decimal: i16 = rhs.decimal;
            while c_int_data > 0 || c_decimal > 0 {
                if c_int_data > 0 {
                    c_int_data -= 1;
                    r_int_data += 1;
                }

                if c_decimal > 0 {
                    c_decimal -= 1;
                    r_decimal += 1;

                    if r_decimal == 10000 {
                        r_decimal = 0;
                        r_int_data += 1;
                    }
                }
            }

            Self { int_data: r_int_data, decimal: r_decimal }
        }
    }

    impl ops::Add<BadInt> for BadDouble {
        type Output = BadDouble;
        fn add(self, rhs: BadInt) -> Self::Output {
            let mut r_int_data: i64 = self.int_data;
            let mut c_data: i64 = rhs.data;
            while c_data > 0 {
                r_int_data += 1;
                c_data -= 1;
            }

            Self { int_data: r_int_data, decimal: self.decimal }
        }
    }

    impl ops::AddAssign<BadDouble> for BadDouble {
        fn add_assign(&mut self, rhs: BadDouble) {
            let mut r_int_data: i64 = self.int_data; let mut r_decimal: i16 = self.decimal;
            let mut c_int_data: i64 = rhs.int_data; let mut c_decimal: i16 = rhs.decimal;
            while c_int_data > 0 || c_decimal > 0 {
                if c_int_data > 0 {
                    c_int_data -= 1;
                    r_int_data += 1;
                }

                if c_decimal > 0 {
                    c_decimal -= 1;
                    r_decimal += 1;

                    if r_decimal == 10000 {
                        r_decimal = 0;
                        r_int_data += 1;
                    }
                }
            }

            *self = BadDouble{ int_data: r_int_data, decimal: r_decimal }
        }
    }

    impl ops::Sub<BadDouble> for BadDouble {
        type Output = BadDouble;
        fn sub(self, rhs: BadDouble) -> Self::Output {
            let mut r_int_data: i64 = self.int_data; let mut r_decimal: i16 = self.decimal;
            let mut c_int_data: i64 = rhs.int_data; let mut c_decimal: i16 = rhs.decimal;
            while c_int_data > 0 || c_decimal > 0 {
                if c_int_data > 0 {
                    c_int_data -= 1;
                    r_int_data -= 1;
                }

                if c_decimal > 0 {
                    c_decimal -= 1;
                    r_decimal -= 1;

                    if r_decimal == -1 {
                        r_decimal = 9999;
                        r_int_data -= 1;
                    }
                }
            }

            Self { int_data: r_int_data, decimal: r_decimal }
        }
    }

    impl ops::Sub<BadInt> for BadDouble{
        type Output = BadDouble;
        fn sub(self, rhs: BadInt) -> Self::Output {
            let mut r_int_data: i64 = self.int_data;
            let mut c_data:i64 = rhs.data;
            while c_data > 0 {
                r_int_data -= 1;
                c_data -= 1;
            }
            Self { int_data: r_int_data, decimal: self.decimal }
        }
    }

    impl ops::SubAssign<BadDouble> for BadDouble {
        fn sub_assign(&mut self, rhs: BadDouble) {
            let mut r_int_data: i64 = self.int_data; let mut r_decimal: i16 = self.decimal;
            let mut c_int_data: i64 = rhs.int_data; let mut c_decimal: i16 = rhs.decimal;
            while c_int_data > 0 || c_decimal > 0 {
                if c_int_data > 0 {
                    c_int_data -= 1;
                    r_int_data -= 1;
                }

                if c_decimal > 0 {
                    c_decimal -= 1;
                    r_decimal -= 1;

                    if r_decimal == -1 {
                        r_decimal = 9999;
                        r_int_data -= 1;
                    }
                }
            }

            *self = BadDouble{ int_data: r_int_data, decimal: r_decimal }
        }
    }

    impl ops::Mul<BadInt> for BadDouble {
        type Output = BadDouble;
        fn mul(self, rhs: BadInt) -> Self::Output {
            let mut r: BadDouble = BadDouble::new(0.0);
            let mut c: i64 = rhs.data;
            while c > 0 {
                r += self.clone();
                c -= 1;
            }
            r.clone()
        }
    }

    impl ops::MulAssign<BadInt> for BadDouble {
        fn mul_assign(&mut self, rhs: BadInt) {
            let mut r: BadDouble = BadDouble::new(0.0);
            let mut c: i64 = rhs.data;
            while c > 0 {
                r += self.clone();
                c -= 1;
            }
            *self = r;
        }
    }
}

use bad_math::{BadInt, BadDouble};

fn main(){
    let thing: BadInt = BadInt::new(2);
    let mut thing2: BadDouble = BadDouble::new(2.5);
    thing2 *= thing.clone();
    println!("{}", thing2)
}