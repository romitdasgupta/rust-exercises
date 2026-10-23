//! `jobq`: the E07 capstone binary. Keep it thin; the logic lives in `ecosystem::e07_capstone`.

use std::process::ExitCode;

/// E07. Wire the program together:
/// 1. Parse with `Cli::parse()` (fine in `main`: on bad usage clap prints and exits 2).
/// 2. Install the global tracing subscriber here and only here: logs to stderr, no
///    ANSI colors, max level WARN by default, INFO for `-v`, DEBUG for `-vv` or more.
/// 3. Build a current-thread tokio runtime with time enabled and `block_on(execute(..))`
///    with stdout as the output.
/// 4. On `Ok(code)` exit with that code. On `Err(err)`, print `error: {err:#}` to stderr
///    and exit with `error_exit_code(&err)`.
fn main() -> ExitCode {
    todo!("Implement this task")
}
