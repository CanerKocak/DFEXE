use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Sandbox, Settings};
use psutil::process::processes;

pub fn main() -> iced::Result {
    DFEXE::run(Settings::default())
}

struct DFEXE {
    dfx_processes: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Refresh,
}

impl Sandbox for DFEXE {
    type Message = Message;

    fn new() -> Self {
        let mut dfexe = DFEXE {
            dfx_processes: Vec::new(),
        };
        dfexe.refresh_processes();
        dfexe
    }

    fn title(&self) -> String {
        String::from("DFEXE")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Refresh => self.refresh_processes(),
        }
    }

    fn view(&self) -> Element<Message> {
        let refresh_button = button("Refresh").on_press(Message::Refresh);

        let process_list = self
            .dfx_processes
            .iter()
            .fold(column![].spacing(10), |column, process| {
                column.push(text(process))
            });

        let content = column![
            text("DFEXE: DFX Process Finder").size(28),
            refresh_button,
            process_list
        ]
            .spacing(20)
            .align_items(Alignment::Center);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl DFEXE {
    fn refresh_processes(&mut self) {
        self.dfx_processes = processes()
            .unwrap_or_default()
            .iter()
            .filter_map(|p| {
                p.as_ref().ok().and_then(|process| {
                    let name = process.name().unwrap_or_default();
                    if name.contains("dfx") {
                        Some(format!("{}: {}", process.pid(), name))
                    } else {
                        None
                    }
                })
            })
            .collect();
    }
}