use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub struct Parser {
    pub stream: std::io::BufReader<std::fs::File>,
    pub now_line: String,
    pub command_type: String,
}

pub fn parse(file_name: &str) -> io::Result<(Parser)> {
    let f = File::open(file_name)?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();

    let mut node = Parser {
        stream: f,
        now_line: "\n".to_string(),
        command_type: "".to_string(),
    };

    Ok((node))
}

impl Parser {
    pub fn advance(&mut self) {
        let mut buf = String::new();
        self.stream
            .read_line(&mut buf)
            .expect("reading from cursor won't fail")
            .to_string();
        self.now_line = buf
    }

    pub fn has_more_commands(&self) -> bool {
        self.now_line != ""
    }
}
