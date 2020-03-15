mod command;
mod error;
pub mod resource;
pub(crate) mod types;

pub use crate::{
    command::Command,
    types::{button::Key, input::CommandKey},
};

pub fn deserialize(data: &str) -> Result<Command, failure::Error> {
    Ok(types::build_command(data)?)
}

pub fn serialize(command: &Command) -> Result<String, failure::Error> {
    Ok(ron::ser::to_string(command)?)
}
