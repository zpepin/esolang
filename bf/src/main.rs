use std::fs;
use std::io::{self, Read};
use clap::Parser as ClapParser;
use pest::Parser as PestParser;
use pest_derive::Parser as PestDeriveParser;

#[derive(ClapParser)]
#[command(name = "esolang", about = "A Rust-based interpreter for esoteric programming languages")]
struct Cli {
    /// program read from script file
    file: String
}

#[derive(PestDeriveParser)]
#[grammar = "grammar.pest"]
struct BrainfuckParser;

pub struct Tape {
    cells: Vec<u8>,
    head: usize,
    left: usize,
    right: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            cells: vec![0; 1],
            head: 0,
            left: 0,
            right: 0,
        }
    }

    pub fn read(&self) -> u8 {
        self.cells[self.head]
    }

    pub fn write(&mut self, value: u8) {
        self.cells[self.head] = value;
    }

    pub fn move_right(&mut self) {
        if self.head == self.right {
            if self.is_full() {
                self.expand();
            }
            self.right = (self.right + 1) % self.cells.len();
        }
        if self.cells[self.left] == 0 {
            self.left = (self.left + 1) % self.cells.len();
        }
        self.head = (self.head + 1) % self.cells.len();
    }

    pub fn move_left(&mut self) {
        if self.head == self.left {
            if self.is_full() {
                self.expand();
            }
            self.left = (self.left + self.cells.len() - 1) % self.cells.len();
        }
        if self.cells[self.right] == 0 {
            self.right = (self.right + self.cells.len() - 1) % self.cells.len();
        }
        self.head = (self.head + self.cells.len() - 1) % self.cells.len();
    }

    fn is_full(&self) -> bool {
        (self.right + 1) % self.cells.len() == self.left
    }

    fn expand(&mut self) {
        let mut new_cells = vec![0; self.cells.len() * 2];
        if self.left <= self.right {
            new_cells[..self.cells.len()].copy_from_slice(&self.cells);
        } else {
            let right_part_len = self.cells.len() - self.right;
            new_cells[..right_part_len].copy_from_slice(&self.cells[self.right..]);
            new_cells[right_part_len..right_part_len + self.left].copy_from_slice(&self.cells[..self.left]);
        }
        
        self.cells = new_cells;
    }
}

fn main() {
    let args = Cli::parse();

    let content = fs::read_to_string(args.file).expect("Failed to read the file");
    let program = BrainfuckParser::parse(Rule::program, &content)
        .expect("Failed to parse the program")
        .next()
        .unwrap();

    // Initialize the tape
    let mut tape = Tape::new();
    
    // Execute the program
    let mut stack = vec![program.clone()];
    while let Some(pair) = stack.pop() {
        match pair.as_rule() {
            Rule::increment => {
                tape.write(tape.read().wrapping_add(1));
            }

            Rule::decrement => {
                tape.write(tape.read().wrapping_sub(1));
            }

            Rule::move_right => {
                tape.move_right();
            }

            Rule::move_left => {
                tape.move_left();
            }

            Rule::output => {
                print!("{}", tape.read() as char);
            }

            Rule::input => {
                if let Some(Ok(byte)) = io::stdin().bytes().next() {
                    tape.write(byte);
                }
            }

            Rule::loop_block => {
                if tape.read() != 0 {
                    stack.push(pair.clone());
                    for inner_pair in pair.into_inner().rev() {
                        stack.push(inner_pair.clone());
                    }
                }
            }
            
            _ => {
                for inner_pair in pair.into_inner().rev() {
                    stack.push(inner_pair.clone());
                }
            }
        }
    }
}
