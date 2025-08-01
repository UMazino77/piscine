use std::fs::{self, File};

pub fn open_file(s: &str) -> File {
    let op_file = File::open(s);

    return match op_file {
        Ok(f) => f,
        Err(err) => panic!("{:?}",err),
    };

}