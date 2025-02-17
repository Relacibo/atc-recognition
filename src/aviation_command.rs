use crate::errors::WhisperAtcError;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    South,
    SouthWest,
    West,
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
}

#[derive(Debug, Clone, Copy)]
pub enum LeftOrRight {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Heading {
    RunwayHeading,
    Direction(Direction),
    DirectionDegrees(DirectionDegrees),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TurnDegrees(u32);

impl TurnDegrees {
    pub fn new(val: u32) -> Result<Self, WhisperAtcError> {
        let res = match val {
            0..180 => Self(val),
            _ => return Err(WhisperAtcError::InvalidTurn(val)),
        };
        Ok(res)
    }

    pub fn direction(&self) -> &u32 {
        &self.0
    }
}

impl Deref for TurnDegrees {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy)]
pub enum ClimbOrDescend {
    Climb,
    Descend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
pub enum AviationCommandPart {
    RadarContact,
    Turn(LeftOrRight),
    TurnDegrees(TurnDegrees),
    FlyHeading(Heading),
    ProceedDirect(String),
    ClimbOrDescend(ClimbOrDescend),
    ChangeAltitude(Feet),
    ContactFrequency {
        frequency: FrequencyThousands,
        station: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum CommunicationEntity {
    All,
    GroundStation(String),
    Aircraft {
        full_name: String,
        airline: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct AviationCommandGroup {
    pub target: Option<CommunicationEntity>,
    pub parts: Vec<AviationCommandPart>,
}
