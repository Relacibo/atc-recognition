use std::ops::Deref;

use thiserror::Error;

mod airlines;

fn main() {
    println!("Hello, world!");
}

pub struct DirectionDegrees(u32);

impl DirectionDegrees {
    pub fn new(val: u32) -> Result<Self, WhisperAtcError> {
        let res = match val {
            1..360 => Self(val),
            360 => Self(0),
            _ => return Err(WhisperAtcError::InvalidDirection(val)),
        };
        Ok(res)
    }

    pub fn direction(&self) -> &u32 {
        &self.0
    }
}

impl Deref for DirectionDegrees {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Feet(i32);

impl Feet {
    pub fn new(val: i32) -> Result<Self, WhisperAtcError> {
        let res = match val {
            -1355..100000 => Self(val),
            _ => return Err(WhisperAtcError::InvalidAltitute(val)),
        };
        Ok(res)
    }

    pub fn altitude(&self) -> &i32 {
        &self.0
    }
}

impl Deref for Feet {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FrequencyThousands(u32);

impl FrequencyThousands {
    pub fn new(val: u32) -> Result<Self, WhisperAtcError> {
        Ok(Self(val))
    }

    pub fn frequency(&self) -> &u32 {
        &self.0
    }
}
impl Deref for FrequencyThousands {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum AviationCommand {
    RadarContact,
    FlyHeading(DirectionDegrees),
    ChangeAltitude(Feet),
    ContactFrequency {
        frequency: FrequencyThousands,
        station: Option<String>,
    },
}

#[derive(Error, Debug)]
pub enum WhisperAtcError {
    #[error("Invalid direction!")]
    InvalidDirection(u32),
    #[error("Invalid altitute!")]
    InvalidAltitute(i32),
    #[error("Serde Json (de)serialization failed!")]
    SerdeDeserialize(#[from] serde_json::Error),
    #[error("Std Io Error!")]
    StdIo(#[from] std::io::Error),
}
