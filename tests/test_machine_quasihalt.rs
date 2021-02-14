use lin_rado_turing::{
    machine::{HaltReason, Machine},
    program::{parse_program, Program, ProgramT},
    types::{State, Symbol},
};

const QUASIHALTING: &[(&str, usize, usize, usize, &str)] = &[
    // 2/2 (not better than BB)
    ("1RB 1LB 1LB 1LA", 3, 6, 1, "2-2"),
    ("1RB 1LB 0LB 1LA", 2, 6, 1, "2-2"),
    ("1RB 0LB 1LB 1LA", 2, 6, 1, "2-2"),
    ("1RB 0LB 0LB 1LA", 1, 6, 1, "2-2"),
    // 2/3
    ("1RB 2LB 1RA 2LB 2LA 0RA", 10, 43, 1, "2-3"), // BBB sigma
    ("1RB 2LB 1LA 2LB 2RA 0RA", 8, 59, 1, "2-3"),  // BBB shift
    ("1RB 0LB 1RA 1LB 2LA 2RA", 3, 45, 1, "2-3"),
    ("1RB 2RA 2LB 2LB 2LA 0LA", 5, 40, 1, "2-3"),
    ("1RB 1LA 2RA 2LA 2LB 2RB", 8, 17, 2, "2-3"),
    // 3/2
    ("1RB 0LB 1LA 0RC 1LC 1LA", 6, 55, 1, "3-2"), // BBB shift
    ("1RB 1RC 1LC 1RA 1RA 1LA", 6, 9, 2, "3-2"),  // BBB sigma
    ("1RB 0LB 1RC 0RC 1LC 1LA", 6, 54, 1, "3-2"),
    ("1RB 0LC 1LB 0RC 1LC 1LA", 5, 52, 1, "3-2"),
    ("1RB 0LC 0LC 0RC 1LC 1LA", 5, 51, 1, "3-2"),
    ("1RB 0LC 1LA 0RC 1RC 1RB", 5, 49, 1, "3-2"),
    ("1RB 0LC 0RC 0RC 1LC 1LA", 5, 48, 1, "3-2"),
    ("1RB 1RC 1LC 0LB 1RA 1LA", 5, 22, 2, "3-2"),
    // 4/2
    ("1RB 0LC 1LD 0LA 1RC 1RD 1LA 0LD", 0, 66349, 1, "4-2"), // BBB shift
    ("1RB 1RC 1LC 1RD 1RA 1LD 0RD 0LB", 69, 2819, 1, "4-2"), // BBB sigma
    ("1RB 1RA 0RC 0RB 0RD 1RA 1LD 1LB", 0, 2568, 1, "4-2"),
    ("1RB 1RA 0RC 1LA 1LC 1LD 0RB 0RD", 0, 2512, 1, "4-2"),
    ("1RB 1RC 1RD 0LC 1LD 0LD 1LB 0RA", 56, 2332, 3, "4-2"),
    ("1RB 0LC 1RC 1LD 1RD 0RB 0LB 1LA", 35, 1460, 3, "4-2"), // QH 1459
    ("1RB 0LD 1LC 0RD 0LC 1LA 1RA 0RB", 25, 1459, 1, "4-2"),
    ("1RB 1LC 1LC 0RD 1LA 0LB 1LD 0RA", 39, 1164, 1, "4-2"),
    ("1RB 1LB 1RC 0LD 0RD 0RA 1LD 0LA", 20, 1153, 1, "4-2"),
    // 5/2
    (
        "1RB 1LC 1LC 1RA 1LB 0LD 1LA 0RE 1RD 1RE",
        504,
        221032,
        2,
        "5-2",
    ),
    // 2/4
    ("1RB 2RB 1LB 1LA 1LB 3RA 3LA 2RB", 3340, 2333909, 1, "2-4"),
    ("1RB 2RB 3LA 2RA 1LB 1LA 1LB 3RB", 63, 22465, 1, "2-4"), // QH 22402  Commentted out are off by 1, mine one less
    ("1RB 2LA 1RA 1LA 2LB 3LA 2RB 2RA", 107, 10459, 3, "2-4"), // QH 10353
    ("1RB 2LA 1RA 1LA 3LA 1LB 2RB 2RA", 90, 7193, 2, "2-4"),  // QH 7106
    ("1RB 2LA 1RA 1LA 3LA 1LB 2RB 2LA", 84, 6443, 2, "2-4"),  // QH 6362
    ("1RB 2RB 1LA 1LA 2LB 2RA 3LB 1LA", 63, 4068, 1, "2-4"),  // QH 4005
];

#[test]
fn test_machine_quasihalts() {
    for (prog_str, marks, steps, period, complexity) in QUASIHALTING {
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

    assert_eq!(
        (machine.marks(), halt.steps, halt.reason),
        (marks, steps, HaltReason::Quasihalt(period))
    );
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
