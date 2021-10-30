use std::fmt::Display;

use crate::system;

pub fn commit_msg<T: ToString>(m: T) {
	system!(
		"git commit -m \"{}\"",
		m.to_string().replace('"', "\\\"").replace('`', "\\`")
	);
}

pub fn add<T: Display>(p: T) {
	system!("git add {}", p)
}
