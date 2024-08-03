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
        if v == 1 {
            Self::Press
        } else if v == 2 {
            Self::Release
        } else if v == 3 {
            Self::Begin
        } else if v == 4 {
            Self::End
        } else {
            Self::Click
        }
    }
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
        if v == 1 {
            Self::Up
        } else if v == 2 {
            Self::Down
        } else if v == 3 {
            Self::Left
        } else if v == 4 {
            Self::Right
        } else {
            Self::Open
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
}

impl Operation {
    pub fn from_u64(v: u64) -> Self {
        if v == 1 {
            Self::Combined
        } else if v == 2 {
            Self::Independent
        } else if v == 3 {
            Self::Request
        } else if v == 4 {
            Self::Sync
        } else {
            Self::Status
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u64,
    pub delay: u64,
    pub open: String,
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 23333,
            delay: 25,
            open: String::from("ctrl_left"),
            up: String::from("w"),
            down: String::from("s"),
            left: String::from("a"),
            right: String::from("d"),
        }
    }
}
