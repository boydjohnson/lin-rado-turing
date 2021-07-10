use clap::{App, Arg};
use lin_rado_turing::{
    program::{parse_program, Program, ProgramT},
    types::{State, Symbol},
};
use std::{collections::BTreeMap, io::Write};

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    App::new("graph")
        .about("give graph information about turing machine programs")
        .arg(
            Arg::with_name("complexity")
                .required(true)
                .help("The number of states and number of symbols. eg 3-2, 4-2, 2-4..."),
        )
        .arg(
            Arg::with_name("program")
                .required(true)
                .help("The Turing program. eg 1RB 0LA 1RB 0LH"),
        )
        .get_matches()
}

fn get_graph_information<S: State, Sym: Symbol>(prog_str: &str, program: Program<S, Sym>) {
    let mut graph = petgraph::Graph::new();

    let mut map = BTreeMap::default();

    for item in program
        .0
        .iter()
        .flat_map(|(first, second)| vec![*first, (second.0, second.1)])
    {
        if map.get(&item).is_none() {
            let node_id = graph.add_node(item);
            map.insert(item, node_id);
        }
    }

    for (start, finish) in program.0.iter() {
        let first = map.get(start).unwrap();

        let second = map.get(&(finish.0, finish.1)).unwrap();

        graph.add_edge(*first, *second, 1);
    }

    let is_cyclic = petgraph::algo::is_cyclic_directed(&graph);

    println!("{}: cyclic({})", prog_str, is_cyclic);

    let mut file = std::fs::File::create(format!("{}.dot", prog_str)).unwrap();

    let dot = format!("{:?}", petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));

    let index = dot.rfind('}').unwrap();

    let (dot, _) = dot.split_at(index);

    file.write_all(format!("{}\nlabelloc=\"t\";\nlabel=\"{}\";\n}}", dot, prog_str).as_bytes())
        .unwrap();
}

fn main() {
    let args = parse_args();

    let prog = args.value_of("program").unwrap();
    let complexity = args.value_of("complexity").unwrap();

    match parse_program(prog, complexity) {
        Ok(ProgramT::TwoTwo(p)) => get_graph_information(prog, p),
        Ok(ProgramT::TwoThree(p)) => get_graph_information(prog, p),
        Ok(ProgramT::TwoFour(p)) => get_graph_information(prog, p),
        Ok(ProgramT::ThreeTwo(p)) => get_graph_information(prog, p),
        Ok(ProgramT::ThreeThree(p)) => get_graph_information(prog, p),
        Ok(ProgramT::ThreeFour(p)) => get_graph_information(prog, p),
        Ok(ProgramT::FourTwo(p)) => get_graph_information(prog, p),
        Ok(ProgramT::FourThree(p)) => get_graph_information(prog, p),
        Ok(ProgramT::FourFour(p)) => get_graph_information(prog, p),
        Ok(ProgramT::FiveTwo(p)) => get_graph_information(prog, p),
        Ok(ProgramT::FiveThree(p)) => get_graph_information(prog, p),
        Ok(ProgramT::FiveFour(p)) => get_graph_information(prog, p),
        Ok(ProgramT::SixTwo(p)) => get_graph_information(prog, p),
        Ok(ProgramT::SixThree(p)) => get_graph_information(prog, p),
        Ok(ProgramT::SixFour(p)) => get_graph_information(prog, p),
        Err(e) => writeln!(std::io::stderr(), "{:?}", e).unwrap(),
    }
}
