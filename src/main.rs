use nand2assembly::parser;
use std::env;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];
    let parser = parser::parse("hosts.txt");
    match parser {
        Ok(mut node) => loop {
            node.advance();
            if !node.has_more_commands() {
                break;
            }
            println!("{}", node.now_line);
        },
        Err(err) => println!("Error: {:?}", err),
    }
}
