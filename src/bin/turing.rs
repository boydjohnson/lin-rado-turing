use clap::{App, Arg};
use lin_rado_turing::{
    machine::run_machine,
    program::{parse_program, ProgramParseError, ProgramT},
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    process::exit,
    str::FromStr,
};

fn main() {
    let args = parse_args();

    let complexity = args
        .value_of("complexity")
        .expect("required field missing.");

    let prog_str = args.value_of("program").expect("program is required");

    let program = match parse_program(prog_str, complexity) {
        Ok(p) => p,
        Err(ProgramParseError::Error(msg)) => {
            writeln!(
                std::io::stderr(),
                "Error parsing program or complexity: {}",
                msg
            )
            .expect("Unable to write to stderr");
            exit(1);
        }
    };

    let check = match args.value_of("check-recurrence") {
        Some(s) => match s.parse() {
            Ok(check) => Some(check),
            Err(e) => {
                writeln!(std::io::stderr(), "Error parsing check-recurrence: {}", e)
                    .expect("Unable to write to stderr");
                exit(1);
            }
        },
        None => None,
    };

    let verbose = args.is_present("verbose");

    let output: Option<Box<dyn Write>> = match args.value_of("output") {
        Some(o) => match o {
            "-" => Some(Box::new(BufWriter::with_capacity(1_000, std::io::stdout()))),
            a => match OpenOptions::new().append(true).create(true).open(a) {
                Ok(file) => Some(Box::new(BufWriter::with_capacity(1_000, file))),
                Err(e) => {
                    writeln!(std::io::stderr(), "Failed to open file: {}", e)
                        .expect("Unable to write to stderr");
                    exit(1);
                }
            },
        },
        None => None,
    };

    let limit = match args
        .value_of("limit")
        .map_or_else(|| Ok(10000), |s| <usize as FromStr>::from_str(s))
    {
        Ok(l) => l,
        Err(e) => {
            writeln!(std::io::stderr(), "Error parsing limit: {}", e)
                .expect("Unable to write to stderr");
            exit(1);
        }
    };

    match program {
        ProgramT::TwoTwo(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::TwoThree(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::TwoFour(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::ThreeTwo(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::ThreeThree(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::ThreeFour(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::FourTwo(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::FourThree(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::FourFour(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::FiveTwo(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::FiveThree(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::FiveFour(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::SixTwo(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::SixThree(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
        ProgramT::SixFour(program) => {
            run_machine(program, prog_str, limit, output, verbose, check);
        }
    }
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    App::new("turing")
        .about("Turing Machine VM")
        .arg(
            Arg::with_name("complexity")
                .required(true)
                .help("The number of states and number of symbols. eg 3-2, 4-2, 2-4..."),
        )
        .arg(
            Arg::with_name("check-recurrence")
                .short("c")
                .long("check")
                .takes_value(true)
                .number_of_values(1)
                .help("Run the recurrence check, taking more time"),
        )
        .arg(
            Arg::with_name("program")
                .required(true)
                .help("The Turing program. eg 1RB 0LA 1RB 0LH"),
        )
        .arg(Arg::with_name("output").help("Filename to write output to or - for stdout."))
        .arg(
            Arg::with_name("limit")
                .help("Number of steps to limit the VM to.")
                .long("limit")
                .takes_value(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Log each step's state and symbol."),
        )
        .get_matches()
}
