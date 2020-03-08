use crate::error::Error;

bitflags::bitflags! {
    pub struct Key : u16 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;

        const FORWARD = 1 << 4;
        const BACKWARD = 1 << 5;
        const UP = 1 << 6;
        const DOWN = 1 << 7;

        const FD = 1 << 8;
        const FU = 1 << 9;
        const BD = 1 << 10;
        const BU = 1 << 11;

        const NEUTRAL = 1 << 12;
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
            "1" => Ok(Key::BD),
            "2" => Ok(Key::DOWN),
            "3" => Ok(Key::FD),
            "4" => Ok(Key::BACKWARD),
            "6" => Ok(Key::FORWARD),
            "7" => Ok(Key::BU),
            "8" => Ok(Key::UP),
            "9" => Ok(Key::BU),
            _ => Err(Error::UnKnownKeyName),
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.contains(Key::FORWARD) {
            f.write_str("6")?;
        }
        if self.contains(Key::BACKWARD) {
            f.write_str("4")?;
        }
        if self.contains(Key::UP) {
            f.write_str("8")?;
        }
        if self.contains(Key::DOWN) {
            f.write_str("2")?;
        }

        if self.contains(Key::FU) {
            f.write_str("9")?;
        }
        if self.contains(Key::FD) {
            f.write_str("3")?;
        }
        if self.contains(Key::BU) {
            f.write_str("7")?;
        }
        if self.contains(Key::BD) {
            f.write_str("1")?;
        }

        if self.contains(Key::NEUTRAL) {
            f.write_str("・")?;
        }
        if self.contains(Key::A) {
            f.write_str("A")?;
        }
        if self.contains(Key::B) {
            f.write_str("B")?;
        }
        if self.contains(Key::C) {
            f.write_str("C")?;
        }
        if self.contains(Key::D) {
            f.write_str("D")?;
        }

        Ok(())
    }
}
