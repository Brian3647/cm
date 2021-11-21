use std::fmt::Display;

/// A type for a commit.
/// Used to differentiate commits and filter them easily.
pub enum CommitType {
	Feat,
	Fix,
	Docs,
	Style,
	Refactor,
	Perf,
	Test,
	Chore
}

// Return a description of the commit type
impl Display for CommitType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			CommitType::Feat => "A new feature",
			CommitType::Fix => "A bug fix",
			CommitType::Docs => "Documentation updates",
			CommitType::Style => "Changes dat do not affect the meaning of the code",
			CommitType::Refactor => "A code change that neither fixes a bug or adds a feature",
			CommitType::Perf => "A code change that improves performance",
			CommitType::Test => "Adding or updating tests",
			CommitType::Chore => "Changes to the build process or auxiliary tools"
		})
	}
}

// This is used in `cli.rs` for getting the commit type based on the full commit message
impl From<String> for CommitType {
	fn from(x: String) -> Self {
		match x.split(' ').collect::<Vec<&str>>()[0] {
			"test:" => Self::Test,
			"chore:" => Self::Chore,
			"perf:" => Self::Perf,
			"refactor:" => Self::Refactor,
			"style:" => Self::Style,
			"docs:" => Self::Docs,
			"fix:" => Self::Fix,
			"feat:" => Self::Feat,
			// Unreachable.
			_ => panic!("Unknown commit type {}", x)
		}
	}
}
