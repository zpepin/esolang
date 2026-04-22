use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(name = "slashalash", about = "An interpreter for the /// or Slashalash programming language.")]
struct Cli {
    /// program read from script file
    file: String
}

#[derive(PartialEq, Eq)]
enum ParseState {
    Output,
    Pattern,
    Replacement,
    Substitution,
}

fn main() {
    let args = Cli::parse();

    let mut program = fs::read_to_string(args.file).expect("Failed to read the file");
    while program.len() > 0 {
        let mut pattern = String::new();
        let mut replacement = String::new();
        let mut new_program = String::new();

        let mut do_escape = false;
        let mut state = ParseState::Output;
        for c in program.chars() {
            if state == ParseState::Substitution {
                new_program.push(c);
            } else if !do_escape && c == '\\' {
                do_escape = true;
            } else if !do_escape && c == '/' {
                state = match state {
                    ParseState::Output => ParseState::Pattern,
                    ParseState::Pattern => ParseState::Replacement,
                    ParseState::Replacement => ParseState::Substitution,
                    ParseState::Substitution => ParseState::Substitution,
                };
            } else {
                match state {
                    ParseState::Output => print!("{}", c),
                    ParseState::Pattern => pattern.push(c),
                    ParseState::Replacement => replacement.push(c),
                    ParseState::Substitution => new_program.push(c),
                }
                do_escape = false;
            }
        }

        if state == ParseState::Substitution {
            program = new_program;
            while program.find(&pattern) != None {
                program = program.replacen(&pattern, &replacement, 1);
            }
        } else {
            program = "".to_owned();
        }
    }
}
