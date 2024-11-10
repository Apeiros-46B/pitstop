use iced::widget::text_editor as editor;

#[derive(Default)]
pub struct Search {
	content: editor::Content,
}

#[derive(Clone, Debug)]
pub enum SearchMsg {
	Action(editor::Action),
}

impl Search {
	pub fn with_text(text: &str) -> Self {
		Self {
			content: editor::Content::with_text(text),
		}
	}

	pub fn update(&mut self, msg: SearchMsg) {
		match msg {
			SearchMsg::Action(act) => self.content.perform(act),
		}
	}

	pub fn view(&self) -> iced::Element<SearchMsg> {
		editor(&self.content).on_action(SearchMsg::Action).into()
	}
}
