// TODO: redo this whole mess
use std::{any::Any, cell::RefCell, rc::Rc};

use super::command::Entry;

pub trait Provider
where
	Self: 'static,
{
	fn new() -> Self
	where
		Self: Sized;

	fn entries(&self, parent: Rc<RefCell<WrappedProvider>>) -> Box<[Entry]>;
	fn activators(&self) -> &'static [fn(Entry)];

	fn activate(&self, e: Entry) {
		(self.activators()[e.ty])(e);
	}
}

pub struct WrappedProvider {
	memo: Option<Vec<Entry>>,
	inner: Rc<RefCell<dyn Any>>, // is actually dyn Provider
}

impl<'a> WrappedProvider {
	pub fn new<T: Provider>() -> Self {
		Self {
			memo: None,
			inner: Rc::new(RefCell::new(T::new())),
		}
	}
}

pub struct FileTreeProvider {
	dir: std::path::PathBuf,
}

impl Provider for FileTreeProvider {
	fn new() -> Self {
		Self {
			dir: homedir::my_home().unwrap().unwrap(),
		}
	}

	fn entries(&self, parent: Rc<RefCell<WrappedProvider>>) -> Box<[Entry]> {
		let rc = parent.clone();
		let memo = &mut parent.borrow_mut().memo;
		if memo.is_none() {
			*memo = Some(
				std::fs::read_dir(self.dir.clone())
					.unwrap()
					.map(Result::unwrap)
					.map(move |x| Entry {
						ty: if x.file_type().unwrap().is_dir() {
							0
						} else {
							1
						},
						name: x.file_name().to_string_lossy().into(),
						provider: rc.clone(),
					})
					.collect(),
			);
		}
		memo.clone().unwrap().into_boxed_slice()
	}

	fn activators(&self) -> &'static [fn(Entry)] {
		&[Self::activate_dir, Self::activate_file]
	}
}

impl FileTreeProvider {
	fn activate_dir(e: Entry) {
		let tmp1 = e.provider
			.borrow_mut()
			.inner
			.clone();
		let mut tmp2 = tmp1.borrow_mut();

		let this = tmp2
			.downcast_mut::<Self>()
			.unwrap();

		this.dir = e.name.to_string().into();
	}

	fn activate_file(e: Entry) {
		let tmp1 = e.provider
			.borrow_mut()
			.inner
			.clone();
		let mut tmp2 = tmp1.borrow_mut();

		let this = tmp2
			.downcast_mut::<Self>()
			.unwrap();

		let mut path = this.dir.clone();
		path.push(e.to_string());
		open::that_detached(path).unwrap();
	}
}
