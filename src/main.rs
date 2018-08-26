#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate pretty_assertions;
use std::{env, io};

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    let user = env::var("LOGNAME").unwrap();
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");
    let stdin = io::stdin();
    repl::start(stdin.lock(), io::stdout());
}
