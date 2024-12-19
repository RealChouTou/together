#![windows_subsystem = "windows"]
use chrono::prelude::*;
use env_logger::Env;
use iced::border::width;
use iced::widget::{
    button, horizontal_space, text, text_editor, text_input, Column, Container, Row,
};
use iced::window::{icon, Icon, Position, Settings};
use iced::{
    exit, time, Background, Center, Color, Element, Left, Length, Padding, Size, Subscription, Task,
};
use log::*;
use rfd::{AsyncFileDialog, FileDialog};
use std::process::Command;

#[derive(Default)]
struct App {
    watch_time: String,
    start_time: String,
    full_screen: bool,
    file_path: String,
    schedule_running: bool,
    info: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    OpenFile,
    FileSelected(Option<String>),
    InputChanged(String),
    WatchTimeChanged(String),
    Tick,
}
impl App {
    pub fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let content = Column::new()
            .padding(Padding {
                top: 100.0,
                right: 0.0,
                bottom: 0.0,
                left: 260.0,
            })
            .push(
                Row::new().align_y(Center).push(text("Watch time: ")).push(
                    text_input("13:20", &self.watch_time)
                        .width(100)
                        .on_input(Message::WatchTimeChanged),
                ),
            )
            .push(horizontal_space().height(20))
            .push(text(&self.watch_time))
            .push(horizontal_space().height(20))
            .push(
                Row::new()
                    .align_y(Center)
                    .push(text("Video begin second: "))
                    .push(
                        text_input("20", &self.start_time)
                            .width(100)
                            .on_input(Message::InputChanged),
                    ),
            )
            .push(horizontal_space().height(20))
            .push(text(&self.start_time))
            .push(horizontal_space().height(20))
            .push(button("Select file").on_press(Message::OpenFile))
            .push(horizontal_space().height(20))
            .push(text(&self.file_path))
            .push(horizontal_space().height(20))
            .push(button("Start").on_press(Message::Start))
            .push(horizontal_space().height(20))
            .push(text(&self.info));
        Container::new(content)
            .style(|theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::BLACK)), // 设置背景色
                    ..Default::default()
                }
            })
            .width(Length::Fill)
            .height(Length::Fill)
            // .align_x(Center)
            // .align_y(Center)
            .into()
    }
    fn subscription(&self) -> Subscription<Message> {
        // 每秒触发一次 TimerTick 消息 (如果需要定时重复输出，可以保留此行)
        if self.schedule_running {
            time::every(time::Duration::from_millis(20)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => {
                self.schedule_running = true;
                self.info = format!("{} {}", "Please wait to", self.watch_time);
                info!("please wait");
                Task::none()
            }
            Message::OpenFile => {
                // 使用 tinyfiledialogs 打开文件对话框
                debug!("Opening file");
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
                debug!("{}", self.file_path);
                Task::none()
            }
            Message::InputChanged(input) => {
                self.info = "".to_string();
                if let Ok(_) = input.parse::<u32>() {
                    self.start_time = input;
                } else {
                    self.info = "Invalid video begin second,example: 10".to_string()
                }
                Task::none()
            }
            Message::WatchTimeChanged(input) => {
                self.info = "".to_string();
                let vec = input.split(":").collect::<Vec<&str>>();
                let size = vec.len();
                if size != 2 {
                    self.info = "Invalid Watch time, example: 13:10".to_string()
                }
                self.watch_time = input;
                Task::none()
            }
            Message::Tick => {
                let now = Local::now();
                let hour = now.hour();
                let minute = now.minute();
                // let second = now.second();
                // let nanosecond = now.nanosecond(); // 纳秒
                // let millisecond = nanosecond / 1_000_000; // 毫秒
                let split: Vec<&str> = self.watch_time.split(":").collect::<Vec<_>>();
                let plan_hour = split[0].parse::<u32>().unwrap();
                let plan_minute = split[1].parse::<u32>().unwrap();
                let go = plan_hour == hour && plan_minute == minute;
                if go && self.schedule_running {
                    self.schedule_running = false;
                    let command = format!(
                        "{}{} {}",
                        "vlc --fullscreen --start-time=",
                        self.start_time,
                        self.file_path.replace("\\", "\\\\")
                    );
                    debug!("{}", command);
                    let output = Command::new("cmd")
                        .args(&["/C", command.as_str()]) // /C 参数用于运行单个命令并退出
                        .output()
                        .expect("Failed to execute command");

                    // 将输出转换为字符串并打印
                    debug!("Status: {}", output.status);
                    debug!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                    debug!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                    // return exit();
                }

                Task::none()
            }
            _ => Task::none(),
        }
    }
}
fn main() -> iced::Result {
    env_logger::Builder::from_env(Env::default().default_filter_or("together=debug")).init();
    // Load the PNG file
    let img_path = "together.png"; // Path to your PNG file
    let img = image::open(img_path).expect("Failed to open image");

    // Convert the image to RGBA8
    let rgba_img = img.to_rgba8();
    // Get the width, height, and raw RGBA bytes
    let (width, height) = rgba_img.dimensions();
    let rgba_bytes: Vec<u8> = rgba_img.into_raw();
    iced::application("Together", App::update, App::view)
        .window(Settings {
            size: Size::new(800.0, 600.0),
            position: Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(icon::from_rgba(rgba_bytes, width, height).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        })
        .subscription(App::subscription)
        .run()
}
