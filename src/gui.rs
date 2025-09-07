use std::sync::Arc;

use iced::Alignment::Center;
use iced::Length::Fill;
use iced::alignment::Horizontal::Left;
use iced::alignment::Vertical::Top;
use iced::widget::text_editor::Edit;
use iced::widget::{button, column, container, image, row, text};
use iced::widget::{
    text_editor,
    text_editor::{Action, Content},
};
use iced::{Element, Theme};
use rfd::FileDialog;

use crate::commands::{decode_command, encode_command};

pub fn start_gui(title: &'static str) {
    let _ = iced::application(title, GUIState::update, GUIState::view)
        .theme(theme)
        .run();
}

fn theme(_: &GUIState) -> Theme {
    // Theme::Light
    // Theme::Dark
    // Theme::Dracula
    // Theme::Nord
    // Theme::SolarizedLight
    // Theme::SolarizedDark
    // Theme::GruvboxLight
    // Theme::GruvboxDark
    // Theme::CatppuccinLatte
    // Theme::CatppuccinFrappe
    // Theme::CatppuccinMacchiato
    // Theme::CatppuccinMocha
    // Theme::TokyoNight
    // Theme::TokyoNightStorm
    // Theme::TokyoNightLight
    // Theme::KanagawaWave
    // Theme::KanagawaDragon
    // Theme::KanagawaLotus
    // Theme::Moonfly
    // Theme::Nightfly
    Theme::Oxocarbon
    // Theme::Ferra
}

#[derive(Default)]
pub struct GUIState {
    pub text_input: Content,
    pub selected_file: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectFile,
    EditText(Action),
    Encode,
    Decode,
}

impl GUIState {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectFile => {
                self.select_file();
            }
            Message::EditText(action) => {
                self.text_input.perform(action);
            }
            Message::Encode => {
                encode_command(
                    self.selected_file.clone(),
                    self.text_input.text().trim().to_string(),
                    None,
                );
            }
            Message::Decode => {
                let result = decode_command(self.selected_file.clone(), None);
                self.text_input.perform(Action::SelectAll);
                self.text_input.perform(Action::Edit(Edit::Backspace));
                self.text_input
                    .perform(Action::Edit(Edit::Paste(Arc::new(result))));
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                row![
                    button("Select File")
                        .on_press(Message::SelectFile)
                        .padding(15),
                    column![
                        text(&self.selected_file),
                        image(&self.selected_file).height(350),
                    ]
                    .align_x(Left)
                    .spacing(15),
                ]
                .align_y(Top)
                .spacing(15),
                row![
                    text_editor(&self.text_input)
                        .placeholder("To Encode")
                        .on_action(Message::EditText),
                    button("Encode").on_press(Message::Encode).padding(15),
                    button("Decode").on_press(Message::Decode).padding(15),
                ]
                .align_y(Top)
                .spacing(15),
            ]
            .align_x(Left)
            .padding(15)
            .spacing(15),
        )
        .width(Fill)
        .height(Fill)
        .align_x(Center)
        .align_y(Top)
        .into()
    }

    pub fn select_file(&mut self) {
        let file = FileDialog::new()
            .add_filter("image", &["png", "jpg", "jpeg"])
            .set_directory("~")
            .pick_file();
        if let Some(file) = file {
            self.selected_file = String::from(file.to_str().unwrap_or_default());
        }
    }
}
