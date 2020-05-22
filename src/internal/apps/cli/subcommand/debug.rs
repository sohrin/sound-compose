// TODO: GUIの実験が終わったら消す
use iced::{Sandbox, Element, Button, Column, Text, Settings, Container, Length, Align, HorizontalAlignment, Color, Background, button, text_input, TextInput};
use iced::window::Settings as WindowSettings;

pub fn debug_proc() {
    debug!("debug_proc() BEGIN.");

    Counter::run(Settings {
        window: WindowSettings {
            size: (300, 300), // (x, y)
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

// state
struct Counter {
    // counter value
    value: i32,

    // state of the two buttons
    increment_button: button::State,
    reset_button: button::State,
    input: text_input::State,
    input_value: String,
}

// message
#[derive(Debug, Clone)] // , Copy
pub enum Message {
    Increment,
    Reset,
    // TODO: 日本語入力が豆腐になる件はフォント読み込みで解決するか？ https://github.com/hecrj/iced/blob/master/examples/todos/src/main.rs
    TextInputChanged(String),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter {
            value: 0,
            increment_button: button::State::default(),
            reset_button: button::State::default(),
            input: text_input::State::default(),
            input_value: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("count up")
    }

    // update
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Reset => {
                self.value = 0;
            }
            Message::TextInputChanged(inputed) => {
                self.input_value = inputed;
            }
        }
    }

    // view logic
    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .push(
                    Button::new(&mut self.increment_button, Text::new("+1"))
                        .on_press(Message::Increment)
// TODO: 後で調べる
//                        .border_radius(5)
//                        .background(Background::Color(Color{r: 0.8, g: 0.8, b: 0.8, a: 1.})),
,
                )
                .push(
                    Text::new(self.value.to_string()).size(50).horizontal_alignment(HorizontalAlignment::Center),
                )
                .push(
                    Button::new(&mut self.reset_button, Text::new("reset"))
                        .on_press(Message::Reset),
                )
                .push(
                    TextInput::new(
                        &mut self.input,
                        "This is the placeholder...",
                        &mut self.input_value,
                        Message::TextInputChanged,
                    )
                )
                .align_items(Align::Center)
        )
            .width(Length::Fill)
            .center_x()
            .height(Length::Fill)
            .center_y()
            .into()
    }
}