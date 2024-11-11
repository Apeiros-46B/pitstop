use std::sync::Arc;

use iced::{
	keyboard::{key::Named, Key},
	widget::{
		self,
		text_editor::{Action, Binding, Content, Edit, KeyPress},
	},
};

use crate::app::Msg;

use super::ListMsg;

#[derive(Default)]
pub struct Search {
	content: Content,
}

#[derive(Clone, Debug)]
pub enum SearchMsg {
	Action(Action),
	Replace(Option<String>),
}

impl From<SearchMsg> for Msg {
	fn from(x: SearchMsg) -> Self {
		Msg::Search(x)
	}
}

impl Search {
	pub fn update(&mut self, msg: SearchMsg) {
		match msg {
			SearchMsg::Action(act) => match act {
				// forbid entering spaces at the start of the string
				Action::Edit(Edit::Insert(' '))
					if self.content.cursor_position().1 == 0 => {},

				// strip newlines from pasted text and remove leading/trailing whitespace
				Action::Edit(Edit::Paste(text)) => {
					let new = Arc::new(text.replace(['\r', '\n'], "").trim().to_string());
					self.content.perform(Action::Edit(Edit::Paste(new)))
				},

				_ => self.content.perform(act),
			},
			SearchMsg::Replace(text) => self.replace(text),
		}
	}

	pub fn view(&self) -> iced::Element<Msg> {
		widget::text_editor(&self.content)
			.placeholder("enter query or command")
			.wrapping(widget::text::Wrapping::None)
			.key_binding(Self::on_key)
			.on_action(Self::on_action)
			.into()
	}
}

impl Search {
	pub fn replace(&mut self, text: Option<String>) {
		self.content.perform(Action::SelectAll);
		let edit = if let Some(text) = text {
			Edit::Paste(Arc::new(text))
		} else {
			Edit::Backspace
		};
		self.content.perform(Action::Edit(edit));
	}

	fn on_key(evt: KeyPress) -> Option<Binding<Msg>> {
		let mods = evt.modifiers;
		// TODO: make this rebindable in config
		match evt.key {
			Key::Named(Named::Escape) => custom(Msg::CloseWindow),
			Key::Named(Named::Enter) => custom(ListMsg::Confirm.into()),
			Key::Named(Named::ArrowUp) => custom(ListMsg::Up.into()),
			Key::Named(Named::ArrowDown) => custom(ListMsg::Down.into()),
			Key::Named(Named::PageUp) => custom(ListMsg::PgUp.into()),
			Key::Named(Named::PageDown) => custom(ListMsg::PgDown.into()),
			Key::Character(ref s) => match s.parse::<usize>() {
				Ok(n) if (1..=9).contains(&n) && (mods.command() ^ mods.alt()) => {
					// Ctrl/Cmd+N = select N
					// Alt+N = confirm N
					custom(
						ListMsg::Nth {
							n: n - 1,
							confirm: mods.alt(),
						}
						.into(),
					)
				},
				_ => key_fallback(evt),
			},
			_ => key_fallback(evt),
		}
	}

	fn on_action(act: Action) -> Msg {
		match act {
			Action::Scroll { .. } => ListMsg::Down.into(),
			_ => SearchMsg::Action(act).into(),
		}
	}
}

fn custom(msg: Msg) -> Option<Binding<Msg>> {
	Some(Binding::Custom(msg))
}

fn key_fallback(evt: KeyPress) -> Option<Binding<Msg>> {
	// if evt.text.is_some() {

	// } else {
		Binding::from_key_press(evt)
	// }
}
