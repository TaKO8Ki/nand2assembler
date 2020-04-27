use nand2assembly::code;
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
                    println!("symbol: {}", symbol);
                    println!("a_address: {}", formatted_address)
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
                code::jump(node.comp().unwrap())
            } else {
                code::jump("".to_string())
            };
            if node.symbol() == None {
                println!("c_address: 111{}{}{}", comp, dest, jump);
            }

            println!("")
        },
        Err(err) => println!("Error: {:?}", err),
    };
}
