use lin_rado_turing::{
    machine::Machine,
    program::Program,
    types::{State, Symbol, ThreeState, TwoState, TwoSymbol},
};

const HALTS_2_2: &[(&str, usize, usize)] = &[
    // 2/2 BB
    ("1RB 1LB 1LA 1RH", 4, 6),
    // 3/2 BB
    ("1RB 1RH 1LB 0RC 1LC 1LA", 5, 21), // shift
    ("1RB 1LC 1RC 1RH 1LA 0LB", 6, 11), // sigma
    // 2/3 BB
    ("1RB 2LB 1RH 2LA 2RB 1LB", 9, 38),
    // 4/2 BB
    ("1RB 1LB 1LA 0LC 1RH 1LD 1RD 0RA", 13, 107), // shift
    ("1RB 0RC 1LA 1RA 1RH 1RD 1LD 0LB", 13, 96),  // sigma
    // 2/4 Runners-up
    ("1RB 3LA 1LA 1RA 2LA 1RH 3RA 3RB", 90, 7195),
    ("1RB 3LA 1LA 1RA 2LA 1RH 3LA 3RB", 84, 6445),
    ("1RB 3LA 1LA 1RA 2LA 1RH 2RA 3RB", 84, 6445),
    ("1RB 2RB 3LA 2RA 1LA 3RB 1RH 1LB", 60, 2351),
    // Milton Green (1964)
    ("1RB 1LA 0LH 1RB", 1, 2),
    ("1RB 1LH 0RC 1RC 0RD 0RC 1RE 1LA 0RF 0RE 1LF 1LD", 35, 436),
    // Lynn (1971)
    ("1RB 1RA 1LC 0LD 0RA 1LB 1RH 0LE 1RC 1RB", 15, 435),
    ("1RB 1RC 1LC 1LD 0RA 1LB 1RE 0LB 1RH 1RD", 22, 292),
    ("1RB 0RC 1LC 0LB 1RD 1LB 1RE 0RA 0RB 1RH", 22, 217),
    // Lynn reports 522 steps
    ("1RB 0LB 1LC 1RH 0LD 0LC 1LE 0RA 0LF 0LE 1RF 1RD", 42, 521),
    // Uwe (1981)

    // Castor diligentissimus et primus et perpetuus (Castor schultis)
    ("1RB 0LC 1RC 1RD 1LA 0RB 0RE 1RH 1LC 1RA", 501, 134467),
    // Castor ministerialis: the Civil Servant Beaver, who cares most
    // for his progress, but does not produce anything.
    ("1RB 1RA 1RC 0RE 1LD 0RA 1LB 1LD 0RH 0RB", 0, 52),
    // Castor scientificus: the Scientific Beaver, who does not produce
    // anything either, but with more effort and less effect on his
    // position.
    ("0RB 0LA 0RC 0RH 1RD 1LE 1LA 0LD 1RC 1RE", 0, 187),
    // Castor exflippus: the Beaver Freak, who tries to survive as long
    // as possible without producing anything, moving on the tape, and
    // changing his state.
    ("0RB 0LA 1RC 0RH 0LC 1RD 0LD 1RE 1LA 0LE", 0, 67),
];

const LIMIT: usize = 10000;

#[test]
fn test_machine_halts() {
    for (prog_str, marks, steps) in HALTS_2_2 {
        println!("{}", prog_str);

        match prog_str.parse::<Program<TwoState, TwoSymbol>>() {
            Ok(prog) => {
                assert_machine(prog, *marks, *steps);
                continue;
            }
            Err(_) => (),
        }

        match prog_str.parse::<Program<ThreeState, TwoSymbol>>() {
            Ok(prog) => {
                assert_machine(prog, *marks, *steps);
                continue;
            }
            Err(_) => panic!("Unable to parse program string"),
        }
    }
}

fn assert_machine<S: State, Sym: Symbol>(prog: Program<S, Sym>, marks: usize, steps: usize) {
    let mut machine = Machine::new(prog);

    machine.run_until_halt::<std::io::Stdout>(vec![], LIMIT, None);

    let halt = machine.halt();

    assert!(halt.is_some());

    let halt = halt.unwrap();

    assert_eq!(halt.steps, steps);
}
