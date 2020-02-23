mod error;
mod parse;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};
use parse::{
    button::Key,
    input::{to_hold_command_key, to_push_command_key, to_release_command_key, CommandKey},
};

pub fn parse_command(input: &str) -> IResult<&str, Vec<CommandKey>> {
    let (rest, (_, command, _)) = tuple((
        multispace0,
        separated_list(sequence, alt((hold_key, push_key, release_key))),
        multispace0,
    ))(input)?;

    Ok((rest, command))
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
    let (rest, (_, frame, _)) = tuple((buffer_start, numbers, buffer_end))(input)?;

    Ok((rest, frame))
}

fn hold_frame(input: &str) -> IResult<&str, &str> {
    let (rest, (_, frame, _)) = tuple((hold_start, numbers, hold_end))(input)?;

    Ok((rest, frame))
}

fn push_key(input: &str) -> IResult<&str, CommandKey> {
    let (rest, _) = tag("p")(input)?;
    let (rest, command) = map_res(tuple((buttons, opt(buffer_frame))), to_push_command_key)(rest)?;

    Ok((rest, command))
}

fn release_key(input: &str) -> IResult<&str, CommandKey> {
    let (rest, _) = tag("r")(input)?;
    let (rest, command) =
        map_res(tuple((buttons, opt(buffer_frame))), to_release_command_key)(rest)?;

    Ok((rest, command))
}

fn hold_key(input: &str) -> IResult<&str, CommandKey> {
    let (rest, _) = tag("h")(input)?;
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
        let (_, commands) = parse_command("h4>r6>pC").unwrap();
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn command_parse_fail() {
        let (_, commands) = parse_command(
            r#"

        h4(60)[8] > r6[10] > pC[20] 　

        "#,
        )
        .unwrap();
        assert_eq!(commands.len(), 3);
    }
}
