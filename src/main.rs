mod cli;
mod commit;
mod commit_type;
mod git;
mod helpers;
mod prompt;

use inquire::error::InquireError;

#[macro_export]
macro_rules! err {
	($($args:expr),*) => {{
		println!();
		eprintln!($($args),*);
		std::process::exit(1);
	}};
}

fn main() {
	if let Err(e) = cli::init() {
		match e {
			InquireError::NotTTY => err!("Sorry, but currently tty is needed"),
			InquireError::InvalidConfiguration(info) => err!("Invalid configuration. {}", info),
			InquireError::OperationCanceled => err!("Cancelled."),
			InquireError::OperationInterrupted => err!("Interrupted."),
			_ => panic!("{}", e)
		}
	};
}
