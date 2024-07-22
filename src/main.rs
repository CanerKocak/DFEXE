use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use psutil::process::{Process, processes};
use std::collections::HashMap;

pub fn main() -> iced::Result {
    DFEXE::run(Settings::default())
}

struct DFEXE {
    processes: HashMap<u32, String>,
    search_query: String,
}

#[derive(Debug, Clone)]
enum Message {
    Refresh,
    SearchQueryChanged(String),
    KillProcess(u32),
}

impl Sandbox for DFEXE {
    type Message = Message;

    fn new() -> Self {
        let mut dfexe = DFEXE {
            processes: HashMap::new(),
            search_query: String::new(),
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
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                self.refresh_processes();
            },
            Message::KillProcess(pid) => {
                if let Ok(process) = Process::new(pid) {
                    if let Err(e) = process.kill() {
                        eprintln!("Failed to kill process {}: {}", pid, e);
                    } else {
                        self.processes.remove(&pid);
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let search_input = text_input("Search processes...", &self.search_query)
            .on_input(Message::SearchQueryChanged);

        let refresh_button = button("Refresh").on_press(Message::Refresh);

        let process_list = self.processes.iter().fold(
            column![].spacing(10),
            |column, (&pid, name)| {
                column.push(
                    row![
                        text(format!("{}: {}", pid, name)),
                        button("Kill").on_press(Message::KillProcess(pid))
                    ]
                        .spacing(20)
                        .align_items(Alignment::Center)
                )
            }
        );

        let content = column![
            text("DFEXE: Process Finder").size(28),
            search_input,
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
        let search_terms: Vec<String> = if self.search_query.is_empty() {
            vec!["ic".to_string(), "replica".to_string()]
        } else {
            vec![self.search_query.clone()]
        };

        self.processes = processes()
            .unwrap_or_default()
            .iter()
            .filter_map(|p| {
                p.as_ref().ok().and_then(|process| {
                    let name = process.name().unwrap_or_default().to_lowercase();
                    if search_terms.iter().any(|term| name.contains(term)) {
                        Some((process.pid(), name))
                    } else {
                        None
                    }
                })
            })
            .collect();
    }
}