use iced::{color, widget, Color, Length, Task};

use crate::{app::Msg, ipc::IpcMsg};

#[derive(Clone, Copy, Default)]
pub enum ListMode {
	#[default]
	Omni,
	Dmenu,
}

#[derive(Default)]
pub struct List {
	mode: ListMode,
	content: Vec<String>, // TODO: replace with Vec<Entry> where Entry is custom struct
	focused: usize,
}

#[derive(Clone, Debug)]
pub enum ListMsg {
	Confirm,
	Nth { n: usize, confirm: bool },
	Up,
	Down,
	PgUp,
	PgDown,
	// TODO: add a message that gets sent when the window is closed
}

impl List {
	pub fn update(&mut self, msg: ListMsg) -> Task<Msg> {
		match msg {
			ListMsg::Confirm => match self.mode {
				ListMode::Omni => todo!(),
				ListMode::Dmenu => {
					todo!()
					// return Task::done(Msg::Ipc(IpcMsg::InternalDmenuResult(
					// 	self.content[self.focused].clone(),
					// )))
				},
			},
			ListMsg::Nth { n, confirm } => {
				self.focused = n;
				if confirm {
					todo!()
				}
			},
			ListMsg::Up => self.focused = self.focused.saturating_sub(1),
			ListMsg::Down => {
				self.focused = self.focused.saturating_add(1).min(self.content.len())
			},
			ListMsg::PgUp => todo!(),
			ListMsg::PgDown => todo!(),
		}
		Task::none()
	}

	pub fn view(&self) -> iced::Element<ListMsg> {
		widget::scrollable(
			widget::Column::with_children(
				self
					.content
					.iter()
					.map(|entry| widget::text(entry).into()),
			)
			.width(Length::Fill),
		)
		.width(Length::Fill)
		.height(Length::Fill)
		.into()
	}
}

impl List {
	pub fn set_mode(&mut self, mode: ListMode) {
		self.mode = mode;
		match mode {
			ListMode::Omni => {
				// TODO
				self.content = (1..100).map(|x| x.to_string()).collect()
			},
			ListMode::Dmenu => self.content.clear(),
		}
	}

	pub fn dmenu_set_entries(&mut self, entries: Vec<String>) {
		self.content = entries;
	}
}
