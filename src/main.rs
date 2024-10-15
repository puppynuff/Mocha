// Developed by Stella / PuppyNuff
// I actually decided to put comments in this.

mod lexer;
mod parser;
mod interpreter;

use std::env;


// I hate how rust doesn't require if parenthesis, and I hate how it warns me about it.
// Dont make me follow your terrible design. Just let me do it.
// I hate rust sometimes. 
fn main() {
    let mut args : Vec<String> = env::args().collect();
    
    let mut test_lexer = false;
    let mut test_parse = false;

    for arg in &args {
        if arg == "--test-lexer" {
            test_lexer = true;
        }

        if arg == "--test-parse" {
            test_parse = true;
        }
    }

    args.push("".to_string());

    if args[1].trim() == "" || args[1].starts_with("--") {
        lexer::run_functions::run_prompt(test_parse, test_lexer);
        return;
    }

    // This will only allow one file to be ran at a time
    // I might implement the import commands later, either in my free time, or if its part of the thing I found
    // Sorry~
    match args[1].as_str() {
        "help" => {
            help();
        }

        _ => {
            lexer::run_functions::run_file(args[1].clone(), test_parse, test_lexer);
            return;
        }
    }
}


fn help() {
    println!("Mocha ~ Help");
    println!("[blank]       opens interactive terminal");
    println!("[file name]   runs file (starts at current working directory)");
    println!("--test-lexer  Shows the output from the lexer");
    println!("--test-parse  Shows the output from the parser (Note: It is not set up to output, might not make any sense)");
}