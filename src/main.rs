mod frontend;
mod repl;
mod runtime;

use std::{env::args, fs::read_to_string, process::exit};

use frontend::utils::Error;
use runtime::evaluator::evaluate;

use crate::{
    frontend::{parser::parse, tokenizer::tokenize},
    repl::repl,
};

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => repl(),
        2 => {
            let source_code = read_to_string(&args[1])
                .expect("Error: Could not read source code file from given path.");
            run(&source_code).unwrap_or_else(|error| {
                error.report(&source_code);
            });
        }
        _ => {
            eprintln!("Error: Unknown number of argument.\nUsage: yai <filename>");
            exit(65);
        }
    }
}

fn run(source_code: &str) -> Result<(), Error> {
    let tokens = tokenize(source_code)?;
    let program = parse(tokens)?;
    evaluate(program, None)?;
    Ok(())
}
