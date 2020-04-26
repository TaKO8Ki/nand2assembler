use nand2assembly::parser;
use std::env;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let parser = parser::parse(file_name);
    match parser {
        Ok(mut node) => loop {
            node.advance();
            if !node.has_more_commands() {
                break;
            }
            println!("line: {}", node.now_line);
            match node.command_type() {
                Some(command_type) => println!("type: {}", command_type),
                None => println!("command_type does not exist"),
            }
            match node.symbol() {
                Some(symbol) => println!("symbol: {}", symbol),
                None => println!("symbol does not exist"),
            }
        },
        Err(err) => println!("Error: {:?}", err),
    }
}
