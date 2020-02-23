use crate::error::Error;

bitflags::bitflags! {
    pub struct Key : u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;

        const FORWARD = 1 << 4;
        const BACKWARD = 1 << 5;
        const UP = 1 << 6;
        const DOWN = 1 << 7;
    }
}

impl std::str::FromStr for Key {
    type Err = Error;

    fn from_str(button: &str) -> Result<Key, Self::Err> {
        match button {
            "A" => Ok(Key::A),
            "B" => Ok(Key::B),
            "C" => Ok(Key::C),
            "D" => Ok(Key::D),
            "1" => Ok(Key::BACKWARD | Key::DOWN),
            "2" => Ok(Key::DOWN),
            "3" => Ok(Key::FORWARD | Key::DOWN),
            "4" => Ok(Key::BACKWARD),
            "6" => Ok(Key::FORWARD),
            "7" => Ok(Key::BACKWARD | Key::UP),
            "8" => Ok(Key::UP),
            "9" => Ok(Key::BACKWARD | Key::UP),
            _ => Err(Error::UnKnownKeyName),
        }
    }
}
