use crate::commit_type::CommitType;
use std::fmt::Display;

pub struct Commit {
	pub ctype: CommitType,
	pub msg: String,
	pub scope: Option<String>
}

impl Commit {
	pub fn new(ctype: CommitType, msg: String, scope: Option<String>) -> Self {
		Self { ctype, msg, scope }
	}

	pub fn scope(&self) -> String {
		match &self.scope {
			Some(x) => format!("({})", x),
			None => "".into()
		}
	}

	pub fn ctype(&self) -> String {
		match &self.ctype {
			CommitType::Feat => "feat",
			CommitType::Fix => "fix",
			CommitType::Docs => "docs",
			CommitType::Style => "style",
			CommitType::Refactor => "refactor",
			CommitType::Perf => "perf",
			CommitType::Test => "test",
			CommitType::Chore => "chore"
		}
		.into()
	}
}

impl Display for Commit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}: {}", self.ctype(), self.scope(), self.msg)
	}
}
