use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;

const A_COMMAND: u32 = 1;
const C_COMMAND: u32 = 2;
const L_COMMAND: u32 = 3;

const A_COMMAND_REGEX: &str = r"^@(.+)$";
const C_COMMAND_REGEX: &str =
    r"^(([AMD]{1,3})=)?([-!]?[AMD01])([-+&|])?([01AMD])?(;)?(J[GELNM][TQETP])?$";
const L_COMMAND_REGEX: &str = r"^\((.+)\)$";

pub struct Parser {
    pub stream: BufReader<std::fs::File>,
    pub now_line: String,
    pub command_type: Option<u32>,
}

impl Parser {
    pub fn new(f: BufReader<std::fs::File>) -> Parser {
        Parser {
            stream: f,
            now_line: "".to_string(),
            command_type: None,
        }
    }

    pub fn advance(&mut self) -> usize {
        let mut buf = String::new();
        let bytes = self.stream.read_line(&mut buf).unwrap();
        self.now_line = formatted(buf);
        self.command_type = self.command_type();
        bytes
    }

    pub fn has_more_commands(&self) -> bool {
        self.now_line != ""
    }

    pub fn command_type(&mut self) -> Option<u32> {
        if self.a_command() {
            Some(A_COMMAND)
        } else if self.c_command() {
            Some(C_COMMAND)
        } else if self.l_command() {
            Some(L_COMMAND)
        } else {
            None
        }
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

    pub fn dest(&self) -> Option<String> {
        if self.command_type != Some(2) {
            return None;
        }
        let re = Regex::new(C_COMMAND_REGEX).unwrap();
        let caps = re.captures(&self.now_line).unwrap();
        if caps.at(2) != None {
            return Some(caps.at(2).unwrap().to_string());
        }
        None
    }

    pub fn comp(&self) -> Option<String> {
        if self.command_type != Some(2) {
            return None;
        }
        let re = Regex::new(C_COMMAND_REGEX).unwrap();
        let caps = re.captures(&self.now_line).unwrap();
        let first = if caps.at(3) == None {
            "".to_string()
        } else {
            caps.at(3).unwrap().to_string()
        };
        let second = if caps.at(4) == None {
            "".to_string()
        } else {
            caps.at(4).unwrap().to_string()
        };
        let third = if caps.at(5) == None {
            "".to_string()
        } else {
            caps.at(5).unwrap().to_string()
        };
        Some(format!("{}{}{}", first, second, third))
    }

    pub fn jump(&self) -> Option<String> {
        if self.command_type != Some(2) {
            return None;
        }
        let re = Regex::new(C_COMMAND_REGEX).unwrap();
        let caps = re.captures(&self.now_line);
        match caps {
            Some(caps) => {
                if caps.at(7) != None {
                    return Some(caps.at(7).unwrap().to_string());
                }
            }
            None => return None,
        }

        None
    }

    pub fn a_command(&self) -> bool {
        let re = Regex::new(A_COMMAND_REGEX).unwrap();
        re.is_match(&self.now_line)
    }

    pub fn c_command(&self) -> bool {
        let re = Regex::new(C_COMMAND_REGEX).unwrap();
        re.is_match(&self.now_line)
    }

    pub fn l_command(&self) -> bool {
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
