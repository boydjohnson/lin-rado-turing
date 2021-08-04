use lin_rado_turing::{
    machine::Machine,
    program::{parse_program, Program, ProgramT},
    types::{State, Symbol},
};

const HALTS: &[(&str, usize, usize, &str)] = &[
    // 2/2 BB
    ("1RB 1LB 1LA 1RH", 4, 6, "2-2"),
    // 3/2 BB
    ("1RB 1RH 1LB 0RC 1LC 1LA", 5, 21, "3-2"), // shift
    ("1RB 1LC 1RC 1RH 1LA 0LB", 6, 11, "3-2"), // sigma
    // 2/3 BB
    ("1RB 2LB 1RH 2LA 2RB 1LB", 9, 38, "2-3"),
    // 4/2 BB
    ("1RB 1LB 1LA 0LC 1RH 1LD 1RD 0RA", 13, 107, "4-2"), // shift
    ("1RB 0RC 1LA 1RA 1RH 1RD 1LD 0LB", 13, 96, "4-2"),  // sigma
    // 2/4 Runners-up
    ("1RB 3LA 1LA 1RA 2LA 1RH 3RA 3RB", 90, 7195, "2-4"),
    ("1RB 3LA 1LA 1RA 2LA 1RH 3LA 3RB", 84, 6445, "2-4"),
    ("1RB 3LA 1LA 1RA 2LA 1RH 2RA 3RB", 84, 6445, "2-4"),
    ("1RB 2RB 3LA 2RA 1LA 3RB 1RH 1LB", 60, 2351, "2-4"),
    // Milton Green (1964)
    ("1RB 1LA 0LH 1RB", 1, 2, "2-2"),
    (
        "1RB 1LH 0RC 1RC 0RD 0RC 1RE 1LA 0RF 0RE 1LF 1LD",
        35,
        436,
        "6-2",
    ),
    // Lynn (1971)
    ("1RB 1RA 1LC 0LD 0RA 1LB 1RH 0LE 1RC 1RB", 15, 435, "5-2"),
    ("1RB 1RC 1LC 1LD 0RA 1LB 1RE 0LB 1RH 1RD", 22, 292, "5-2"),
    ("1RB 0RC 1LC 0LB 1RD 1LB 1RE 0RA 0RB 1RH", 22, 217, "5-2"),
    // Lynn reports 522 steps
    (
        "1RB 0LB 1LC 1RH 0LD 0LC 1LE 0RA 0LF 0LE 1RF 1RD",
        42,
        521,
        "6-2",
    ),
    // Uwe (1981)

    // Castor diligentissimus et primus et perpetuus (Castor schultis)
    (
        "1RB 0LC 1RC 1RD 1LA 0RB 0RE 1RH 1LC 1RA",
        501,
        134467,
        "5-2",
    ),
    // Castor ministerialis: the Civil Servant Beaver, who cares most
    // for his progress, but does not produce anything.
    ("1RB 1RA 1RC 0RE 1LD 0RA 1LB 1LD 0RH 0RB", 0, 52, "5-2"),
    // Castor scientificus: the Scientific Beaver, who does not produce
    // anything either, but with more effort and less effect on his
    // position.
    ("0RB 0LA 0RC 0RH 1RD 1LE 1LA 0LD 1RC 1RE", 0, 187, "5-2"),
    // Castor exflippus: the Beaver Freak, who tries to survive as long
    // as possible without producing anything, moving on the tape, and
    // changing his state.
    ("0RB 0LA 1RC 0RH 0LC 1RD 0LD 1RE 1LA 0LE", 0, 67, "5-2"),
];

const HALTS_SLOWLY: &[(&str, usize, usize, &str)] = &[
    // Slow halting
    // 3/3 Surprise-in-a-box
    ("1RB 2LB 1LC 1LA 2RB 1RB 1RH 2LA 0LC", 31, 2315619, "3-3"),
    // 2/4 BB
    ("1RB 2LA 1RA 1RA 1LB 1LA 3RB 1RH", 2050, 3932964, "2-4"),
    // 3/3 copy of 2/4 BB
    ("1RB 1LC 1RH 1LA 1LC 2RB 1RB 2LC 1RC", 2050, 3932964, "3-3"),
    // 5/2 BB
    (
        "1RB 1LC 1RC 1RB 1RD 0LE 1LA 1LD 1RH 0LA",
        4098,
        47176870,
        "5-2",
    ),
];

#[test]
fn test_machine_halts() {
    for &(prog_str, marks, steps, complexity) in HALTS {
        parse_program_and_assert(prog_str, marks, steps, complexity, false);
    }
}

#[test]
#[cfg_attr(not(feature = "slow-tests"), ignore)]
fn test_machine_halts_slowly() {
    for &(prog_str, marks, steps, complexity) in HALTS_SLOWLY {
        parse_program_and_assert(prog_str, marks, steps, complexity, false);
    }
}

fn assert_machine<S: State + Send + Sync, Sym: Symbol + Send + Sync>(
    prog: Program<S, Sym>,
    marks: usize,
    steps: usize,
    parallel: bool,
) {
    let mut machine = Machine::new(prog);

    machine.run_until_halt::<std::io::Stdout>(vec![], steps, &mut None, None, None, parallel);

    let halt = machine.halt();

    assert!(halt.is_some());

    let halt = halt.unwrap();

    assert_eq!(halt.steps, steps);

    assert_eq!(machine.marks(), marks);
}

fn parse_program_and_assert(
    prog_str: &str,
    marks: usize,
    steps: usize,
    complexity: &str,
    parallel: bool,
) {
    let program = parse_program(prog_str, complexity).unwrap();

    match program {
        ProgramT::TwoTwo(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::TwoThree(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::TwoFour(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::ThreeTwo(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::ThreeThree(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::ThreeFour(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::FourTwo(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::FourThree(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::FourFour(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::FiveTwo(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::FiveThree(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::FiveFour(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::SixTwo(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::SixThree(prog) => assert_machine(prog, marks, steps, parallel),
        ProgramT::SixFour(prog) => assert_machine(prog, marks, steps, parallel),
    }
}
