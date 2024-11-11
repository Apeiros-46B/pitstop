use iced::advanced::widget::{
	operation::{self, focusable, Focusable},
	Id, Operation,
};

pub fn focus_first<T>() -> impl Operation<T>
where
	T: Send + 'static,
{
	struct FocusType {
		count: focusable::Count,
	}

	impl<T> Operation<T> for FocusType {
		fn focusable(&mut self, state: &mut dyn Focusable, _id: Option<&Id>) {
			match self.count.focused {
				None => state.focus(),
				Some(0) => state.focus(),
				Some(_) => state.unfocus(),
			}
		}

		fn container(
			&mut self,
			_id: Option<&Id>,
			_bounds: iced::Rectangle,
			operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
		) {
			operate_on_children(self);
		}
	}

	operation::then(focusable::count(), |count| FocusType { count })
}
