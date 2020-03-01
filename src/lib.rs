mod error;
pub(crate) mod parse;
pub mod resource;

pub use parse::{button::Key, input::CommandKey, Command};

pub fn deserialize(data: &str) -> Result<Command, failure::Error> {
    Ok(parse::parse_command(data)?)
}

pub fn serialize(command: &Command) -> Result<String, failure::Error> {
    Ok(ron::ser::to_string(command)?)
}
