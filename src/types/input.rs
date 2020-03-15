use crate::{error::Error, types::button::Key};

#[derive(Debug)]
pub enum CommandKey {
    Push {
        key: Key,
        buffer_frame: Option<u32>,
    },
    Release {
        key: Key,
        buffer_frame: Option<u32>,
    },
    Hold {
        key: Key,
        buffer_frame: Option<u32>,
        hold_frame: Option<u32>,
    },
}

pub(crate) fn to_push_command_key(
    (key, buffer_frame): (Key, Option<&str>),
) -> Result<CommandKey, Error> {
    let buffer_frame = match buffer_frame {
        Some(f) => Some(f.parse().map_err(|err| Error::IntParseError { err })?),
        None => None,
    };

    Ok(CommandKey::Push { key, buffer_frame })
}

pub(crate) fn to_release_command_key(
    (key, buffer_frame): (Key, Option<&str>),
) -> Result<CommandKey, Error> {
    let buffer_frame = match buffer_frame {
        Some(f) => Some(f.parse().map_err(|err| Error::IntParseError { err })?),
        None => None,
    };

    Ok(CommandKey::Release { key, buffer_frame })
}

pub(crate) fn to_hold_command_key(
    (key, (hold_frame, buffer_frame)): (Key, (Option<&str>, Option<&str>)),
) -> Result<CommandKey, Error> {
    let buffer_frame = match buffer_frame {
        Some(f) => Some(f.parse().map_err(|err| Error::IntParseError { err })?),
        None => None,
    };
    let hold_frame = match hold_frame {
        Some(f) => Some(f.parse().map_err(|err| Error::IntParseError { err })?),
        None => None,
    };

    Ok(CommandKey::Hold {
        key,
        hold_frame,
        buffer_frame,
    })
}
