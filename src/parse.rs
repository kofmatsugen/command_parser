pub mod button;
pub mod input;

use crate::error;
use button::Key;
use input::CommandKey;
use input::{to_hold_command_key, to_push_command_key, to_release_command_key};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};

#[derive(Debug)]
pub struct Command {
    keys: Vec<CommandKey>,
}

impl Command {
    pub fn new(command: &str) -> Result<Self, failure::Error> {
        Ok(parse_command(command)?)
    }

    pub fn keys(&self) -> impl Iterator<Item = &CommandKey> {
        self.keys.iter()
    }
}

pub fn parse_command(input: &str) -> Result<Command, error::Error> {
    let (rest, (_, command, _)) = tuple((
        multispace0,
        separated_list(sequence, alt((hold_key, push_key, release_key))),
        multispace0,
    ))(input)
    .map_err(|_| error::Error::NomParseError {
        string: input.into(),
    })?;

    if rest.is_empty() == false {
        return Err(error::Error::NotCompleteParse { rest: rest.into() });
    }

    Ok(Command { keys: command })
}

fn button(input: &str) -> IResult<&str, Key> {
    map_res(
        alt((tag("A"), tag("B"), tag("C"), tag("D"))),
        std::str::FromStr::from_str,
    )(input)
}

fn stick(input: &str) -> IResult<&str, Key> {
    map_res(
        alt((
            tag("1"),
            tag("2"),
            tag("3"),
            tag("4"),
            tag("6"),
            tag("7"),
            tag("8"),
            tag("9"),
        )),
        std::str::FromStr::from_str,
    )(input)
}

fn buffer_start(input: &str) -> IResult<&str, &str> {
    tag("[")(input)
}

fn buffer_end(input: &str) -> IResult<&str, &str> {
    tag("]")(input)
}

fn hold_start(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}

fn hold_end(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}

fn sequence(input: &str) -> IResult<&str, ()> {
    let (rest, (_, _, _)) = tuple((multispace0, tag(">"), multispace0))(input)?;

    Ok((rest, ()))
}

fn numbers(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn buttons(input: &str) -> IResult<&str, Key> {
    fold_many1(alt((stick, button)), Key::empty(), |acc, b| acc | b)(input)
}

fn buffer_frame(input: &str) -> IResult<&str, &str> {
    let (rest, (_, _, _, frame, _, _, _)) = tuple((
        multispace0,
        buffer_start,
        multispace0,
        numbers,
        multispace0,
        buffer_end,
        multispace0,
    ))(input)?;

    Ok((rest, frame))
}

fn hold_frame(input: &str) -> IResult<&str, &str> {
    let (rest, (_, _, _, frame, _, _, _)) = tuple((
        multispace0,
        hold_start,
        multispace0,
        numbers,
        multispace0,
        hold_end,
        multispace0,
    ))(input)?;

    Ok((rest, frame))
}

fn push_key(input: &str) -> IResult<&str, CommandKey> {
    let (rest, _) = tuple((tag("p"), multispace0))(input)?;
    let (rest, command) = map_res(tuple((buttons, opt(buffer_frame))), to_push_command_key)(rest)?;

    Ok((rest, command))
}

fn release_key(input: &str) -> IResult<&str, CommandKey> {
    let (rest, _) = tuple((tag("r"), multispace0))(input)?;
    let (rest, command) =
        map_res(tuple((buttons, opt(buffer_frame))), to_release_command_key)(rest)?;

    Ok((rest, command))
}

fn hold_key(input: &str) -> IResult<&str, CommandKey> {
    let (rest, _) = tuple((tag("h"), multispace0))(input)?;
    let (rest, command) = map_res(
        tuple((buttons, permutation((opt(hold_frame), opt(buffer_frame))))),
        to_hold_command_key,
    )(rest)?;

    Ok((rest, command))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_parse() {
        push_key("pABC1234[100]").unwrap();
    }
    #[test]
    fn hold_parse() {
        hold_key("hABC1234(10)").unwrap();
    }
    #[test]
    fn release_parse() {
        release_key("rABC1234[100]").unwrap();
    }

    #[test]
    fn command_parse() {
        // > の前後は半角スペース，タブ可
        let commands = parse_command("h4 >       r6 > pC").unwrap();
        assert_eq!(commands.keys.len(), 3);
    }

    #[test]
    fn command_parse_space() {
        // キー入力ボタン部分以外は半角スペース，タブ，改行を許容
        parse_command(
            r#"h 4 (60)[ 8 ]
            > r 6 [10 ]
        > p C6 [ 20]"#,
        )
        .unwrap();
    }

    #[test]
    fn command_parse_fail() {
        // キー入力ボタン部分に隙間ができるとだめ
        parse_command(r#"h4(60)[8]>r6[10]>pC 6[20]"#).unwrap_err();
    }
}
