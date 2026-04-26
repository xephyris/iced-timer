use std::time::{Duration, Instant};

use iced::theme::{Base, Mode, Style};
use iced::widget::space::horizontal;
use iced::widget::{Column, Container, Row, button, column, container, row, space, text, text_input};
use iced::window::Settings;
use iced::{Alignment, Background, Color, Element, Length, Point, Size, Subscription, Task, Theme, system};
use iced_timer::stopwatch::Stopwatch;
use iced_timer::{Message, clear_button_style};
use iced_timer::timer::Timer;

#[derive(Default)]
struct TimerWidget {
    task_timer: Timer,
    stopwatch: Stopwatch,
    current_break: Stopwatch,
    break_enabled: bool,
}

impl TimerWidget {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTimer(save, reset) => {
                self.task_timer.toggle(save, reset);
            },
            Message::ToggleEditing => {
                self.task_timer.toggle_editing(false);
            },
            Message::Tick => {
                if self.task_timer.started() || self.task_timer.ended() {
                    self.task_timer.tick();
                }
                if self.stopwatch.started() {
                    self.stopwatch.tick();
                    self.current_break.tick();
                }
            },
            Message::Editing(index, value) => {
                self.task_timer.set_temp_vals(index, value);
            },
            Message::ToggleBreak => {
                if self.break_enabled {
                    self.stopwatch.pause();
                    self.current_break.reset();
                    self.task_timer.start();
                    self.break_enabled = false;
                } else {
                    self.stopwatch.start();
                    self.current_break.start();
                    self.task_timer.stop();
                    self.break_enabled = true;
                }
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let hms = self.task_timer.to_hmsms();
        let temp_vals = self.task_timer.get_temp_vals();
        Container::new(column![
            Container::new(column![
                row![
                    horizontal(),
                    if !self.task_timer.editing() {
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
                    if !self.task_timer.ms_enabled() {
                        text(" : ").size(20).center()
                    } else {
                        text(".").size(24).center()
                    }, 
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
                    },
                    horizontal()
                ].width(Length::Fill),
                row![
                    horizontal(),
                    button(
                        text(if self.task_timer.editing() {
                            "Confirm"
                        } else { 
                            if !self.task_timer.started() && !self.task_timer.ended() {
                                "Start"
                            } else if self.task_timer.ended() {
                                "Stop"
                            } else {
                                "Pause"
                            }
                        }).center()
                    ).width(65.0).on_press(Message::ToggleTimer(true, false)),
                    space().width(10),
                    button(
                        text(if self.task_timer.editing() {
                            "Cancel"
                        } else { 
                            "Reset"
                        }).center()
                    ).width(65.0).on_press(Message::ToggleTimer(false, true)),
                    space().width(10),
                    button(
                        text(if self.break_enabled {
                            "End Break"
                        } else {
                            "Break"
                        }).center()
                    ).on_press(Message::ToggleBreak),
                    horizontal(),
                ].width(Length::Fill),
            ].align_x(Alignment::Center)).align_x(Alignment::Center).width(Length::Fill).center(Length::Fill),
            space().height(20),
            if self.break_enabled {
                row![
                    horizontal(),
                    text(format!(" Current / Total Break:\n {} / {}", self.current_break.to_string(), self.stopwatch.to_string_ms_removed())).align_x(Alignment::Center).center(),
                    horizontal()
                ].align_y(Alignment::Center).width(Length::Fill)    
            } else {
                row![
                    horizontal(),
                    text(format!(" Total Break:\n {}", self.stopwatch.to_string())).align_x(Alignment::Center).center().width(Length::Fill),
                    horizontal()
                ].align_y(Alignment::Center).width(Length::Fill)
            }
            
        ].width(Length::Fill).align_x(Alignment::Center)).center(Length::Fill).style(|_theme| {
            container::Style{
                background: Some(Background::Color(Color::from_linear_rgba(0.5, 0.5, 0.5, 0.0))),
                ..Default::default()
            }
        }).into()
    }

    pub fn subscription(timer: &TimerWidget) -> Subscription<Message> {
        if timer.task_timer.ms_enabled() || timer.stopwatch.ms_enabled() {
            iced::time::every(Duration::from_millis(10)).map(|_| Message::Tick )
        } else {
            iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick )
        }
    }

    fn style(status: &TimerWidget, theme: &Theme) -> Style {
        Style {
            background_color: theme.palette().background.scale_alpha(0.6), 
            text_color: theme.palette().text,
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(|| 
            TimerWidget {
                task_timer: Timer::new(Duration::from_mins(10)), 
                stopwatch: Stopwatch::new(), 
                current_break: Stopwatch::new(),
                break_enabled: false
            }, 
            TimerWidget::update, 
            TimerWidget::view)
        .window(Settings {
            level: iced::window::Level::AlwaysOnTop, 
            size: Size {width: 250.0, height: 150.0}, 
            position: iced::window::Position::SpecificWith(|window, resolution| {
                    Point{x: resolution.width - window.width, y: 0.0}
                }), 
            decorations: false,
            transparent: true,
            ..Default::default()
        })
        .style(TimerWidget::style)
        .subscription(|f| TimerWidget::subscription(f))
        .run()
}