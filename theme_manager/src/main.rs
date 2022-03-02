use iced::{Element, Sandbox, Settings};
use iced_theme::theme::Theme;

mod activitys;
use activitys::{Activity, Message};

pub struct Window {
	activity: Activity,
	theme: Theme,
}

impl Sandbox for Window {
	type Message = Message;

	fn new() -> Window {
		Window {
			theme: Theme::default(),
			activity: Activity::Select,
		}
	}

	fn title(&self) -> String {
		String::from("Iced Theme Manager")
	}

	fn update(&mut self, message: Self::Message) {}

	fn view(&mut self) -> Element<Self::Message> {
		match self.activity {
			Activity::Select => activitys::select::view(self),
		}
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
