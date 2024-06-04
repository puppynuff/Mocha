use std::{self, fs, io::Write};

use super::scanner;

// For running the file
// When you want to do stuff fast
pub fn run_file(path: String) {
    if !std::fs::metadata(&path).expect("Inputted path does not exist!\n").is_file() {
        panic!("Inputted path is not a file!");
    }

    let file_content = fs::read_to_string(&path).expect("Failed to read file content!");

    run(file_content);
}

// I have to do this to get rid of the stupid warning
// For the miniscule amount of people who use the interactive prompt terminal thing
#[allow(while_true)]
pub fn run_prompt() {
    while true {
        let mut input = String::new();
        print!("> "); // Writing the input notifier

        // Why must input in rust be so damn complicated
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).expect("Did not enter a string");

        // Allowing you to exit without the "Failed to exit correctly! Error"
        if input.trim() == "exit()" {
            println!("Exiting!");
            return;
        }

        // Everything comes down to this
        run(input.to_string().to_owned())
    }
}

pub fn run(source: String) {
    // Yea... for everything redirecting here its kindof sad.
    let mut current_scanner = scanner::Scanner::new(source);

    let tokens = current_scanner.scan_tokens();

    // Just for now before the language can actually do shit.
    // Currently a rock is just as useful for programming
    for mut token in tokens {
        println!("{}", token.as_string());
    }

    return;
}