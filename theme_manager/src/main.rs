use iced::{Container, Element, Length, Sandbox, Settings};

mod activitys;
use activitys::{Activity, Message};

pub struct Window {
	activity: Activity,
	theme: Box<dyn typetest_themes::ApplicationTheme>,
}

impl Sandbox for Window {
	type Message = Message;

	fn new() -> Window {
		Window {
			theme: typetest_themes::Theme::DefaultDark.into(),
			activity: Activity::Select,
		}
	}

	fn title(&self) -> String {
		String::from("rusty-vocabulary")
	}

	fn update(&mut self, message: Self::Message) {}

	fn view(&mut self) -> Element<Self::Message> {
		let content: Element<Self::Message> = match self.activity {
			Activity::Select => activitys::select::view(self),
		};
		Container::new(content)
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.style(&typetest_themes::Theme::DefaultDark.into())
			.into()
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
