use iced::widget::{Row, button, row, text};
use iced::Element;
use iced_timer::Message;


#[derive(Default)]
struct Timer {
    counter: u32,
}

impl Timer {
    fn update(&mut self, message: Message) {
        match message {
            Message::CounterIncrement => self.counter += 1,
            Message::CounterDecrement => self.counter -= 1,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        row![
            button(text("-")).on_press(Message::CounterDecrement),
            text(&self.counter),
            button(text("+")).on_press(Message::CounterIncrement),
        ].into()
    }

}


pub fn main() -> iced::Result {
    iced::run(Timer::update, Timer::view)
}