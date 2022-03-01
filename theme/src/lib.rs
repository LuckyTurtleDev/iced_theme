pub mod theme {
	use iced::{button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input, Color};

	#[derive(Debug, Clone)]
	pub struct ThemeColors {
		surface: Color,
		background: Color,
		accent: Color,
		active: Color,
		hovered: Color,
		text: Color,
	}

	#[derive(Debug, Clone)]
	pub struct Theme {
		name: String,
		description: Option<String>,
		dark: bool,
		colors: ThemeColors,
	}

	impl Default for Theme {
		fn default() -> Theme {
			let colors = ThemeColors {
				surface: Color::from_rgb8(0x40, 0x44, 0x4B),
				accent: Color::from_rgb8(0x6F, 0xFF, 0xE9),
				active: Color::from_rgb8(0x72, 0x89, 0xDA),
				hovered: Color::from_rgb8(0x67, 0x7B, 0xC4),
				background: Color::from_rgb8(0x36, 0x39, 0x3F),
				text: Color::WHITE,
			};
			Theme {
				name: "Default Dark".into(),
				description: Some("The default dark theme based on the iced styling example".into()),
				dark: true,
				colors,
			}
		}
	}

	impl container::StyleSheet for ThemeColors {
		fn style(&self) -> container::Style {
			container::Style {
				background: self.background.into(),
				text_color: Some(self.text),
				..container::Style::default()
			}
		}
	}

	impl<'a> From<&Theme> for Box<dyn container::StyleSheet + 'a> {
		fn from(theme: &Theme) -> Self {
			theme.colors.clone().into()
		}
	}

	impl radio::StyleSheet for Theme {
		fn active(&self) -> radio::Style {
			radio::Style {
				background: self.colors.surface.into(),
				text_color: Some(self.colors.text),
				dot_color: self.colors.active,
				border_width: 1.0,
				border_color: self.colors.active,
			}
		}

		fn hovered(&self) -> radio::Style {
			radio::Style {
				background: Color {
					a: 0.5,
					..self.colors.surface
				}
				.into(),
				..self.active()
			}
		}
	}

	impl text_input::StyleSheet for Theme {
		fn active(&self) -> text_input::Style {
			text_input::Style {
				background: self.colors.surface.into(),
				border_radius: 2.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
			}
		}

		fn focused(&self) -> text_input::Style {
			text_input::Style {
				border_width: 1.0,
				border_color: self.colors.accent.into(),
				..self.active()
			}
		}

		fn hovered(&self) -> text_input::Style {
			text_input::Style {
				border_width: 1.0,
				border_color: Color {
					a: 0.3,
					..self.colors.accent
				},
				..self.focused()
			}
		}

		fn placeholder_color(&self) -> Color {
			Color::from_rgb(0.4, 0.4, 0.4)
		}

		fn value_color(&self) -> Color {
			Color::WHITE
		}

		fn selection_color(&self) -> Color {
			self.colors.accent
		}
	}

	impl button::StyleSheet for Theme {
		fn active(&self) -> button::Style {
			button::Style {
				background: self.colors.active.into(),
				border_radius: 3.0,
				text_color: self.colors.text,
				..button::Style::default()
			}
		}

		fn hovered(&self) -> button::Style {
			button::Style {
				background: self.colors.hovered.into(),
				text_color: self.colors.text,
				..self.active()
			}
		}

		fn pressed(&self) -> button::Style {
			button::Style {
				border_width: 1.0,
				border_color: Color::WHITE,
				..self.hovered()
			}
		}
	}

	impl scrollable::StyleSheet for Theme {
		fn active(&self) -> scrollable::Scrollbar {
			scrollable::Scrollbar {
				background: self.colors.surface.into(),
				border_radius: 2.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				scroller: scrollable::Scroller {
					color: self.colors.active,
					border_radius: 2.0,
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
				},
			}
		}

		fn hovered(&self) -> scrollable::Scrollbar {
			let active = self.active();

			scrollable::Scrollbar {
				background: Color {
					a: 0.5,
					..self.colors.surface
				}
				.into(),
				scroller: scrollable::Scroller {
					color: self.colors.hovered,
					..active.scroller
				},
				..active
			}
		}

		fn dragging(&self) -> scrollable::Scrollbar {
			let hovered = self.hovered();

			scrollable::Scrollbar {
				scroller: scrollable::Scroller {
					color: Color::from_rgb(0.85, 0.85, 0.85),
					..hovered.scroller
				},
				..hovered
			}
		}
	}

	impl slider::StyleSheet for Theme {
		fn active(&self) -> slider::Style {
			slider::Style {
				rail_colors: (self.colors.active, Color {
					a: 0.1,
					..self.colors.active
				}),
				handle: slider::Handle {
					shape: slider::HandleShape::Circle { radius: 9.0 },
					color: self.colors.active,
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
				},
			}
		}

		fn hovered(&self) -> slider::Style {
			let active = self.active();

			slider::Style {
				handle: slider::Handle {
					color: self.colors.hovered,
					..active.handle
				},
				..active
			}
		}

		fn dragging(&self) -> slider::Style {
			let active = self.active();

			slider::Style {
				handle: slider::Handle {
					color: Color::from_rgb(0.85, 0.85, 0.85),
					..active.handle
				},
				..active
			}
		}
	}

	impl progress_bar::StyleSheet for Theme {
		fn style(&self) -> progress_bar::Style {
			progress_bar::Style {
				background: self.colors.surface.into(),
				bar: self.colors.active.into(),
				border_radius: 10.0,
			}
		}
	}

	impl checkbox::StyleSheet for Theme {
		fn active(&self, is_checked: bool) -> checkbox::Style {
			checkbox::Style {
				background: if is_checked { self.colors.active } else { self.colors.surface }.into(),
				text_color: Some(self.colors.text),
				checkmark_color: self.colors.text,
				border_radius: 2.0,
				border_width: 1.0,
				border_color: self.colors.active,
			}
		}

		fn hovered(&self, is_checked: bool) -> checkbox::Style {
			checkbox::Style {
				background: Color {
					a: 0.8,
					..if is_checked { self.colors.active } else { self.colors.surface }
				}
				.into(),
				..self.active(is_checked)
			}
		}
	}

	impl rule::StyleSheet for Theme {
		fn style(&self) -> rule::Style {
			rule::Style {
				color: self.colors.surface,
				width: 2,
				radius: 1.0,
				fill_mode: rule::FillMode::Padded(15),
			}
		}
	}
}
