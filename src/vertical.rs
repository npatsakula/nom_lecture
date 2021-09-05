use nom::{alt, named, tag, IResult};

#[derive(Debug, PartialEq, Eq)]
pub enum VerticalPosition {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

named!(pub vertical_position_parser<&str, VerticalPosition>, alt!(
        tag!("1") => { |_: &str| VerticalPosition::One }   |
        tag!("2") => { |_| VerticalPosition::Two }   |
        tag!("3") => { |_| VerticalPosition::Three } |
        tag!("4") => { |_| VerticalPosition::Four }  |
        tag!("5") => { |_| VerticalPosition::Five }  |
        tag!("6") => { |_| VerticalPosition::Six }   |
        tag!("7") => { |_| VerticalPosition::Seven } |
        tag!("8") => { |_| VerticalPosition::Eight }
));

impl VerticalPosition {
    pub fn parse(source: &str) -> IResult<&str, Self> {
        use nom::{branch::alt, bytes::complete::tag, combinator::map};
        use VerticalPosition::*;

        alt((
            map(tag("1"), |_: &str| One),
            map(tag("2"), |_: &str| Two),
            map(tag("3"), |_: &str| Three),
            map(tag("4"), |_: &str| Four),
            map(tag("5"), |_: &str| Five),
            map(tag("6"), |_: &str| Six),
            map(tag("7"), |_: &str| Seven),
            map(tag("8"), |_: &str| Eight),
        ))(source)
    }
}

#[cfg(test)]
mod vertical_position {
    use super::{vertical_position_parser, VerticalPosition};
    use nom::IResult;
    use test_case::test_case;

    #[test_case("4;;" => IResult::Ok((";;", VerticalPosition::Four)); "simple one-digit test")]
    #[test_case("43;" => IResult::Ok(("3;", VerticalPosition::Four)); "simple two-digit test")]
    #[test_case("" => matches IResult::Err(_); "simple error-causing test")]
    fn macro_parser(source: &str) -> IResult<&str, VerticalPosition> {
        vertical_position_parser(source)
    }

    #[test_case("4;;" => IResult::Ok((";;", VerticalPosition::Four)); "simple one-digit test")]
    #[test_case("43;" => IResult::Ok(("3;", VerticalPosition::Four)); "simple two-digit test")]
    #[test_case("" => matches IResult::Err(_); "simple error-causing test")]
    fn function_parser(source: &str) -> IResult<&str, VerticalPosition> {
        VerticalPosition::parse(source)
    }
}
