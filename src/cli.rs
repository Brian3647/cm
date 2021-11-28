use colored::Colorize;
use inquire::error::InquireResult;

use crate::commit::Commit;
use crate::commit_type::CommitType;
use crate::git;
use crate::prompt::confirm;
use crate::prompt::get_commit_type;
use crate::prompt::input;
use crate::prompt::optinput;

/// The main CLI entry point.
/// This function returns a result from the `inquire` crate.
pub fn init() -> InquireResult<()> {
	let ctype = CommitType::from(get_commit_type()?);
	let msg = input("Introduce a commit message:")?;
	let scope = optinput("Add a scope to the commit (optional):")?;
	let add = confirm("Do you want to add all files ('.') before committing?")?;
	let commit = Commit::new(ctype, msg, scope).to_string();

	// User confirmed new changes should be added
	if add {
		git::add('.')
	};

	// Show a newline and commit changes
	println!();
	git::commit_msg(&commit);

	// Print the generated commit message
	println!("\n{}", (&commit).blue());

	Ok(())
}
