use std::process::ExitStatus;

pub const EXIT_SUCCESS: i32 = 0;

pub fn check_exit_ok(status: ExitStatus) -> bool {
	if let Some(code) = status.code() {
		code == EXIT_SUCCESS
	} else {
		true
	}
}
