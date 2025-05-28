use super::expression::One;

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

impl One for i8 {
    fn one() -> Self {
        1
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}

impl One for i64 {
    fn one() -> Self {
        1
    }
}
