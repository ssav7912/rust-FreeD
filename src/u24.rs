    use std::ops::{Add};
    use std::ops::{Mul};
    use std::ops::{Div};
    

    #[derive(Clone, Copy)]
    #[allow(non_camel_case_types)]
    pub struct u24 ([u8;3]);

    trait MinMax {
        const MIN:Self;
        const MAX:Self;
    }

    impl MinMax for u24 {
        const MIN:Self = u24([0x00, 0x00, 0x00]);
        const MAX:Self = u24([0xFF, 0xFF, 0xFF]);
    }

    

    impl Add for u24 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self::from_u32(self.to_u32() + rhs.to_u32())
        }
    }

    impl Mul for u24 {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Self::from_u32(self.to_u32() * rhs.to_u32())
        }
    }

    impl Div for u24 {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            Self::from_u32(self.to_u32() / rhs.to_u32())
        }
    }

    impl u24 {
        
        pub fn new(value: u32) -> Self {
            if value > u24::MAX.to_u32() || value < u24::MIN.to_u32(){
                compile_error!("This value does not fit into the type u24");
            }

            return Self::from_u32(value);
        }

        fn to_u32(self) -> u32 {
            let u24([a,b,c]) = self;
            u32::from_le_bytes([a,b,c,0])
        }
        fn from_u32(n: u32) -> Self {
            let [a,b,c,d] = n.to_le_bytes();
            debug_assert!(d == 0);
            u24([a,b,c])
        }
        

    }

#[cfg(test)]
mod tests {

    use super::*;
    

    #[test]
    fn u24_overflow() {
        let x: u24 = u24::from_u32(0xFFFFFF);
        let y: u24 = u24::from_u32(0x000001);
        assert_eq!((x + y).to_u32(), 0);
    }

}
