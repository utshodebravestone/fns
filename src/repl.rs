use std::io::{stdin, stdout, Write};

use crate::{
    frontend::{parser::parse, tokenizer::tokenize, utils::Error},
    runtime::{environment::Environment, interpreter::evaluate},
};

pub fn repl() {
    let mut source_code = String::new();
    let mut environment = Environment::new(None);
    let mut stdout = stdout();
    let stdin = stdin();

    println!("fns repl v0.0.1");
    println!("press [ctrl + c] to exit\n");
    loop {
        print!("fns ⇒  ");
        stdout.flush().expect("Error: Could not flush <stdout>.");
        stdin
            .read_line(&mut source_code)
            .expect("Error: Could not read from <stdin>.");
        match run(&source_code, environment.clone()) {
            Ok(old_environment) => environment = old_environment,
            Err(error) => error.report(&source_code),
        }
        source_code.clear();
    }
}

fn run(source_code: &str, environment: Environment) -> Result<Environment, Error> {
    let tokens = tokenize(source_code)?;
    let program = parse(tokens)?;
    let (value, environment) = evaluate(program, Some(environment))?;
    println!("{value}");
    Ok(environment)
}
