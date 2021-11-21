use inquire::error::InquireResult;
use inquire::Confirm;
use inquire::Select;
use inquire::Text;

/// Creates an option used in `get_commit_type`
macro_rules! option {
	($base:expr,$ct:ident) => {{
		use crate::commit_type::CommitType::$ct;

		format!("{}: {}", $base, $ct)
	}};
}

/// Simple prompt
pub fn input(m: &str) -> InquireResult<String> {
	Text::new(m)
		.with_validator(&|x| {
			if x.trim().is_empty() {
				Err("Input can't be empty.".into())
			} else {
				Ok(())
			}
		})
		.prompt()
}

/// Optional input
pub fn optinput(m: &str) -> InquireResult<Option<String>> {
	match Text::new(m).prompt_skippable() {
		Ok(Some(x)) => Ok({
			if x.trim().is_empty() {
				None
			} else {
				Some(x)
			}
		}),

		other => other
	}
}

/// Gets the commit type based on a selection
pub fn get_commit_type() -> InquireResult<String> {
	let options = vec![
		option!("feat", Feat),
		option!("fix", Fix),
		option!("docs", Docs),
		option!("style", Style),
		option!("refactor", Refactor),
		option!("perf", Perf),
		option!("test", Test),
		option!("chore", Chore),
	];

	Select::new("Select the type of commit:", options).prompt()
}

pub fn confirm(m: &str) -> InquireResult<bool> {
	Confirm::new(m).prompt()
}
