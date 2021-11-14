use lin_rado_turing::{
    machine::{run_machine, HaltReason, Machine},
    program::{parse_program, ProgramT},
    types::{State, Symbol},
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymodule]
fn tm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_bb, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn run_bb(
    prog: &str,
    _tape: Vec<i64>,
    x_limit: usize,
    check_rec: Option<usize>,
    check_blanks: bool,
    _samples: Vec<i64>,
) -> PyResult<PyMachine> {
    let blank = if check_blanks { Some(0) } else { None };

    let program = match parse_program(prog) {
        Ok(p) => p,
        Err(e) => return Err(PyValueError::new_err(e.0)),
    };
    match program {
        ProgramT::TwoTwo(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::TwoThree(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::TwoFour(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::ThreeTwo(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::ThreeThree(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::ThreeFour(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::FourTwo(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::FourThree(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::FourFour(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::FiveTwo(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::FiveThree(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::FiveFour(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::SixTwo(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::SixThree(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
        ProgramT::SixFour(program) => Ok((
            prog,
            run_machine(program, prog, x_limit, None, false, check_rec, blank, false),
        )
            .into()),
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyMachine {
    #[pyo3(get)]
    r#final: MachineResult,
}

impl PyMachine {
    fn new(res: MachineResult) -> Self {
        PyMachine { r#final: res }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct MachineResult {
    #[pyo3(get)]
    prog: String,
    #[pyo3(get)]
    undfnd: Option<(usize, String)>,
    #[pyo3(get)]
    linrec: Option<usize>,
    #[pyo3(get)]
    qsihlt: Option<usize>,
    #[pyo3(get)]
    blanks: Option<usize>,
    #[pyo3(get)]
    xlimit: Option<usize>,
}

impl<S: State + Send + Sync + ToString, Sym: Symbol + Send + Sync + ToString>
    From<(&str, Machine<S, Sym>)> for PyMachine
{
    fn from((prog_str, other): (&str, Machine<S, Sym>)) -> Self {
        let halt = other.halt().expect("Machine has been run until halt");
        let result = match &halt.reason {
            HaltReason::Quasihalt(period) => MachineResult {
                prog: prog_str.into(),
                undfnd: None,
                linrec: None,
                qsihlt: Some(*period),
                blanks: None,
                xlimit: None,
            },
            HaltReason::Recurr(period) => MachineResult {
                prog: prog_str.into(),
                undfnd: None,
                linrec: Some(*period),
                qsihlt: None,
                blanks: None,
                xlimit: None,
            },
            HaltReason::Blanking => MachineResult {
                prog: prog_str.into(),
                undfnd: None,
                linrec: None,
                qsihlt: None,
                blanks: Some(halt.steps),
                xlimit: None,
            },
            HaltReason::XLimit => MachineResult {
                prog: prog_str.into(),
                undfnd: None,
                linrec: None,
                qsihlt: None,
                blanks: None,
                xlimit: Some(halt.steps),
            },
            HaltReason::Undefined(state_color) => MachineResult {
                prog: prog_str.into(),
                undfnd: Some((halt.steps, state_color.to_owned())),
                linrec: None,
                qsihlt: None,
                blanks: None,
                xlimit: None,
            },
            HaltReason::Halt => MachineResult {
                prog: prog_str.into(),
                undfnd: None,
                linrec: None,
                qsihlt: None,
                blanks: None,
                xlimit: None,
            },
        };

        PyMachine::new(result)
    }
}
