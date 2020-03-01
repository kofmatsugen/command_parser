use crate::{Command, CommandKey};
use serde::ser::{Serialize, Serializer};

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ser: String = self
            .keys()
            .map(|k| match k {
                CommandKey::Push { key, buffer_frame } => {
                    let mut ser = String::new();
                    ser.push_str("p");
                    ser.push_str(&format!("{}", key));
                    if let Some(buffer_frame) = buffer_frame {
                        ser.push_str(&format!("[{}]", buffer_frame));
                    }
                    ser
                }
                CommandKey::Release { key, buffer_frame } => {
                    let mut ser = String::new();
                    ser.push_str("r");
                    ser.push_str(&format!("{}", key));
                    if let Some(buffer_frame) = buffer_frame {
                        ser.push_str(&format!("[{}]", buffer_frame));
                    }
                    ser
                }
                CommandKey::Hold {
                    key,
                    hold_frame,
                    buffer_frame,
                } => {
                    let mut ser = String::new();
                    ser.push_str("h");
                    ser.push_str(&format!("{}", key));
                    if let Some(hold_frame) = hold_frame {
                        ser.push_str(&format!("({})", hold_frame));
                    }
                    if let Some(buffer_frame) = buffer_frame {
                        ser.push_str(&format!("[{}]", buffer_frame));
                    }
                    ser
                }
            })
            .collect::<Vec<_>>()
            .join(">");

        serializer.serialize_str(&ser)
    }
}
