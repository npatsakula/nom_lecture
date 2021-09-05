use crate::{horizontal_position_parser, HorizontalPosition, VerticalPosition};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    pub vertical: VerticalPosition,
    pub horizontal: HorizontalPosition,
}

pub fn position_parser(source: &str) -> IResult<&str, Position> {
    let (rest, _spaces) = nom::character::complete::space0(source)?;
    let (rest, horizontal) = horizontal_position_parser(rest)?;
    let (rest, vertical) = VerticalPosition::parse(rest)?;
    Ok((
        rest,
        Position {
            vertical,
            horizontal,
        },
    ))
}

#[cfg(test)]
mod entire_position {
    use super::{position_parser, HorizontalPosition::*, Position, VerticalPosition::*};
    use nom::IResult;
    use test_case::test_case;

    #[test_case("a5 : f8" => IResult::Ok((" : f8", Position { vertical: Five, horizontal: A })); "simple position parse test")]
    #[test_case("  a5" => IResult::Ok(("", Position { vertical: Five, horizontal: A })); "test with whitespace")]
    #[test_case("5a" => matches IResult::Err(_); "simple error-causing test")]
    fn basic(source: &str) -> IResult<&str, Position> {
        position_parser(source)
    }
}
