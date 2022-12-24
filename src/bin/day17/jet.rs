use adventofcode_2022::CustomError;

pub enum Jet {
    Left = -1,
    Right = 1,
}

impl TryFrom<char> for Jet {
    type Error = CustomError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(CustomError {
                msg: "Malformed jet detected.".into(),
            }),
        }
    }
}
