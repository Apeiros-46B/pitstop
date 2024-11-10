use iced::{widget::text_editor as editor, Element};

pub struct Search {
	content: editor::Content,
}

#[derive(Clone, Debug)]
pub struct SearchMsg {

}

impl Search {
	pub fn new() -> Self {
		Self::with_text("")
	}

	pub fn with_text(text: &str) -> Self {
		Self {
			content: editor::Content::with_text(text),
		}
	}

	pub fn update(&mut self, msg: SearchMsg) {

	}

	// pub fn view(&self) -> Element<>
}
