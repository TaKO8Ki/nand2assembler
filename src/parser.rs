use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

const A_COMMAND: i32 = 1;
const C_COMMAND: i32 = 2;
const L_COMMAND: i32 = 3;

const A_COMMAND_REGEX: &str = r"^@(.+)$";
const C_COMMAND_REGEX: &str =
    r"^(([AMD]{1,3})=)?([-!]?[AMD01])([-+&|])?([01AMD])?(;)?(J[GELNM][TQETP])?$";
const L_COMMAND_REGEX: &str = r"^\((.+)\)$";

pub struct Parser {
    pub stream: std::io::BufReader<std::fs::File>,
    pub now_line: String,
    pub command_type: Option<i32>,
}

pub fn parse(file_name: &str) -> io::Result<Parser> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let node = Parser {
        stream: f,
        now_line: "\n".to_string(),
        command_type: None,
    };

    Ok(node)
}

impl Parser {
    pub fn advance(&mut self) {
        let mut buf = String::new();
        self.stream
            .read_line(&mut buf)
            .expect("reading from cursor won't fail")
            .to_string();
        self.now_line = formatted(buf)
    }

    pub fn has_more_commands(&self) -> bool {
        self.now_line != ""
    }

    pub fn command_type(&mut self) -> Option<i32> {
        if self.a_command() {
            self.command_type = Some(A_COMMAND);
        } else if self.c_command() {
            self.command_type = Some(C_COMMAND);
        } else if self.l_command() {
            self.command_type = Some(L_COMMAND);
        }
        self.command_type
    }

    pub fn symbol(&self) -> Option<String> {
        if self.command_type == Some(1) {
            let re = Regex::new(A_COMMAND_REGEX).unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.command_type == Some(3) {
            let re = Regex::new(L_COMMAND_REGEX).unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        }
        None
    }

    pub fn dest(&self) {}

    pub fn comp(&self) {}

    pub fn jump(&self) {}

    fn a_command(&self) -> bool {
        let re = Regex::new(A_COMMAND_REGEX).unwrap();
        re.is_match(&self.now_line)
    }

    fn c_command(&self) -> bool {
        let re = Regex::new(C_COMMAND_REGEX).unwrap();
        re.is_match(&self.now_line)
    }

    fn l_command(&self) -> bool {
        let re = Regex::new(L_COMMAND_REGEX).unwrap();
        re.is_match(&self.now_line)
    }
}

fn formatted(str: String) -> String {
    let comment_re = Regex::new(r"//.+").unwrap();
    comment_re
        .replace_all(&str, "")
        .replace("\n", "")
        .replace(" ", "")
}
