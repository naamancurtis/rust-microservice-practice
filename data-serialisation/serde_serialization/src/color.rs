use rand::distributions::Uniform;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::cmp::{max, min};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const WHITE: Color = Color {
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
};

pub const BLACK: Color = Color {
    red: 0x00,
    green: 0x00,
    blue: 0x00,
};

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &WHITE => f.write_str("white"),
            &BLACK => f.write_str("black"),
            color => write!(f, "#{:02X}{:02x}{:02x}", color.red, color.green, color.blue),
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "white" => Ok(WHITE.to_owned()),
            "black" => Ok(BLACK.to_owned()),
            s if s.starts_with("#") && s.len() == 7 => {
                let red = u8::from_str_radix(&s[1..3], 16)?;
                let green = u8::from_str_radix(&s[3..5], 16)?;
                let blue = u8::from_str_radix(&s[5..7], 16)?;
                Ok(Color { red, green, blue })
            }
            other => Err(ColorError::InvalidValue {
                value: other.to_owned(),
            }),
        }
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a color value expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<Color>()
            .map_err(|err| de::Error::custom(err.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl<'a> Deserialize<'a> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

#[derive(Debug, Fail)]
pub enum ColorError {
    #[fail(display = "parse color's component error: {}", _0)]
    InvalidComponent(#[cause] ParseIntError),
    #[fail(display = "invalid value: {}", value)]
    InvalidValue { value: String },
}

impl From<ParseIntError> for ColorError {
    fn from(err: ParseIntError) -> Self {
        ColorError::InvalidComponent(err)
    }
}

pub fn color_range(from: u8, to: u8) -> Uniform<u8> {
    let (from, to) = (min(from, to), max(from, to));
    Uniform::new_inclusive(from, to)
}
