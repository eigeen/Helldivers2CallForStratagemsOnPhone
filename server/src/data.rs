use rdev::{Button, Key};
use serde::{Deserialize, Serialize};
use std::fmt;

pub enum InputType {
    Click = 0,
    Press = 1,
    Release = 2,
    Begin = 3,
    End = 4,
}

impl InputType {
    pub fn from_u64(v: u64) -> Self {
        match v {
            1 => Self::Press,
            2 => Self::Release,
            3 => Self::Begin,
            4 => Self::End,
            _ => Self::Click,
        }
    }
}

pub enum KeyType {
    Keyboard = 0,
    MouseButton = 1,
    WheelUp = 2,
    WheelDown = 3,
}

pub struct InputData {
    pub key_type: KeyType,
    pub keyboard: Key,
    pub mouse_button: Button,
}

#[derive(Clone)]
pub enum Step {
    Open = 0,
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl Step {
    pub fn from_u64(v: u64) -> Self {
        match v {
            1 => Self::Up,
            2 => Self::Down,
            3 => Self::Left,
            4 => Self::Right,
            _ => Self::Open,
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Up => write!(f, "↑"),
            Self::Down => write!(f, "↓"),
            Self::Left => write!(f, "←"),
            Self::Right => write!(f, "→"),
        }
    }
}

pub enum Operation {
    Status = 0,
    Combined = 1,
    Independent = 2,
    Request = 3,
    Sync = 4,
    Auth = 5,
}

impl Operation {
    pub fn from_u64(v: u64) -> Self {
        match v {
            1 => Self::Combined,
            2 => Self::Independent,
            3 => Self::Request,
            4 => Self::Sync,
            5 => Self::Auth,
            _ => Self::Status,
        }
    }
}

// To comply with the JSON specification, ignore non_snake_case warnings.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u64,
    pub delay: u64,
    pub open: String,
    pub openType: String,
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub ip: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 23333,
            delay: 25,
            open: String::from("ctrl_left"),
            openType: String::from("hold"),
            up: String::from("w"),
            down: String::from("s"),
            left: String::from("a"),
            right: String::from("d"),
            ip: String::from(""),
        }
    }
}

pub enum Status {
    Success = 0,
    VersionMismatch = 1,
    Unauthorized = 2,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Success => write!(f, "0"),
            Self::VersionMismatch => write!(f, "1"),
            Self::Unauthorized => write!(f, "2"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Auth {
    pub sid: String,
    pub time: u64,
}
