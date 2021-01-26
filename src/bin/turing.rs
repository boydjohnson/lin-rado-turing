use std::process::exit;

use lin_rado_turing::{
    machine::Machine,
    program::{Program, ProgramParseError},
    types::{FourState, ThreeState, TwoState, TwoSymbol},
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

    let mut machine = Machine::new(program);

    machine.run_until_halt(vec![], limit);
}
