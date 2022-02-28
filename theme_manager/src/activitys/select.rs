use super::Message;
use iced::{Container, Element, Length, Text};

fn into_tab<'a, T>(win: &mut super::Window, content: T) -> Element<'a, Message>
where
	T: Into<Element<'a, Message>>,
{
	Container::new(content)
		.width(Length::Fill)
		.height(Length::Fill)
		.center_x()
		.center_y()
//		.style(&win.theme)
		.into()
}

pub fn view(win: &mut super::Window) -> Element<super::Message> {
	let content = Text::new("TODO select");
	into_tab(win, content)
}
