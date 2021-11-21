use crate::commit_type::CommitType;
use std::fmt::Display;

/// A commit representation
pub struct Commit {
	/// The commit type
	pub ctype: CommitType,
	/// Commit message
	pub msg: String,
	/// Scope of the commit (if any)
	pub scope: Option<String>
}

impl Commit {
	pub fn new(ctype: CommitType, msg: String, scope: Option<String>) -> Self {
		Self { ctype, msg, scope }
	}

	/// Convert the scope into a string used to create the commit message
	pub fn scope(&self) -> String {
		self
			.scope
			.as_ref()
			.map(|x| format!("({})", x))
			.unwrap_or_else(|| "".into())
	}

	/// Converts the commit type into lowercase
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
	// Example result: "docs(test): hello world!"
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}: {}", self.ctype(), self.scope(), self.msg)
	}
}
