#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    Bounded(f64),
    Unbounded,
}
