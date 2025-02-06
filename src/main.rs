use std::fs::File;
use std::io::Read;
use std::process::Command;
use iced::widget::{button, text_input, Column, Container, Image};
use iced::{application, Element, Fill, Font, Task, Theme};
use iced::widget::image::Handle;
use couplet_gen::{gen_couplet, gen_couplet_by_gpt, Couplet};

fn main() -> iced::Result {
    /*let couplet = gen_couplet_by_gpt("发财");
    gen_couplet(&Couplet::new(
        couplet.title,
        couplet.top,
        couplet.bottom,
    ));*/
    application(CoupletApp::title,CoupletApp::update,CoupletApp::view)
        .window_size((250f32,400f32))
        .default_font(Font::with_name("微软雅黑"))
        .run_with(CoupletApp::new)
}

#[derive(Debug)]
struct CoupletApp {
    name: String,
    version: String,
    input: String,
    generated_image: Option<String>,
}

#[derive(Debug,Clone)]
enum Message {
    GenerateImage,
    InputChanged(String),
}

impl CoupletApp {
    
    fn new() -> (Self,Task<Message>) {
        (Self {
            name: "CoupletApp".to_string(),
            version: "0.1.0".to_string(),
            input: "".to_string(),
            generated_image: None,
        }, Task::none())
    }
    fn title(&self) -> String {
        format!("{} - {}", self.name, self.version)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GenerateImage => {
                let description = self.input.clone();
                // Here you would make an actual API request to generate the image
                let couplet = gen_couplet_by_gpt(&self.input);
                let image_data = gen_couplet(&Couplet::new(
                    couplet.title,
                    couplet.top,
                    couplet.bottom,
                ));
                self.generated_image = Some(image_data);
            }
            Message::InputChanged(new_value) => {
                self.input = new_value;
            }
        }
    }
    
    fn view(&self) -> Element<Message> {
        let input =  text_input("输入春联主题...", &self.input)
            .on_input(Message::InputChanged)
            .padding(2)
            .size(20);

        let button = button("生成").on_press(Message::GenerateImage)
            .padding(2);

        let mut content = Column::new()
            .spacing(20)
            .push(input)
            .push(button);

        if let Some(image_data) = &self.generated_image {
            let image = Image::new(image_data);
            content = content.push(image);
        }

        Container::new(content).padding(20)
            .center_x(Fill)
            .into()
    }
}