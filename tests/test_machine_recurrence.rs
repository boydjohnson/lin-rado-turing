use lin_rado_turing::{
    machine::{HaltReason, Machine},
    program::{parse_program, Program, ProgramT},
    types::{State, Symbol},
};

const RECURRENCE: &'static [(&str, usize, usize, usize, &str)] = &[
    // Lin-Rado examples
    ("1RB 1RH 0RC 1LB 1LA 0RB", 2, 9, 10, "3-2"), // total recurrence
    ("1RB 1RH 1LB 0LC 1LA 1RA", 4, 12, 7, "3-2"), // left barrier
    ("1RB 1RH 1LC 1RA 1LA 0LC", 4, 12, 8, "3-2"), // right barrier
    // 2/2
    ("1RB 0LB 1LA 0RB", 3, 9, 3, "2-2"),
    ("1RB 1LA 0LA 1RA", 3, 7, 5, "2-2"),
    ("1RB 1LB 1LA 0RB", 2, 7, 3, "2-2"),
    // 3/2
    ("1RB 1LB 0RC 0LA 1LC 0LA", 9, 101, 24, "3-2"),
    ("1RB 1LA 1LC 1RC 1LA 0RB", 10, 69, 16, "3-2"),
    ("1RB 1LB 1RC 0LA 1LA 1RC", 10, 65, 16, "3-2"),
    ("1RB 0LC 1LC 1RB 1RA 1LA", 9, 50, 16, "3-2"),
    ("1RB 0LC 1LC 1RB 1RB 1LA", 9, 50, 12, "3-2"),
    ("1RB 0LB 1LC 0RC 1RA 1LA", 6, 38, 21, "3-2"),
    ("1RB 1LA 0RC 0RA 1LC 0LA", 4, 17, 36, "3-2"),
    ("1RB 1LB 0RC 0RB 1LC 0LA", 3, 4, 38, "3-2"),
    ("1RB 0LA 0RC 1LA 1LC 0RB", 0, 0, 92, "3-2"),
    ("1RB 0LA 0RC 0RC 1LC 1LA", 0, 0, 48, "3-2"),
    ("1RB 1LB 0RC 1LA 1LA 0RA", 0, 0, 21, "3-2"),
    // 2/3
    ("1RB 0LA 0RH 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 0LH 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 1RH 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 1LH 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 0RA 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 0RB 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 0LA 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 0LB 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 1RA 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 1RB 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 1LA 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 1LB 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 2RA 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 2RB 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 2LA 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 0LA 2LB 1LB 2LA 0RB", 15, 165, 54, "2-3"),
    ("1RB 1LB 2LA 1LA 2RB 0RA", 12, 101, 26, "2-3"),
    ("1RB 2RB 1LB 1LA 2RB 0LA", 13, 97, 14, "2-3"),
    ("1RB 2LA 0RB 1LA 1RB 1RA", 13, 94, 20, "2-3"),
    ("1RB 2LA 0RB 1LA 2LB 1RA", 11, 89, 26, "2-3"),
    ("1RB 1LA 1LB 1LA 2RB 0LA", 12, 80, 20, "2-3"),
    ("1RB 2LA 0RB 1LA 2LA 1RA", 12, 78, 14, "2-3"),
    ("1RB 2LA 0RB 1LB 2LA 1RA", 10, 76, 14, "2-3"),
    ("1RB 2LA 0RB 1LA 0LB 1RA", 2, 75, 4, "2-3"),
    ("1RB 2LB 2LA 2LA 0LB 0RA", 8, 63, 32, "2-3"),
    ("1RB 0RA 2LB 2LA 2RA 0LB", 6, 59, 32, "2-3"),
    ("1RB 1LB 1LB 1LA 2RB 0LA", 9, 58, 8, "2-3"),
    ("1RB 2LA 2LB 1LA 2RA 0LB", 8, 57, 60, "2-3"),
    ("1RB 1LA 2LB 2LA 2RA 0LB", 6, 57, 30, "2-3"),
    ("1RB 2LA 0RB 1LB 1RA 1RA", 6, 55, 10, "2-3"),
    ("1RB 0RB 0LB 2LA 2RA 1LB", 7, 54, 40, "2-3"),
    ("1RB 2LA 1RB 1LB 1LA 2RA", 7, 24, 46, "2-3"),
    ("1RB 1LA 2LB 1LA 2RA 0LB", 7, 20, 48, "2-3"),
    ("1RB 2RB 2LA 1LB 1RA 0LA", 4, 14, 54, "2-3"),
    ("1RB 0RB 1LA 2LA 2RA 0LB", 3, 10, 48, "2-3"),
    ("1RB 0RA 1LB 2LA 2RB 0LA", 3, 6, 48, "2-3"),
    ("1RB 2LA 0RB 0LB 1LA 0RA", 1, 2, 57, "2-3"),
    ("1RB 2LB 0RA 1LA 2RB 2RA", 0, 0, 60, "2-3"),
    ("1RB 2LA 1LB 0LA 0RB 1RA", 0, 0, 47, "2-3"),
    // 4/2
    ("1RB 1RC 1LC 0RB 1LD 0RA 1RA 0LB", 51, 1727, 622, "4-2"),
    ("1RB 0LC 1RD 0RD 1LA 0RC 1LB 1RC", 39, 1527, 522, "4-2"),
    ("1RB 0LC 1RC 1RD 1LD 0RC 1LA 0RB", 45, 1301, 622, "4-2"),
    ("1RB 1LC 1RD 0RB 0LC 1LA 1RC 0RA", 33, 1111, 131, "4-2"),
    ("1RB 1RC 1LB 1LC 1RD 0LB 1RA 0RD", 30, 1033, 174, "4-2"),
    ("1RB 0LC 1RD 0RB 1LC 1LA 1RC 1RA", 30, 1004, 174, "4-2"),
    ("1RB 1LA 1RC 0RD 0LA 0RC 1RC 1LC", 29, 979, 144, "4-2"),
    ("1RB 1RC 1LC 0LD 0RA 1LB 1RD 0LA", 24, 928, 128, "4-2"),
    ("1RB 0RA 0LB 0LC 1RD 1LC 1RA 1LB", 19, 868, 404, "4-2"),
    ("1RB 0RC 0LD 1RA 0LA 0RD 1LC 1LA", 12, 383, 200, "4-2"),
    ("1RB 0LA 1LC 1LD 1RD 1LB 1RA 0RD", 12, 79, 481, "4-2"),
    ("1RB 1LA 1RC 0RC 1LD 0RD 0LA 1LA", 7, 66, 284, "4-2"),
    ("1RB 1RC 0RC 1RA 1LD 0RB 0LD 1LA", 7, 50, 597, "4-2"),
    ("1RB 1RA 1LC 0RB 1RC 0LD 1LA 1LD", 8, 45, 228, "4-2"),
    ("1RB 1LA 1LC 0RA 1LD 0LC 1RA 0LA", 3, 5, 385, "4-2"),
    ("1RB 0RA 1LC 1RA 1LD 0LC 1LA 0RB", 3, 5, 244, "4-2"),
    ("1RB 1RC 0LD 1RA 1LB 0RD 1LA 0RC", 1, 2, 294, "4-2"),
    ("1RB 0LC 1LD 1LC 1RD 0LA 0RA 1LB", 0, 0, 294, "4-2"),
    ("1RB 1LA 1LB 0RC 1LC 1LD 0RA 0LD", 0, 0, 238, "4-2"),
    ("1RB 0LA 1LB 0RC 1RD 1RC 1LA 1LD", 0, 0, 228, "4-2"),
];

#[test]
fn test_machine_recurrence() {
    for (prog_str, marks, steps, period, complexity) in RECURRENCE {
        println!("{}", prog_str);
        parse_program_and_assert(prog_str, *marks, *steps, *period, complexity);
    }
}

fn assert_machine<S: State, Sym: Symbol>(
    prog: Program<S, Sym>,
    marks: usize,
    steps: usize,
    period: usize,
) {
    let mut machine = Machine::new(prog);

    let check = if steps < 256 { Some(0) } else { Some(steps) };

    machine.run_until_halt::<std::io::Stdout>(vec![], steps + 2 * period, &mut None, check);

    let halt = machine.halt();

    assert!(halt.is_some());

    let halt = halt.unwrap();

    assert_eq!(halt.reason, HaltReason::Recurr(period));

    assert_eq!(halt.steps, steps);

    assert_eq!(machine.marks(), marks);
}

fn parse_program_and_assert(
    prog_str: &str,
    marks: usize,
    steps: usize,
    period: usize,
    complexity: &str,
) {
    let program = parse_program(prog_str, complexity).unwrap();

    match program {
        ProgramT::TwoTwo(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::TwoThree(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::TwoFour(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::ThreeTwo(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::ThreeThree(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::ThreeFour(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::FourTwo(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::FourThree(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::FourFour(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::FiveTwo(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::FiveThree(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::FiveFour(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::SixTwo(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::SixThree(prog) => assert_machine(prog, marks, steps, period),
        ProgramT::SixFour(prog) => assert_machine(prog, marks, steps, period),
    }
}
