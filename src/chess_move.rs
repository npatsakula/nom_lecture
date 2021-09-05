use super::{position_parser, HorizontalPosition, Position, VerticalPosition};
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct Move {
    pub departure: Position,
    pub destination: Position,
}

impl Move {
    pub fn full_move_parse<'a>(source: &'a str, delimiter: &str) -> IResult<&'a str, Self> {
        let (rest, departure) = position_parser(source)?;
        let (rest, _delimiter) = nom::bytes::complete::tag(delimiter)(rest)?;
        let (rest, destination) = position_parser(rest)?;
        Ok((
            rest,
            Self {
                departure,
                destination,
            },
        ))
    }

    pub fn short_notation_parse(source: &str, departure: VerticalPosition) -> IResult<&str, Self> {
        let (rest, _spaces) = nom::character::complete::space0(source)?;
        let (rest, departure_horizontal) = HorizontalPosition::parse(rest)?;
        let (rest, destination_horizontal) = HorizontalPosition::parse(rest)?;
        let (rest, destination_vertical) = VerticalPosition::parse(rest)?;

        Ok((
            rest,
            Self {
                departure: Position {
                    vertical: departure,
                    horizontal: departure_horizontal,
                },
                destination: Position {
                    vertical: destination_vertical,
                    horizontal: destination_horizontal,
                },
            },
        ))
    }
}

#[cfg(test)]
mod full_notation_move {
    use super::{Move, Position};
    use crate::{
        HorizontalPosition::*,
        VerticalPosition::{self, *},
    };
    use nom::IResult;
    use test_case::test_case;

    #[test_case(" e3-d4", "-" => IResult::Ok(("", Move {
        departure: Position { horizontal: E, vertical: Three },
        destination: Position { horizontal: D, vertical: Four },
    })); "simple test with front space")]
    #[test_case("e3 d4 ", " " => IResult::Ok((" ", Move {
        departure: Position { horizontal: E, vertical: Three },
        destination: Position { horizontal: D, vertical: Four },
    })); "simple test with space delimiter")]
    fn full_notation<'a>(source: &'a str, delimiter: &str) -> IResult<&'a str, Move> {
        Move::full_move_parse(source, delimiter)
    }

    #[test_case("ed4", Three => IResult::Ok(("", Move {
        departure: Position { horizontal: E, vertical: Three },
        destination: Position { horizontal: D, vertical: Four },
    })); "simple test")]
    fn short_notation(source: &str, departure: VerticalPosition) -> IResult<&str, Move> {
        Move::short_notation_parse(source, departure)
    }
}
