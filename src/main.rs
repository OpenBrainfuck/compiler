extern crate clap;

use clap::{App, Arg};

use std::fs::{File, self};
use std::io::{Write, ErrorKind};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,       // +
    Sub,       // -
    Right,     // >
    Left,      // <
    Read,      // ,
    Write,     // .
    BeginLoop, // [
    EndLoop,   // ]
}

use self::Token::*;

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {}
        }
    }
    tokens
}

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.c"));
    for &token in tokens {
        match token {
            Add => {
                // Increment the value at the selected cell
                output.push_str("++*ptr;\n");
            }
            Sub => {
                // Decrement the value at the selected cell
                output.push_str("--*ptr;\n");
            }
            Right => {
                // Change our selected cell to the next to the right
                output.push_str("++ptr;\n");
            }
            Left => {
                // Change our selected cell to the next to the left
                output.push_str("--ptr;\n");
            }
            Read => {
                // Read a single character into the selected cell
                output.push_str("*ptr=getchar();\n");
            }
            Write => {
                // Print the character at the selected cell
                output.push_str("putchar(*ptr);\n");
            }
            BeginLoop => {
                // Begin a loop at the current cell
                output.push_str("while (*ptr) {\n");
            }
            EndLoop => {
                // Close a loop
                output.push_str("}\n");
            }
        }
    }
    output.push_str("}\n");
    output
}


fn main() {
    let matches = App::new("obf")
                        .version("1.0.0")
                        .author("OpenBrainfuck")
                        .about("The open source brainfuck compiler.")
                        .arg(Arg::with_name("INPUT")
                            .help(".obf source file to compile")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("output")
                            .short('o')
                            .help("output filename")
                            .takes_value(true)
                            .required(true))
                        .get_matches();
    

    let f = File::open(matches.value_of("output").unwrap());

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(matches.value_of("output").unwrap()) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let contents = fs::read_to_string(matches.value_of("INPUT").unwrap())
        .expect("Something went wrong reading the OpenBrainfuck script file");

    let tokens = tokenize(&contents);

    let info = generate(&tokens);
    println!("{}", generate(&tokens));
    f.write_all(info.as_bytes()).expect("Could not write");
     

}