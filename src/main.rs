use nand2assembly::{code, parser, symbol_table};
use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::BufReader;

fn main() {
    const INPUT_FILE_REGEX: &str = r"^(.+)\.asm$";
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];

    let f = File::open(input_file_name).unwrap();
    let f = BufReader::new(f);

    let mut parser = parser::Parser::new(f);
    let mut symbol_table = symbol_table::SymbolTable::new();

    loop {
        let bytes = parser.advance();
        if bytes == 0 {
            break;
        }

        if !parser.has_more_commands() {
            continue;
        }

        if let Some(symbol) = parser.symbol() {
            match symbol.clone().parse::<u32>() {
                Ok(_) => (),
                Err(_) => {
                    if parser.l_command() && !symbol_table.contains(symbol.clone()) {
                        let value = format!("{:0>1$b}", symbol_table.l_variable_address, 16);
                        symbol_table.add_entry(symbol.clone(), value);
                    }
                }
            }
        }

        if parser.a_command() || parser.c_command() {
            symbol_table.l_variable_address += 1;
        }
    }

    let f = File::open(input_file_name).unwrap();
    let f = BufReader::new(f);
    parser.stream = f;
    let mut a_address = 16;
    let mut addresses: Vec<String> = Vec::new();

    loop {
        let bytes = parser.advance();
        if bytes == 0 {
            break;
        }

        if !parser.has_more_commands() {
            continue;
        }

        if let Some(symbol) = parser.symbol() {
            match symbol.clone().parse::<u32>() {
                Ok(symbol) => addresses.push(format!("{:0>1$b}", symbol, 16)),
                Err(_) => {
                    if parser.a_command() && symbol_table.contains(symbol.clone()) {
                        addresses.push(symbol_table.get_address(symbol.clone()).unwrap());
                    } else if !parser.l_command() {
                        addresses.push(format!("{:0>1$b}", a_address, 16));
                        symbol_table.add_entry(symbol.clone(), format!("{:0>1$b}", a_address, 16));
                        a_address += 1;
                    }
                }
            };
        }

        let dest = code::dest(parser.dest());
        let comp = code::comp(parser.comp());
        let jump = code::jump(parser.jump());

        if parser.c_command() {
            addresses.push(format!("111{}{}{}", comp, dest, jump));
        }
    }

    let re = Regex::new(INPUT_FILE_REGEX).unwrap();
    let caps = re.captures(input_file_name).unwrap();
    let output_file_name = caps.at(1).unwrap().to_string();

    fs::write(
        format!("tmp/{}.hack", output_file_name),
        format!("{}\n", addresses.join("\n")),
    )
    .unwrap();
}
