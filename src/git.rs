//! Interacting with git commands

use std::fmt::Display;

use crate::system;

/// Commit with a message
pub fn commit_msg<T: ToString>(m: T) {
	system!(
		"git commit -m \"{}\"",
		m.to_string().replace('"', "\\\"").replace('`', "\\`")
	);
}

/// `p` needs to be a path
pub fn add<T: Display>(p: T) {
	system!("git add {}", p)
}
