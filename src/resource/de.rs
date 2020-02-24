use crate::{parse_command, Command};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

struct CommandVisitor;

impl<'de> Visitor<'de> for CommandVisitor {
    type Value = Command;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("require command text format")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_command(s).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Command, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CommandVisitor)
    }
}
