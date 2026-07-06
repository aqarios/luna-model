mod vtype;

pub trait LmFrom<T> {
    fn from(value: T) -> Self;
}
