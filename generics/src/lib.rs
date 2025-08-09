 use std::fmt::Debug;

pub fn identity(v: impl Debug) -> impl Debug {
    v
}