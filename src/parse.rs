use nom::{
    character::{complete::satisfy, is_space},
    combinator::recognize,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

type Instruction = (char, char, char);
type StateInstructions = Vec<Instruction>;
pub type Instructions = Vec<StateInstructions>;

fn instruction(s: &str) -> IResult<&str, Instruction> {
    tuple((
        satisfy(|c| matches!(c, '0'..='9') || c == '.'),
        satisfy(|c| matches!(c, 'L' | 'R') || c == '.'),
        satisfy(|c| matches!(c, 'A'..='Z') || c == '.'),
    ))(s)
}

fn single(s: &str) -> IResult<&str, &str> {
    recognize(satisfy(|c| is_space(c as u8)))(s)
}

fn double(s: &str) -> IResult<&str, &str> {
    recognize(tuple((single, single)))(s)
}

fn state_instructions(s: &str) -> IResult<&str, StateInstructions> {
    separated_list1(single, instruction)(s)
}

pub fn parse_instructions(s: &str) -> IResult<&str, Instructions> {
    separated_list1(double, state_instructions)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction() {
        assert_eq!(instruction("1RB"), Ok(("", ('1', 'R', 'B'))));
    }

    #[test]
    fn test_state_instruction() {
        assert_eq!(
            state_instructions("1RB 0LA  1RB 0LA"),
            Ok(("  1RB 0LA", vec![('1', 'R', 'B'), ('0', 'L', 'A')]))
        );
    }

    #[test]
    fn test_instructions() {
        assert_eq!(
            parse_instructions("1RB 0LA  1RB 0LA"),
            Ok((
                "",
                vec![
                    vec![('1', 'R', 'B'), ('0', 'L', 'A')],
                    vec![('1', 'R', 'B'), ('0', 'L', 'A')]
                ]
            ))
        );
    }
}
