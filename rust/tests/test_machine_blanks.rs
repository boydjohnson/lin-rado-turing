use lin_rado_turing::{
    machine::{HaltReason, Machine},
    program::{parse_program, Program, ProgramT},
    types::{State, Symbol},
};

const BLANK_FAST: &[(&str, usize, &str)] = &[
    // 2/2
    ("1RB 0RA  1LB 1LA", 8, "2-2"),
    ("1RB 0RA  0LB 1LA", 7, "2-2"),
    ("1RB 1LA  0LA 0LB", 6, "2-2"),
    ("1RB 0LA  1LB 1RA", 5, "2-2"),
    ("1RB 1RB  1LA 0LB", 5, "2-2"),
    // 3/2
    ("1RB 1LB  1LA 1LC  1RC 0LC", 34, "3-2"),
    ("1RB 1LC  1LB 1LA  1RC 0LC", 27, "3-2"),
    ("1RB 1LB  1LA 1RC  1LC 0RC", 26, "3-2"),
    ("1RB 1LB  1LA 0LC  1RC 0LC", 25, "3-2"),
    ("1RB 0RB  1LC 1RC  0LA 1LA", 25, "3-2"),
    ("1RB 0RB  1LC 0LC  1LA 1RA", 23, "3-2"),
    ("1RB 0LB  1LA 1LC  1RC 0LC", 23, "3-2"),
    ("1RB 1LB  1LA 1RC  0RB 0LC", 22, "3-2"),
    ("1RB 1LB  0RC 1LA  1LA 0RA", 21, "3-2"),
    ("1RB 1LA  1LA 1RC  1LC 0RC", 20, "3-2"),
    ("1RB 1LA  1LA 1LC  1RC 0LC", 20, "3-2"),
    ("1RB 0LC  1LB 1LA  1RC 0LC", 20, "3-2"),
    ("1RB 0LB  1LA 1LC  0RC 0RB", 20, "3-2"),
    ("1RB 1RC  1LC 0LB  1RA 1LA", 16, "3-2"),
    ("1RB 1LB  0LC 0RB  1RA 1LA", 14, "3-2"),
    // 2/3
    ("1RB 2LA 0RB  1LA 0LB 1RA", 77, "2-3"),
    ("1RB 2RA 2RB  2LB 1LA 0RB", 29, "2-3"),
    ("1RB 2RB 0RA  2LA 1LA 1LB", 27, "2-3"),
    ("1RB 2RB 2RA  2LB 1LA 0RB", 24, "2-3"),
    ("1RB 1LA 2RB  2LA 2RA 0LB", 24, "2-3"),
    ("1RB 0RB 1LA  2LA 2RA 0LB", 4, "2-3"),
    // 4/2
    ("1RB 0LC  1LD 0LA  1RC 1RD  1LA 0LD", 66345, "4-2"),
    ("1RB 1RA  0RC 0RB  0RD 1RA  1LD 1LB", 2566, "4-2"),
    ("1RB 1RA  0RC 1LA  1LC 1LD  0RB 0RD", 2510, "4-2"),
    ("1RB 0RB  1LC 0LC  1RA 0LD  1LB 0LB", 976, "4-2"),
    ("1RB 1LC  1LA 0RD  0RD 0RC  1LD 1LA", 711, "4-2"),
    ("1RB 1LD  1LC 0RC  1LC 1LA  0RC 0RD", 709, "4-2"),
    ("1RB 1LC  1RC 0RD  0RD 0RC  1LD 1LA", 704, "4-2"),
    ("1RB 1LC  0RC 0RD  0RD 0RC  1LD 1LA", 702, "4-2"),
    ("1RB 1LC  1LA 1RB  0RD 0RC  1LD 1LA", 534, "4-2"),
    ("1RB 1LA  0LC 0LB  1RC 1RD  1LA 1RB", 495, "4-2"),
    ("1RB 1LC  0RC 1RB  0RD 0RC  1LD 1LA", 455, "4-2"),
    ("1RB 1RA  0RC 0RB  1LC 1LD  1RA 1LB", 426, "4-2"),
    ("1RB 1RA  1LC 0RD  1LB 1LD  1RA 0RB", 319, "4-2"),
    // constructed from BB(3) sigma champ
    ("1RB 1LC  1RC 1LD  1LA 0LB  1RD 0LD", 77, "4-2"),
    // constructed from BB(3) shift champ
    ("1RB 1LD  1LB 0RC  1LC 1LA  1RD 0LD", 66, "4-2"),
    ("1RB 0RA  0LB 0LC  1RD 1LC  1RA 1LB", 3, "4-2"),
    ("1RB 1RC  0LD 1RA  1LB 0RD  1LA 0RC", 3, "4-2"),
    // 5/2
    ("1RB 1LC  0LC 0RD  1RD 1LE  1RE 1LA  1LA 0LB", 31315, "5-2"),
    ("1RB 1LC  1RD 1RA  1LB 0LA  1RE 0RC  1RC 0LE", 3241, "5-2"),
    ("1RB 1RC  1LD 0RA  0RB 1RA  1LE 1LD  1RA 0RE", 725, "5-2"),
    ("1RB 1LC  1RC 1LD  1RE 1RD  0RE 0LA  1LB 0LE", 362, "5-2"),
    ("1RB 1RC  0RC 1LD  1LE 1RD  0LE 1RA  1LB 0LC", 277, "5-2"),
    ("1RB 1LC  1LC 0LD  1LE 1LD  1RE 0RA  0RB 1LA", 134, "5-2"),
    ("1RB 1LC  1LD 1RA  0LB 1LE  0LE 1LC  1RA 0RE", 127, "5-2"),
    ("1RB 1LC  1RD 0RE  1LA 1LE  1RC 0LE  1RE 0LD", 123, "5-2"),
    // 2/4
    ("1RB 2RA 1RA 2RB  2LB 3LA 0RB 2LA", 190524, "2-4"),
    ("1RB 2RA 3LA 2RB  2LB 1LA 0RB 0RA", 2501, "2-4"),
    ("1RB 2RA 1RA 2LB  2LB 3LA 0RB 0RA", 1612, "2-4"),
    ("1RB 2RB 1LA 0LA  2LB 3LA 0RB 1RA", 888, "2-4"),
    ("1RB 2RA 1RA 2LB  2LB 3LA 0RB 2LA", 759, "2-4"),
    ("1RB 2RA 1LA 0RB  2LB 3LA 0RB 2RA", 604, "2-4"),
    ("1RB 2RA 1LA 2LB  2LB 3LA 0RB 2RA", 301, "2-4"),
    ("1RB 2RB 2RA 1LA  2LB 3LA 0RB 1RA", 281, "2-4"),
    ("1RB 2LA 0RA 1LA  3LA 0LB 1RA 2LA", 239, "2-4"),
    ("1RB 2LB 3LA 0RA  1LA 3RB 3LB 2RA", 224, "2-4"),
    ("1RB 2RB 1LA 1LB  2LB 3LA 0RB 2RA", 158, "2-4"),
    ("1RB 1LA 2RB 1LB  3LA 0LB 1RA 2RA", 91, "2-4"),
    ("1RB 1LA 0LB 2RB  2LA 3RA 1LB 0LA", 27, "2-4"),
];

#[test]
fn test_machine_blanks() {
    for (prog_str, steps, complexity) in BLANK_FAST {
        println!("{}", prog_str);
        parse_program_and_assert(prog_str, *steps, complexity);
    }
}

fn assert_machine<S: State + Send + Sync + ToString, Sym: Symbol + Send + Sync + ToString>(
    prog: Program<S, Sym>,
    steps: usize,
) {
    let mut machine = Machine::new(prog);

    let blank = Some(0);

    machine.run_until_halt::<std::io::Stdout>(vec![], steps + 20, &mut None, None, blank, false);

    let halt = machine.halt();

    assert!(halt.is_some());

    let halt = halt.unwrap();

    assert_eq!(halt.reason, HaltReason::Blanking);

    assert_eq!(halt.steps, steps);

    assert_eq!(machine.marks(), 0);
}

fn parse_program_and_assert(prog_str: &str, steps: usize, _complexity: &str) {
    let program = parse_program(prog_str).unwrap();

    match program {
        ProgramT::TwoTwo(prog) => assert_machine(prog, steps),
        ProgramT::TwoThree(prog) => assert_machine(prog, steps),
        ProgramT::TwoFour(prog) => assert_machine(prog, steps),
        ProgramT::ThreeTwo(prog) => assert_machine(prog, steps),
        ProgramT::ThreeThree(prog) => assert_machine(prog, steps),
        ProgramT::ThreeFour(prog) => assert_machine(prog, steps),
        ProgramT::FourTwo(prog) => assert_machine(prog, steps),
        ProgramT::FourThree(prog) => assert_machine(prog, steps),
        ProgramT::FourFour(prog) => assert_machine(prog, steps),
        ProgramT::FiveTwo(prog) => assert_machine(prog, steps),
        ProgramT::FiveThree(prog) => assert_machine(prog, steps),
        ProgramT::FiveFour(prog) => assert_machine(prog, steps),
        ProgramT::SixTwo(prog) => assert_machine(prog, steps),
        ProgramT::SixThree(prog) => assert_machine(prog, steps),
        ProgramT::SixFour(prog) => assert_machine(prog, steps),
    }
}
