use iced::{Color, Theme, widget::button};

pub mod timer;
pub mod stopwatch;

#[derive(Clone, Eq, PartialEq)]
pub enum Message {
    ToggleTimer(bool),
    Tick,
    Editing(u32, String),
    ToggleEditing,
    ToggleBreak,
}

pub fn clear_button_style(text: Color) -> button::Style {
    button::Style {
        background: None,
        text_color: text,
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
        snap: false,
    }
    
}