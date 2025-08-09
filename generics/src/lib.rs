 use std::fmt::Display;

pub fn identity(v: impl Display) -> impl Display {
    v
}