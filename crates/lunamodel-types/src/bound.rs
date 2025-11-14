#[derive(Debug, Clone, Copy)]
pub enum Bound {
    Bounded(f64),
    Unbounded,
}
