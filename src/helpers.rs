use std::process::Command;
use std::process::Stdio;

/// Executes a command
///
/// # Panics
///
/// It can't panic but it can exit the program if the command fails.
#[macro_export]
macro_rules! system {
	($($args:expr),*) => {
		$crate::helpers::system(format!($($args),*))
	};
}

#[doc(hidden)]
/// Executes a command
/// Uses cmd on Windows and sh on Unix
pub fn system(cmd: String) {
	let program;
	let firstarg;

	if cfg!(target_os = "windows") {
		program = "cmd";
		firstarg = "/C";
	} else {
		program = "sh";
		firstarg = "-c";
	}

	let out = Command::new(program)
		.arg(firstarg)
		.arg(cmd)
		.stderr(Stdio::inherit())
		.stdout(Stdio::inherit())
		.stdin(Stdio::inherit())
		.output()
		.unwrap_or_else(|e| crate::err!("{}", e));

	if !out.status.success() {
		std::process::exit(out.status.code().unwrap_or(1))
	}
}
