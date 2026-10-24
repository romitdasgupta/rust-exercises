//! Milestone m18 (Chapter 18), refactored in m19: the command-line program.
//!
//! `src/main.rs` only connects the real process (arguments, stdin, stdout,
//! stderr, exit status) to [`main_with`]. Everything else lives here and
//! takes its streams as parameters, so tests can run the whole program on
//! in-memory buffers as well as through the built binary.

use crate::error::Error;
use crate::report::Group;
use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::path::PathBuf;

/// The help text, printed by `loglens --help`. Its first line is the usage
/// line printed after argument errors.
pub const USAGE: &str = "\
usage: loglens <grep|count|report> [OPTIONS] [FILE...]

Commands:
  grep     print matching records, one canonical line each
  count    print the number of matching records
  report   print a summary of matching records

Options:
  -w, --where EXPR  keep records matching EXPR (repeatable; terms are ANDed)
  -b, --by KEY      report groups: target, level, or a field name [default: target]
  -n, --top N       report rows [default: 10]
  -j, --jobs N      worker threads for several files; 0 = one per core [default: 1]
      --width N     grep: cut each output line to N characters
  -f, --follow      grep: keep reading one FILE as it grows
      --strict      exit with status 2 if any line is malformed
  -h, --help        print this help

FILE '-', or no FILE, reads standard input.
Exit status: 0 if a record matched, 1 if none matched, 2 on error.
";

/// What the program was asked to do.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Command {
    /// Print [`USAGE`].
    #[default]
    Help,
    /// Print matching records.
    Grep,
    /// Print the number of matching records.
    Count,
    /// Print a [`crate::report::render`] report.
    Report,
}

/// One input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    /// Standard input (`-`).
    Stdin,
    /// A file.
    Path(PathBuf),
}

/// Parsed command-line options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// The command.
    pub command: Command,
    /// The filter expression: every `--where` value, joined by one space.
    pub filter: String,
    /// `--by`.
    pub group: Group,
    /// `--top`.
    pub top: usize,
    /// `--jobs`; 0 means one per core.
    pub jobs: usize,
    /// `--width`.
    pub width: Option<usize>,
    /// `--follow`.
    pub follow: bool,
    /// `--strict`.
    pub strict: bool,
    /// The inputs, in order; `[Input::Stdin]` when no FILE was given.
    pub inputs: Vec<Input>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            command: Command::Help,
            filter: String::new(),
            group: Group::Target,
            top: 10,
            jobs: 1,
            width: None,
            follow: false,
            strict: false,
            inputs: Vec::new(),
        }
    }
}

/// m18 D. Parse the arguments (without the program name), by hand.
///
/// - An argument `-h` or `--help` before any `--` returns
///   `Options::default()` (whose command is `Help`), ignoring everything
///   else, even an unknown command.
/// - The first argument is the command: `grep`, `count`, `report`, or
///   `help` (which means `Command::Help`).
/// - Options may appear anywhere after the command. A value is the next
///   argument (`--top 5`, `-n 5`) or, for long names only, follows `=`
///   (`--top=5`). Short options are never combined (`-fn` is unknown).
/// - `--` ends the options: every later argument is a FILE. `-` is
///   [`Input::Stdin`]; any other argument that starts with `-` is an option.
/// - Numbers use [`crate::num::parse_u64`] with `Overflow::Reject`, then
///   `usize::try_from`.
/// - Option names, values, and the command must be UTF-8. FILE arguments
///   need not be: keep them as `OsString`/`PathBuf`.
///
/// Every problem is an [`Error::Usage`] with one of these messages:
///
/// | Problem | Message |
/// | --- | --- |
/// | no arguments | `missing command` |
/// | first argument is not a command | `unknown command 'X'` |
/// | an unknown option | `unknown option 'X'` |
/// | the value is missing | `option 'X' needs a value` (X as typed: `-n` or `--top`) |
/// | a number is invalid | `invalid number 'V' for 'X'` |
/// | non-UTF-8 command, option, or value | `arguments must be valid UTF-8` |
/// | `--follow` with another command than grep (checked first) | `--follow works only with grep` |
/// | `--follow` without exactly one FILE (not `-`) | `--follow needs exactly one FILE` |
pub fn parse_args<I, S>(args: I) -> Result<Options, Error>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    todo!("m18 D: hand-written argument parsing")
}

/// m18 E. Run parsed options; return the exit status.
///
/// 1. `Command::Help`: write [`USAGE`] to `stdout`, return 0.
/// 2. Compile the filter. If invalid, write `loglens: {error}` (the
///    [`Error::Filter`] message) and a newline to `stderr`, and return 2.
/// 3. `--follow`: follow the one file with [`crate::follow::Follower`],
///    printing matching lines; flush `stdout` after each poll and sleep
///    about 100 ms when a poll found nothing. This never returns unless an
///    error happens.
/// 4. Otherwise process each input in order with
///    [`crate::pipeline::process_reader`]:
///    - `grep` writes each matching record's canonical line to `stdout`,
///      cut to `--width` characters with [`crate::text::truncate`] if given;
///    - `count` and `report` aggregate across all inputs and, after the last
///      input, write `{matched}\n` or [`crate::report::render`] output.
///    - A file that cannot be opened: write `loglens: {error}` for the
///      [`Error::Open`] and a newline to `stderr`, and go on with the next.
///    - A malformed line: write `loglens: {name}: {parse error}` and a newline
///      to `stderr`, where name is the path as given (`path.display()`) or
///      `<stdin>`.
///    - A read error in the middle of an input: write `loglens: {name}:
///      {error}` and a newline, count it like a file that could not be
///      opened, and go on with the next input.
/// 5. Flush `stdout`. The status is 2 if a file could not be opened or if
///    `--strict` is set and any line was malformed; otherwise 0 if at least
///    one record matched and 1 if none did.
///
/// Writing to `stdout` can fail. If the reader went away (`BrokenPipe`, as
/// in `loglens grep app.log | head -1`), stop quietly and return 0. For any
/// other write error, report `loglens: {Error::Io}` and return 2.
///
/// m19 refactor: when `jobs != 1` and there are several inputs, none of them
/// stdin, process the files with [`crate::parallel::map_ordered`]. Buffer
/// each file's output and messages, then write them in input order, so the
/// output is byte-for-byte the same as with `--jobs 1`.
pub fn run(
    opts: &Options,
    stdin: &mut dyn BufRead,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> u8 {
    todo!("m18 E: the program")
}

/// m18 F. The whole program: parse `args` and run them.
///
/// On an argument error, write `loglens: {error}`, a newline, and the first
/// line of [`USAGE`] (with its newline) to `stderr`, and return 2.
pub fn main_with<I, S>(
    args: I,
    stdin: &mut dyn BufRead,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> u8
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    todo!("m18 F: parse, then run")
}
