use nand2assembly::parser;
use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::BufReader;

fn main() {
    const FILE_REGEX: &str = r"^(.+)\.asm$";

    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];
    let re = Regex::new(FILE_REGEX).unwrap();
    let caps = re.captures(input_file_name).unwrap();
    let output_file_name = caps.at(1).unwrap().to_string();
    let f = File::open(input_file_name).unwrap();
    let f = BufReader::new(f);
    let parser = parser::parse(f).unwrap();
    fs::write(
        format!("tmp/{}.hack", output_file_name),
        format!("{}\n", parser.join("\n")),
    )
    .unwrap();
}
