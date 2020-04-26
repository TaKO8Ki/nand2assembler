use nand2assembly::parser;
use std::env;

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
                Some(symbol) => {
                    let address: usize = symbol.parse().unwrap();
                    let formatted_address = format!("{:0>1$b}", address, 16);
                    println!("symbol: {}, address: {}", symbol, formatted_address);
                }
                None => println!("symbol does not exist"),
            }

            match node.dest() {
                Some(dest) => println!("dest: {}", dest),
                None => println!("dest does not exist"),
            }

            match node.comp() {
                Some(comp) => println!("comp {}", comp),
                None => println!("comp does not exist"),
            }

            match node.jump() {
                Some(jump) => println!("jump {}", jump),
                None => println!("jump does not exist"),
            }

            println!("")
        },
        Err(err) => println!("Error: {:?}", err),
    };
    let hoge: usize = "100".parse().unwrap();
    let test = format!("{:0>1$b}", hoge, 16);
    println!("{}", test);
}
