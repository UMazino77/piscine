pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_owned(),
        Security::Message => server.unwrap_or_else(|_| {panic!("ERROR: program stops")    }).to_owned(),
        Security::Warning => server.unwrap_or_else(|_| "WARNING: check the sever").to_owned(),
        Security::NotFound => {
            match server {
                Ok(data) => data.to_owned(),
                Err(err) => format!("Not found: {}", err),
            }
        },
        Security::UnexpectedUrl => server.unwrap_err().to_owned(),

    }
}