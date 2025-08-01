use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self{short_hand : ("-".to_owned()+&name[0..1]).to_owned(), long_hand : ("--".to_owned()+&name).to_string(), desc : d.to_owned()}
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);

    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let aa =self.flags.get(input).ok_or_else(|| format!("Error : {}", input))?;
        if argv.len() <= 1 {
            return Err("error".to_string());
        }
       aa(argv[0], argv[1]).map_err(|e| format!("Error : {}", e))
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let aa: f64 = a.parse()?;
    let bb:f64 = b.parse()?;

    Ok((aa/bb).to_string()) 
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
        let aa: f64 = a.parse()?;
    let bb:f64 = b.parse()?;

    return match bb {
        _ => Ok((aa%bb).to_string())
    } 
}