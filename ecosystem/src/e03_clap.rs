//! LESSON E03 — Command-line interfaces with clap's derive API (after Chapter 18)
//! Run: cargo test --manifest-path ecosystem/Cargo.toml e03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What exit code does `--help` produce,
//! and on which stream? Why should testable code never call `Cli::parse()`?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Trace `jobctl submit build --priority 11` from argv to the error
//! text and exit code. Which part rejects 11, and why is that better than checking
//! later in `run`?
//! JUDGMENT: clap is the default for real CLIs: help, suggestions, validation, and
//! completions come for free, at a compile-time cost. For a tiny internal tool,
//! `std::env::args` or a small crate (lexopt, pico-args) can be enough.

use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write;
use std::time::Duration;

/// A. The parsed command line of `jobctl`. Contract:
/// - `-v`/`--verbose` may repeat (`-vv` = 2) and is accepted before or after the
///   subcommand (`global`);
/// - the program name is `jobctl` and `--version` prints `jobctl 1.0.0`.
#[derive(Debug, PartialEq, Parser)]
#[command(name = "jobctl", version = "1.0.0", about = "Submit and inspect jobs")]
pub struct Cli {
    /// More output (-v, -vv)
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Command,
}

/// A. Subcommands:
/// - `submit NAME [--priority N] [--timeout DURATION] [--tag TAG]...`:
///   priority 1..=10, default 5; timeout parsed by `parse_duration`; repeatable `--tag`.
/// - `list [--format table|json]`, default `table`.
/// - `cancel ID...` at least one numeric id.
#[derive(Debug, PartialEq, Subcommand)]
pub enum Command {
    /// Submit a job
    Submit {
        name: String,
        priority: u8,
        #[arg(long, value_parser = parse_duration)]
        timeout: Option<Duration>,
        tags: Vec<String>,
    },
    /// List jobs
    List { format: Format },
    /// Cancel jobs by id
    Cancel { ids: Vec<u32> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Table,
    Json,
}

/// B. Parse `250ms`, `30s`, or `5m` (a u64 followed by exactly one unit).
/// Errors are human-readable strings; clap shows them inside its own message.
/// - unknown or missing unit: ``unknown unit in `TEXT` (use ms, s, or m)``
/// - bad number: ``invalid number in `TEXT` ``
pub fn parse_duration(text: &str) -> Result<Duration, String> {
    todo!("Implement this task")
}

/// C. Run the CLI the way `main` would, but testably. Parse with `Cli::try_parse_from`.
/// - On a clap error, render it (`err.render()` or Display) to `stdout` when it is
///   help or version output and to `stderr` otherwise; return `err.exit_code()`.
/// - On success, write one line to `stdout` and return 0:
///   `submit NAME priority=P timeout=T tags=A,B` (timeout in ms, `none` if absent,
///   tags joined by commas, possibly empty), `list FORMAT` (`table` or `json`),
///   or `cancel ID ID ...`.
/// - When verbose > 0, first write `verbose=N` on its own line to `stderr`.
pub fn run<I, T>(args: I, stdout: &mut impl Write, stderr: &mut impl Write) -> i32
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    todo!("Implement this task")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guess;
    use clap::CommandFactory;
    use clap::error::ErrorKind;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("jobctl").chain(args.iter().copied()))
    }

    fn run_captured(args: &[&str]) -> (i32, String, String) {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = run(
            std::iter::once("jobctl").chain(args.iter().copied()),
            &mut out,
            &mut err,
        );
        (
            code,
            String::from_utf8(out).unwrap(),
            String::from_utf8(err).unwrap(),
        )
    }

    #[test]
    fn a_definition_passes_clap_debug_asserts() {
        // Catches conflicting names and bad defaults at test time instead of in production.
        Cli::command().debug_assert();
    }

    #[test]
    fn a_submit_parses_flags_defaults_and_repeats() {
        let cli = parse(&["-vv", "submit", "build", "--tag", "ci", "--tag", "fast"]).unwrap();
        assert_eq!(cli.verbose, 2);
        assert_eq!(
            cli.command,
            Command::Submit {
                name: "build".into(),
                priority: 5,
                timeout: None,
                tags: vec!["ci".into(), "fast".into()],
            }
        );
        let cli = parse(&["submit", "x", "--priority=10", "--timeout", "2s", "-v"]).unwrap();
        assert_eq!(cli.verbose, 1, "-v is global");
        assert!(matches!(
            cli.command,
            Command::Submit { priority: 10, timeout: Some(t), .. } if t == Duration::from_secs(2)
        ));
    }

    #[test]
    fn a_list_and_cancel() {
        assert_eq!(
            parse(&["list"]).unwrap().command,
            Command::List {
                format: Format::Table
            }
        );
        assert_eq!(
            parse(&["list", "--format", "json"]).unwrap().command,
            Command::List {
                format: Format::Json
            }
        );
        assert_eq!(
            parse(&["cancel", "3", "4"]).unwrap().command,
            Command::Cancel { ids: vec![3, 4] }
        );
        assert_eq!(
            parse(&["cancel"]).unwrap_err().kind(),
            ErrorKind::MissingRequiredArgument
        );
    }

    #[test]
    fn a_validation_happens_while_parsing() {
        let err = parse(&["submit", "x", "--priority", "11"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ValueValidation);
        assert!(err.to_string().contains("--priority"));
        assert_eq!(
            parse(&["list", "--format", "xml"]).unwrap_err().kind(),
            ErrorKind::InvalidValue
        );
        assert_eq!(
            parse(&["cancel", "seven"]).unwrap_err().kind(),
            ErrorKind::ValueValidation
        );
        assert_eq!(
            parse(&["deploy"]).unwrap_err().kind(),
            ErrorKind::InvalidSubcommand
        );
    }

    #[test]
    fn b_parse_duration_units_and_errors() {
        assert_eq!(parse_duration("250ms"), Ok(Duration::from_millis(250)));
        assert_eq!(parse_duration("30s"), Ok(Duration::from_secs(30)));
        assert_eq!(parse_duration("5m"), Ok(Duration::from_secs(300)));
        assert_eq!(parse_duration("0s"), Ok(Duration::ZERO));
        assert_eq!(
            parse_duration("10h"),
            Err("unknown unit in `10h` (use ms, s, or m)".to_owned())
        );
        assert_eq!(
            parse_duration("10"),
            Err("unknown unit in `10` (use ms, s, or m)".to_owned())
        );
        assert_eq!(parse_duration("s"), Err("invalid number in `s`".to_owned()));
        assert_eq!(
            parse_duration("-5s"),
            Err("invalid number in `-5s`".to_owned())
        );
    }

    #[test]
    fn b_value_parser_errors_reach_the_user() {
        let err = parse(&["submit", "x", "--timeout", "10h"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ValueValidation);
        let text = err.to_string();
        assert!(text.contains("--timeout"), "{text}");
        assert!(text.contains("unknown unit in `10h`"), "{text}");
    }

    #[test]
    fn c_success_writes_a_summary() {
        let (code, out, err) = run_captured(&[
            "submit",
            "build",
            "--timeout",
            "1m",
            "--tag",
            "a",
            "--tag",
            "b",
        ]);
        assert_eq!((code, err.as_str()), (0, ""));
        assert_eq!(out, "submit build priority=5 timeout=60000ms tags=a,b\n");
        let (code, out, err) = run_captured(&["-v", "cancel", "1", "2"]);
        assert_eq!(
            (code, out.as_str(), err.as_str()),
            (0, "cancel 1 2\n", "verbose=1\n")
        );
        assert_eq!(run_captured(&["list"]).1, "list table\n");
        assert_eq!(
            run_captured(&["submit", "n"]).1,
            "submit n priority=5 timeout=none tags=\n"
        );
    }

    #[test]
    fn c_help_and_version_go_to_stdout_with_exit_code_0() {
        let (code, out, err) = run_captured(&["--help"]);
        assert_eq!((code, err.as_str()), (0, ""));
        assert!(out.contains("Usage: jobctl"), "{out}");
        assert!(out.contains("submit") && out.contains("Submit a job"));
        let (code, out, _) = run_captured(&["--version"]);
        assert_eq!((code, out.trim()), (0, "jobctl 1.0.0"));
    }

    #[test]
    fn c_usage_errors_go_to_stderr_with_exit_code_2() {
        let (code, out, err) = run_captured(&["submit"]);
        assert_eq!((code, out.as_str()), (2, ""));
        assert!(err.contains("<NAME>"), "{err}");
        let (code, _, err) = run_captured(&["lsit"]);
        assert_eq!(code, 2);
        assert!(
            err.contains("list"),
            "clap suggests the closest subcommand: {err}"
        );
    }

    #[derive(Debug, Parser)]
    #[command(name = "p")]
    struct Probe {
        #[arg(short, action = clap::ArgAction::Count)]
        v: u8,
        #[arg(long)]
        name: Option<String>,
        rest: Vec<String>,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        let probe =
            |args: &[&str]| Probe::try_parse_from(std::iter::once("p").chain(args.iter().copied()));

        // P1. `p -v -vv`: value of v?
        let p1: u8 = guess();
        assert_eq!(
            p1,
            probe(&["-v", "-vv"]).unwrap().v,
            "P1: Count adds every occurrence"
        );

        // P2. `p -- -v x`: (v, rest)?
        let parsed = probe(&["--", "-v", "x"]).unwrap();
        let p2: (u8, Vec<&str>) = guess();
        assert_eq!(
            p2,
            (parsed.v, parsed.rest.iter().map(String::as_str).collect()),
            "P2: `--` ends option parsing"
        );

        // P3. `p --name a --name b` for an Option<String> flag: is it an error?
        let p3: bool = guess();
        assert_eq!(
            p3,
            probe(&["--name", "a", "--name", "b"]).is_err(),
            "P3: Set rejects repeats"
        );

        // P4. The exit code clap reports for `--help`.
        let p4: i32 = guess();
        assert_eq!(
            p4,
            probe(&["--help"]).unwrap_err().exit_code(),
            "P4: help is not a failure"
        );
    }
}
