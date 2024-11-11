mod command;
mod providers;

use iced::{widget, Length, Task};

use crate::app::Msg;

use command::Entry;

#[derive(Default)]
pub struct List {
	entries: Vec<Entry>,
	focused: usize,
}

#[derive(Clone, Debug)]
pub enum ListMsg {
	Confirm,
	Nth { n: usize, confirm: bool }, // TODO: make this select the nth on the current page
	Up,
	Down,
	PgUp,
	PgDown,
	// TODO: add a message that gets sent when the window is closed
}

impl From<ListMsg> for Msg {
	fn from(x: ListMsg) -> Self {
		Msg::List(x)
	}
}

impl List {
	pub fn update(&mut self, msg: ListMsg) -> Task<Msg> {
		match msg {
			ListMsg::Confirm => todo!(),
			ListMsg::Nth { n, confirm } => {
				self.focused = n.min(self.entries.len());
				if confirm {
					todo!()
				}
			},
			ListMsg::Up => self.focused = self.focused.saturating_sub(1),
			ListMsg::Down => {
				self.focused = self.focused.saturating_add(1).min(self.entries.len())
			},
			ListMsg::PgUp => todo!(),
			ListMsg::PgDown => todo!(),
		}
		Task::none()
	}

	pub fn view(&self) -> iced::Element<ListMsg> {
		// widget::scrollable(
			widget::Column::with_children(
				self
					.entries
					.iter()
					.map(|entry| widget::text(entry.to_string()).into()),
			)
			.width(Length::Fill)
			// ,
		// )
		// .width(Length::Fill)
		// .height(Length::Fill)
		.into()
	}
}
