use nom::{alt, named, tag, IResult};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HorizontalPosition {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

named!(pub horizontal_position_parser<&str, HorizontalPosition>, alt!(
        alt!(tag!("a") | tag!("A")) => { |_: &str| HorizontalPosition::A } |
        alt!(tag!("b") | tag!("B")) => { |_| HorizontalPosition::B } |
        alt!(tag!("c") | tag!("C")) => { |_| HorizontalPosition::C } |
        alt!(tag!("d") | tag!("D")) => { |_| HorizontalPosition::D } |
        alt!(tag!("e") | tag!("E")) => { |_| HorizontalPosition::E } |
        alt!(tag!("f") | tag!("F")) => { |_| HorizontalPosition::F } |
        alt!(tag!("g") | tag!("G")) => { |_| HorizontalPosition::G } |
        alt!(tag!("h") | tag!("H")) => { |_| HorizontalPosition::H }
));

impl HorizontalPosition {
    // Можно написать парсер, который содержит меньше визуального мусора,
    // но тогда усложнится интеграция его в общую структуру комбинаторов,
    // т.к. мы имеем несовместимый тип ошибки парсинга.
    pub fn try_from_char(source: char) -> Option<Self> {
        use HorizontalPosition::*;
        let result = match source {
            'a' | 'A' => A,
            'b' | 'B' => B,
            'c' | 'C' => C,
            'd' | 'D' => D,
            'e' | 'E' => E,
            'f' | 'F' => F,
            'g' | 'G' => G,
            'h' | 'H' => H,
            _ => return None,
        };

        Some(result)
    }

    pub fn dirty_pants_parser(source: &str) -> IResult<&str, Self> {
        use nom::{
            error::{make_error, ErrorKind},
            Err::Error as NomError,
        };

        let (rest, _spaces) = nom::character::complete::space0(source)?;
        let (rest, raw) = nom::character::complete::anychar(rest)?;

        match Self::try_from_char(raw) {
            Some(result) => Ok((rest, result)),
            // Здесь вылезает вся неаккуратность подхода с собственными парсерами,
            // а именно интеграция их в комбинаторную структуру.
            None => Err(NomError(make_error(source, ErrorKind::Verify))),
        }
    }

    // В `nom` 7.0+ было принято решение отказаться от макросов для создания
    // парсеров, т.к. они добавляют неявности в код, усложняют жизнь IDE и
    // затрудняют оценку производительности парсера.
    //
    // На мой взгляд, в подобном переборе вариантов тегов, вариант на голых
    // функциях тяжело анализировать из-за большого количества скобок и
    // вложенности.
    pub fn parse(source: &str) -> IResult<&str, Self> {
        use nom::{branch::alt, bytes::complete::tag, combinator::map};
        alt((
            map(alt((tag("a"), tag("A"))), |_: &str| Self::A),
            map(alt((tag("b"), tag("B"))), |_: &str| Self::B),
            map(alt((tag("c"), tag("C"))), |_: &str| Self::C),
            map(alt((tag("d"), tag("D"))), |_: &str| Self::D),
            map(alt((tag("e"), tag("E"))), |_: &str| Self::E),
            map(alt((tag("f"), tag("F"))), |_: &str| Self::F),
            map(alt((tag("g"), tag("G"))), |_: &str| Self::G),
            map(alt((tag("h"), tag("H"))), |_: &str| Self::H),
        ))(source)
    }
}

#[cfg(test)]
mod horizontal_position {
    use super::{horizontal_position_parser, HorizontalPosition};
    use nom::IResult;
    use test_case::test_case;

    #[test_case("a7" => IResult::Ok(("7", HorizontalPosition::A)); "simple lowercase test")]
    #[test_case("A7" => IResult::Ok(("7", HorizontalPosition::A)); "simple uppercase test")]
    #[test_case("c" => IResult::Ok(("", HorizontalPosition::C)); "simple single character test")]
    #[test_case("9" => matches IResult::Err(_); "simple error-causing test")]
    fn macro_parser(source: &str) -> IResult<&str, HorizontalPosition> {
        horizontal_position_parser(source)
    }

    #[test_case("a7" => IResult::Ok(("7", HorizontalPosition::A)); "simple lowercase test")]
    #[test_case("A7" => IResult::Ok(("7", HorizontalPosition::A)); "simple uppercase test")]
    #[test_case("c" => IResult::Ok(("", HorizontalPosition::C)); "simple single character test")]
    #[test_case("9" => matches IResult::Err(_); "simple error-causing test")]
    fn function_parser(source: &str) -> IResult<&str, HorizontalPosition> {
        HorizontalPosition::parse(source)
    }

    #[test_case("a7" => IResult::Ok(("7", HorizontalPosition::A)); "simple lowercase test")]
    #[test_case("A7" => IResult::Ok(("7", HorizontalPosition::A)); "simple uppercase test")]
    #[test_case("c" => IResult::Ok(("", HorizontalPosition::C)); "simple single character test")]
    #[test_case("9" => matches IResult::Err(_); "simple error-causing test")]
    fn dirty_parser(source: &str) -> IResult<&str, HorizontalPosition> {
        HorizontalPosition::dirty_pants_parser(source)
    }
}
