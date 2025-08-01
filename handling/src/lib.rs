use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let op_file = OpenOptions::new().create(true).append(true).open(path);

    let _ = match op_file {
        Ok(mut f) => f.write_all(content.as_bytes()) ,
        Err(err) => panic!("{:?}",err),
    };
}