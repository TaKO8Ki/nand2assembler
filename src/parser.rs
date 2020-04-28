use crate::code;
use regex::Regex;
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
    pub stream: BufReader<std::fs::File>,
    pub now_line: String,
    pub command_type: Option<i32>,
}

pub fn parse(f: BufReader<std::fs::File>) -> io::Result<Vec<String>> {
    let mut addresses: Vec<String> = Vec::new();
    let mut node = Parser {
        stream: f,
        now_line: "\n".to_string(),
        command_type: None,
    };

    loop {
        node.advance();
        if !node.has_more_commands() {
            break;
        }

        match node.symbol() {
            Some(symbol) => {
                let address: usize = symbol.parse().unwrap();
                let formatted_address = format!("{:0>1$b}", address, 16);
                addresses.push(formatted_address)
            }
            None => (),
        }

        let dest = if node.dest() != None {
            code::dest(node.dest().unwrap())
        } else {
            code::dest("".to_string())
        };
        let comp = if node.comp() != None {
            code::comp(node.comp().unwrap())
        } else {
            code::comp("".to_string())
        };
        let jump = if node.jump() != None {
            code::jump(node.jump().unwrap())
        } else {
            code::jump("".to_string())
        };
        if node.symbol() == None {
            let c_address = format!("111{}{}{}", comp, dest, jump);
            addresses.push(c_address);
        }
    }

    Ok(addresses)
}

impl Parser {
    pub fn advance(&mut self) {
        let mut buf = String::new();
        self.stream
            .read_line(&mut buf)
            .expect("reading from cursor won't fail")
            .to_string();
        self.now_line = formatted(buf);
        self.command_type().unwrap();
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
