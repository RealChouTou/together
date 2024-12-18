use iced::widget::{button, horizontal_space, text, text_editor, text_input, Column, Container};
use iced::{Background, Center, Color, Element, Length, Task};
use rfd::{AsyncFileDialog, FileDialog};
use std::process::Command;

#[derive(Default)]
struct App {
    start_time: String,
    full_screen: bool,
    file_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    OpenFile,
    FileSelected(Option<String>),
    InputChanged(String),
}
impl App {
    pub fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let content = Column::new()
            .padding(20)
            .align_x(Center)
            .push(
                text_input("start time", &self.start_time)
                    .width(300)
                    .on_input(Message::InputChanged),
            )
            .push(horizontal_space().height(20))
            .push(text(self.start_time.clone()))
            .push(horizontal_space().height(20))
            .push(button("Select file").on_press(Message::OpenFile))
            .push(horizontal_space().height(20))
            .push(text(self.file_path.clone()))
            .push(horizontal_space().height(20))
            .push(button("Start").on_press(Message::Start));
        Container::new(content)
            .style(|theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::BLACK)), // 设置背景色
                    ..Default::default()
                }
            })
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => {
                println!("{} {}", "Start", self.file_path);
                let command = format!(
                    "{}{} {}",
                    "vlc --fullscreen --start-time=",
                    self.start_time,
                    self.file_path.replace("\\", "\\\\")
                );
                println!("{}", command);
                let output = Command::new("cmd")
                    .args(&["/C", command.as_str()]) // /C 参数用于运行单个命令并退出
                    .output()
                    .expect("Failed to execute command");

                // 将输出转换为字符串并打印
                println!("Status: {}", output.status);
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                Task::none()
            }
            Message::OpenFile => {
                // 使用 tinyfiledialogs 打开文件对话框
                println!("Opening file");
                Task::perform(
                    async {
                        AsyncFileDialog::new()
                            .pick_file()
                            .await
                            .map(|path| path.path().to_str().unwrap().to_string())
                    },
                    Message::FileSelected,
                )
            }
            Message::FileSelected(path) => {
                self.file_path = path.unwrap();
                println!("{}", self.file_path);
                Task::none()
            }
            Message::InputChanged(input) => {
                self.start_time = input;
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
fn main() -> iced::Result {
    iced::run("Together", App::update, App::view)
}
