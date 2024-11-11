use std::{cell::RefCell, fmt, rc::Rc};

use smol_str::SmolStr;

use super::providers::WrappedProvider;

#[derive(Clone)]
pub struct Entry {
	pub ty: usize, // one type per activation function
	pub name: SmolStr,
	pub provider: Rc<RefCell<WrappedProvider>>,
}

impl fmt::Display for Entry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}
