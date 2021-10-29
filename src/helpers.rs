use std::io;
use std::io::Write;
use std::process::Command;

#[macro_export]
macro_rules! system {
	($($args:expr),*) => {
		crate::helpers::__system(format!($($args),*))
	};
}

pub fn __system(cmd: String) {
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
		.output()
		.unwrap_or_else(|e| crate::err!("{}", e));

	io::stdout().write_all(&out.stdout).unwrap();
	io::stderr().write_all(&out.stderr).unwrap();

	if !out.status.success() {
		std::process::exit(out.status.code().unwrap_or(1))
	}
}
