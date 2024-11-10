// use std::any::{Any, TypeId};

// use iced::{
// 	advanced::{
// 		text::highlighter::PlainText,
// 		widget::{self, operation, Operation},
// 	},
// 	widget::TextEditor,
// };

// use crate::app::Msg;

// pub struct FocusEditor {
// 	target: widget::Id,
// }

// impl FocusEditor {
// 	pub fn new(target: widget::Id) -> Self {
// 		Self { target }
// 	}
// }

// impl Operation for FocusEditor {
// 	fn container(
// 		&mut self,
// 		_id: Option<&widget::Id>,
// 		_bounds: iced::Rectangle,
// 		operate_on_children: &mut dyn FnMut(&mut dyn Operation<()>),
// 	) {
// 		operate_on_children(self);
// 	}

// 	fn focusable(
// 		&mut self,
// 		x: &mut dyn operation::Focusable,
// 		id: Option<&widget::Id>,
// 	) {
// 		if x.type_id() == TypeId::of::<TextEditor<PlainText, Msg>>() {}
// 		if let Some(id) = id {
// 			if *id == self.target {}
// 		}
// 	}
// }

// pub fn focus()
