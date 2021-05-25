use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WithOption<T, U> {
	pub always: T,
	pub maybe: Option<U>,
}

impl<T, U> WithOption<T, U> {
	pub fn single(first: T) -> Self {
		Self {
			always: first,
			maybe: None,
		}
	}

	pub fn both(first: T, unused: U) -> Self {
		Self {
			always: first,
			maybe: Some(unused),
		}
	}
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LeakedStringRegistry {
	inner: HashSet<&'static str>,
}

impl LeakedStringRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert_then_leak(&mut self, s: String) -> WithOption<&'static str, String> {
		match self.inner.get(&s[..]) {
			Some(s_static) => WithOption::both(*s_static, s),
			None => {
				let s_box = Box::new(s);
				let s_static = Box::leak(s_box) as &'static str;
				self.inner.insert(s_static);
				WithOption::single(s_static)
			}
		}
	}

	pub fn insert_then_leak_else_drop(&mut self, s: String) -> &'static str {
		self.insert_then_leak(s).always
	}

	pub fn insert_then_leak_to_string(&mut self, s: &str) -> &'static str {
		match self.inner.get(s) {
			Some(s_static) => *s_static,
			None => {
				let s_box = Box::new(s.to_string());
				let s_static = Box::leak(s_box) as &'static str;
				self.inner.insert(s_static);
				s_static
			}
		}
	}

	pub fn get<S>(&mut self, s: &S) -> Option<&'static str>
	where
		&'static str: Borrow<S>,
		S: Eq + Hash,
	{
		self.inner.get(s).copied()
	}
}
