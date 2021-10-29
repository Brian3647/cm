use colored::Colorize;
use inquire::error::InquireResult;

use crate::commit::Commit;
use crate::commit_type::CommitType;
use crate::git;
use crate::prompt::confirm;
use crate::prompt::get_commit_type;
use crate::prompt::input;
use crate::prompt::optinput;

pub fn init() -> InquireResult<()> {
	let ctype = CommitType::from(get_commit_type()?);
	let scope = optinput("Add a scope to the commit (optional):")?;
	let msg = input("Introduce a commit message:")?;
	let add = confirm("Do you want to add all files ('.') before committing?")?;
	let commit = Commit::new(ctype, msg, scope).to_string();

	if add {
		git::add('.')
	};

	git::commit_msg(&commit);

	println!("\n{}", (&commit).blue());

	Ok(())
}
