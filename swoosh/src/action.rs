use serde::{ Deserialize, Serialize };
use strum::Display;
use crate::components::value::ImageStatus;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Down,
    Up,
    ShiftUp,
    ShiftDown,
    Select,
    Delete,
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    UpdateImageStatus {
        index: usize,
        status: ImageStatus,
    },
    ClearImages,
    Help,
}
