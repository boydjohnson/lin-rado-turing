use lin_rado_turing::{
    machine::Machine,
    program::{Program, ProgramParseError},
    types::{FourState, ThreeState, TwoState, TwoSymbol},
};
use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
    process::exit,
};

fn main() {
    let mut args = std::env::args().skip(1);

    let program = match args.next() {
        Some(prog) => match prog.parse::<Program<FourState, TwoSymbol>>() {
            Ok(p) => p,
            Err(ProgramParseError::Error(msg)) => {
                println!("Failed to parse program: {}", msg);
                exit(1);
            }
        },
        None => {
            println!("usage: '<program>' <num steps: optional>");
            exit(1);
        }
    };

    let limit = match args.next() {
        Some(a) => match a.parse() {
            Ok(limit) => limit,
            Err(e) => {
                println!("Failed to parse second arg: {}", e);
                exit(1);
            }
        },
        None => 1000,
    };

    let file = match args.next() {
        Some(f) => match OpenOptions::new().append(true).create(true).open(f) {
            Ok(file) => Some(BufWriter::with_capacity(1_000, file)),
            Err(e) => {
                println!("Failed to open or create file: {}", e);
                exit(1)
            }
        },
        None => None,
    };

    let mut machine = Machine::new(program);

    machine.run_until_halt(vec![], limit, file);
}
