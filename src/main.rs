use std::time::{Duration, Instant};

use iced::widget::{Row, button, row, text, text_input, column};
use iced::window::Settings;
use iced::{Alignment, Element, Subscription, Color, Theme};
use iced_timer::{Message, clear_button_style};
use iced_timer::timer::Timer;

#[derive(Default)]
struct TimerWidget {
    task_timer: Timer,
}

impl TimerWidget {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTimer(save) => {
                self.task_timer.toggle(save);
            },
            Message::ToggleEditing => {
                self.task_timer.toggle_editing(false);
            },
            Message::Tick => {
                if self.task_timer.started() || self.task_timer.ended() {
                    self.task_timer.tick();
                }
            },
            Message::Editing(index, value) => {
                self.task_timer.set_temp_vals(index, value);
            },
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let hms = self.task_timer.to_hmsms();
        let temp_vals = self.task_timer.get_temp_vals();
        column![
            row![if !self.task_timer.editing() {
                    button(text(hms.0).size(20).width(30).align_x(Alignment::End))
                        .style(|theme: &Theme, status| {
                            clear_button_style(
                                if self.task_timer.ended() {
                                    Color::from_rgb8(255, 0, 0)
                                } else {
                                    theme.palette().text
                                }
                            )
                    }).on_press(Message::ToggleEditing)
                } else {
                    button(text_input(&hms.0, &temp_vals.0).size(14).on_input(|val| Message::Editing(0, val)).width(30).align_x(Alignment::Center))
                        .style(|theme: &Theme, _status| {
                            clear_button_style(theme.palette().text)
                    })
                },
                text(" : ").size(20).center(), 
                if !self.task_timer.editing() {
                    button(text(hms.1).size(20).width(30).align_x(Alignment::End))
                        .style(|theme: &Theme, status| {
                            clear_button_style(
                                if self.task_timer.ended() {
                                    Color::from_rgb8(255, 0, 0)
                                } else {
                                    theme.palette().text
                                }
                            )
                    }).on_press(Message::ToggleEditing)
                } else {
                    button(text_input(&hms.1, &temp_vals.1).size(14).on_input(|val| Message::Editing(1, val)).width(30).align_x(Alignment::Center))
                        .style(|theme: &Theme, _status| {
                            clear_button_style(theme.palette().text)
                    })
                }, 
                text(" : ").size(20).center(), 
                if !self.task_timer.editing() {
                    button(text(hms.2).size(20).width(30).align_x(Alignment::End))
                        .style(|theme: &Theme, status| {
                            clear_button_style(
                                if self.task_timer.ended() {
                                    Color::from_rgb8(255, 0, 0)
                                } else {
                                    theme.palette().text
                                }
                            )
                    }).on_press(Message::ToggleEditing)
                } else {
                    button(text_input(&hms.2, &temp_vals.2).size(14).on_input(|val| Message::Editing(2, val)).width(30).align_x(Alignment::Center))
                        .style(|theme: &Theme, _status| {
                            clear_button_style(theme.palette().text)
                    })
                }
                ],
            row![
                button(if self.task_timer.editing() {
                        "Confirm"
                    } else { 
                        if !self.task_timer.started() && !self.task_timer.ended() {
                            "Start"
                        } else if self.task_timer.ended() {
                            "Stop"
                        } else {
                            "Pause"
                        }
                    }
                ).on_press(Message::ToggleTimer(true)),
                button(if self.task_timer.editing() {
                        "Cancel"
                    } else { 
                        "Reset"
                    }
                ).on_press(Message::ToggleTimer(false))
            ]
        ].into()
    }

    pub fn subscription(timer: &TimerWidget) -> Subscription<Message> {
        if timer.task_timer.ms_enabled() {
            iced::time::every(Duration::from_millis(10)).map(|_| Message::Tick )
        } else {
            iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick )
        }
    }
}


pub fn main() -> iced::Result {
    iced::application(|| TimerWidget {task_timer: Timer::new(Duration::from_mins(10))}, TimerWidget::update, TimerWidget::view)
        .window(Settings {level: iced::window::Level::AlwaysOnTop, ..Default::default()})
        .subscription(|f| TimerWidget::subscription(f))
        .run()
}