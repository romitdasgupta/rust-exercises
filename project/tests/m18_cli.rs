//! MILESTONE m18 (part 2) — The `loglens` command-line program (Chapter 18)
//! Run: cargo test --test m18_cli
//! Try it: cargo run -- report fixtures/app.log   (from the project directory)
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: what belongs on stdout, what on stderr,
//! and what in the exit status? Why does `main.rs` do almost nothing?
//! IMPLEMENT `src/cli.rs`: `parse_args`, `run`, `main_with`; `src/main.rs`: `main`.
//! EXPLAIN afterward: why the program takes its streams as `&mut dyn Write` parameters,
//! and why file names are `OsString`/`PathBuf` and not `String`.

mod common;

use common::{TempDir, guess};
use loglens::cli::{Command, Input, Options, USAGE, main_with, parse_args, run};
use loglens::{Error, Group};
use std::io::{self, BufRead, BufReader, Cursor, Write};
use std::path::PathBuf;
use std::process::{Command as Process, Stdio};
use std::sync::mpsc;
use std::time::Duration;

const APP_WARNINGS: &str = concat!(
    "2026-10-23T17:03:02Z WARN auth user=bob latency_ms=310 msg=\"slow password check\"\n",
    "2026-10-23T17:03:05Z ERROR auth user=bob latency_ms=97 msg=\"login failed: \\\"bad password\\\"\"\n",
    "2026-10-23T17:03:21Z ERROR db latency_ms=5012 msg=\"connection reset\"\n",
);

const APP_REPORT: &str = "records: 8
levels: TRACE=0 DEBUG=1 INFO=4 WARN=1 ERROR=2
top 10 by target:
  auth    count=4 errors=1 mean_latency_ms=122.0 max_latency_ms=310
  db      count=2 errors=1 mean_latency_ms=2508.0 max_latency_ms=5012
  billing count=1 errors=0 mean_latency_ms=- max_latency_ms=-
  cache   count=1 errors=0 mean_latency_ms=- max_latency_ms=-
";

fn opts(command: Command) -> Options {
    Options {
        command,
        ..Options::default()
    }
}

fn path(p: &str) -> Input {
    Input::Path(PathBuf::from(p))
}

#[test]
fn a_parse_args() {
    assert_eq!(
        parse_args(["grep", "--where", "level>=warn", "fixtures/app.log"]).unwrap(),
        Options {
            filter: "level>=warn".into(),
            inputs: vec![path("fixtures/app.log")],
            ..opts(Command::Grep)
        }
    );
    assert_eq!(
        parse_args(["count"]).unwrap(),
        Options {
            inputs: vec![Input::Stdin],
            ..opts(Command::Count)
        }
    );
    assert_eq!(
        parse_args([
            "report", "-b", "user", "a.log", "-n", "3", "-", "--jobs=4", "b.log"
        ])
        .unwrap(),
        Options {
            group: Group::Field("user".into()),
            top: 3,
            jobs: 4,
            inputs: vec![path("a.log"), Input::Stdin, path("b.log")],
            ..opts(Command::Report)
        }
    );
    assert_eq!(
        parse_args([
            "grep",
            "-w",
            "user=ada",
            "--where=level>=info",
            "--by=level"
        ])
        .unwrap(),
        Options {
            filter: "user=ada level>=info".into(),
            group: Group::Level,
            inputs: vec![Input::Stdin],
            ..opts(Command::Grep)
        }
    );
    assert_eq!(
        parse_args(["grep", "--width", "20", "--strict", "-f", "app.log"]).unwrap(),
        Options {
            width: Some(20),
            strict: true,
            follow: true,
            inputs: vec![path("app.log")],
            ..opts(Command::Grep)
        }
    );
    assert_eq!(
        parse_args(["grep", "--", "-odd.log", "--help", "-"])
            .unwrap()
            .inputs,
        [path("-odd.log"), path("--help"), Input::Stdin]
    );
}

#[test]
fn a_parse_args_help() {
    for args in [
        &["--help"][..],
        &["-h"],
        &["help"],
        &["grep", "x.log", "-h"],
        &["frob", "--help"],
    ] {
        assert_eq!(
            parse_args(args).unwrap(),
            Options::default(),
            "args {args:?}"
        );
    }
}

fn usage_error(args: &[&str]) -> String {
    match parse_args(args) {
        Err(Error::Usage(message)) => message,
        other => panic!("expected a usage error for {args:?}, got {other:?}"),
    }
}

#[test]
fn a_parse_args_errors() {
    let cases: [(&[&str], &str); 13] = [
        (&[], "missing command"),
        (&["frob"], "unknown command 'frob'"),
        (&["grep", "-x"], "unknown option '-x'"),
        (&["grep", "-fn"], "unknown option '-fn'"),
        (&["grep", "--colour"], "unknown option '--colour'"),
        (&["grep", "--top"], "option '--top' needs a value"),
        (&["grep", "-n"], "option '-n' needs a value"),
        (
            &["report", "--top", "ten"],
            "invalid number 'ten' for '--top'",
        ),
        (&["report", "--top=-1"], "invalid number '-1' for '--top'"),
        (
            &["report", "-j", "99999999999999999999"],
            "invalid number '99999999999999999999' for '-j'",
        ),
        (
            &["count", "--follow", "a.log"],
            "--follow works only with grep",
        ),
        (&["grep", "-f"], "--follow needs exactly one FILE"),
        (
            &["grep", "-f", "a.log", "-"],
            "--follow needs exactly one FILE",
        ),
    ];
    for (args, message) in cases {
        assert_eq!(usage_error(args), message, "args {args:?}");
    }
}

#[cfg(unix)]
#[test]
fn a_file_names_need_not_be_utf8() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    let name = OsString::from_vec(b"caf\xe9.log".to_vec());
    let parsed = parse_args([OsString::from("count"), name.clone()]).unwrap();
    assert_eq!(parsed.inputs, [Input::Path(PathBuf::from(name.clone()))]);
    let err = parse_args([OsString::from("count"), OsString::from("-w"), name]).unwrap_err();
    assert!(matches!(err, Error::Usage(m) if m == "arguments must be valid UTF-8"));
}

/// Runs `run` in memory: (status, stdout, stderr).
fn run_in_memory(options: &Options, stdin: &str) -> (u8, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run(options, &mut Cursor::new(stdin), &mut out, &mut err);
    (
        status,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

fn app_log() -> String {
    common::fixture_text("app.log")
}

#[test]
fn b_run_in_memory() {
    let grep = Options {
        filter: "level>=warn".into(),
        inputs: vec![Input::Stdin],
        ..opts(Command::Grep)
    };
    assert_eq!(
        run_in_memory(&grep, &app_log()),
        (
            0,
            APP_WARNINGS.to_string(),
            "loglens: <stdin>: line 9, column 1: invalid timestamp\n".to_string()
        )
    );

    let count_none = Options {
        filter: "user=nobody".into(),
        ..grep.clone()
    };
    let count_none = Options {
        command: Command::Count,
        ..count_none
    };
    assert_eq!(
        run_in_memory(&count_none, &app_log()).0,
        1,
        "nothing matched"
    );
    assert_eq!(run_in_memory(&count_none, &app_log()).1, "0\n");

    let strict = Options {
        strict: true,
        ..grep.clone()
    };
    assert_eq!(
        run_in_memory(&strict, &app_log()).0,
        2,
        "malformed line under --strict"
    );

    let report = Options {
        filter: String::new(),
        ..opts(Command::Report)
    };
    let report = Options {
        inputs: vec![Input::Stdin],
        ..report
    };
    assert_eq!(run_in_memory(&report, &app_log()).1, APP_REPORT);

    assert_eq!(
        run_in_memory(&Options::default(), ""),
        (0, USAGE.to_string(), String::new())
    );
}

#[test]
fn b_invalid_filter() {
    let options = Options {
        filter: "level>=loud".into(),
        inputs: vec![Input::Stdin],
        ..opts(Command::Count)
    };
    let (status, out, err) = run_in_memory(&options, &app_log());
    assert_eq!((status, out.as_str()), (2, ""));
    assert_eq!(
        err,
        "loglens: invalid filter: term \"level>=loud\": unknown level\n"
    );
}

/// A stdout that fails like a closed pipe, or with another error.
struct BadOut(io::ErrorKind);

impl Write for BadOut {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(self.0, "write failed"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(self.0, "write failed"))
    }
}

#[test]
fn b_write_errors() {
    let grep = Options {
        inputs: vec![Input::Stdin],
        ..opts(Command::Grep)
    };
    let mut err = Vec::new();
    let status = run(
        &grep,
        &mut Cursor::new(app_log()),
        &mut BadOut(io::ErrorKind::BrokenPipe),
        &mut err,
    );
    assert_eq!(status, 0, "a closed pipe is not an error");
    assert!(!String::from_utf8(err).unwrap().contains("write failed"));

    let mut err = Vec::new();
    let status = run(
        &grep,
        &mut Cursor::new(app_log()),
        &mut BadOut(io::ErrorKind::Other),
        &mut err,
    );
    assert_eq!(status, 2);
    assert!(
        String::from_utf8(err)
            .unwrap()
            .ends_with("loglens: I/O error: write failed\n")
    );
}

#[test]
fn c_main_with_reports_argument_errors() {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = main_with(["frob"], &mut Cursor::new(""), &mut out, &mut err);
    assert_eq!(status, 2);
    assert!(out.is_empty());
    assert_eq!(
        String::from_utf8(err).unwrap(),
        "loglens: unknown command 'frob'\nusage: loglens <grep|count|report> [OPTIONS] [FILE...]\n"
    );
}

/// Runs the built binary from the project directory: (status, stdout, stderr).
fn loglens(args: &[&str], stdin: &str) -> (i32, String, String) {
    let mut child = Process::new(env!("CARGO_BIN_EXE_loglens"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run loglens");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.code().expect("exited normally"),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

#[test]
fn d_binary_grep_count_report() {
    let (status, out, err) = loglens(&["grep", "--where", "level>=warn", "fixtures/app.log"], "");
    assert_eq!(status, 0);
    assert_eq!(out, APP_WARNINGS);
    assert_eq!(
        err,
        "loglens: fixtures/app.log: line 9, column 1: invalid timestamp\n"
    );

    assert_eq!(
        loglens(&["count", "-w", "user=ada", "fixtures/app.log"], "").1,
        "2\n"
    );
    assert_eq!(loglens(&["report", "fixtures/app.log"], "").1, APP_REPORT);

    let (status, out, _) = loglens(&["count", "-w", "user=nobody", "fixtures/app.log"], "");
    assert_eq!((status, out.as_str()), (1, "0\n"));
}

#[test]
fn d_binary_reads_stdin() {
    let (status, out, err) = loglens(&["grep", "-w", "target=db", "-"], &app_log());
    assert_eq!(status, 0);
    assert_eq!(
        out,
        "2026-10-23T17:03:09Z INFO db query=\"select 1\" latency_ms=4\n\
         2026-10-23T17:03:21Z ERROR db latency_ms=5012 msg=\"connection reset\"\n"
    );
    assert_eq!(
        err,
        "loglens: <stdin>: line 9, column 1: invalid timestamp\n"
    );
    assert_eq!(
        loglens(&["count"], &app_log()).1,
        "8\n",
        "no FILE means stdin"
    );
}

#[test]
fn d_binary_errors_and_exit_status() {
    let (status, out, err) = loglens(&["frobnicate"], "");
    assert_eq!((status, out.as_str()), (2, ""));
    assert!(err.starts_with("loglens: unknown command 'frobnicate'\nusage: "));

    let (status, out, err) = loglens(&["count", "fixtures/missing.log", "fixtures/app.log"], "");
    assert_eq!(status, 2, "a missing file is an error");
    assert_eq!(out, "8\n", "the other file was still counted");
    assert!(
        err.starts_with("loglens: fixtures/missing.log: "),
        "stderr: {err}"
    );

    assert_eq!(loglens(&["count", "--strict", "fixtures/app.log"], "").0, 2);
    assert_eq!(loglens(&["count", "--strict", "fixtures/web.log"], "").0, 0);
    let (status, _, err) = loglens(&["count", "-w", "level~x", "fixtures/app.log"], "");
    assert_eq!(status, 2);
    assert_eq!(
        err,
        "loglens: invalid filter: term \"level~x\": unsupported operator\n"
    );

    let (status, out, _) = loglens(&["--help"], "");
    assert_eq!((status, out.as_str()), (0, USAGE));
}

#[test]
fn d_binary_handles_invalid_utf8_and_width() {
    let (status, out, err) = loglens(&["grep", "fixtures/bad_utf8.log"], "");
    assert_eq!(status, 0);
    assert_eq!(out.lines().count(), 2);
    assert_eq!(
        err,
        "loglens: fixtures/bad_utf8.log: line 2, column 51: line is not valid UTF-8\n"
    );

    let (_, out, _) = loglens(
        &[
            "grep",
            "--width",
            "40",
            "-w",
            "target=auth",
            "fixtures/app.log",
        ],
        "",
    );
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0], "2026-10-23T17:03:00Z INFO auth user=ada…");
    assert!(lines.iter().all(|l| l.chars().count() == 40));
}

#[test]
fn d_binary_follows_a_growing_file() {
    let dir = TempDir::new("cli-follow");
    let file = dir.file("live.log", "2026-10-23T17:03:00Z WARN auth first=1\n");
    let mut child = Process::new(env!("CARGO_BIN_EXE_loglens"))
        .args(["grep", "--follow", "-w", "level>=warn"])
        .arg(&file)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("run loglens");
    let stdout = child.stdout.take().unwrap();
    let (tx, rx) = mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            if tx.send(line.unwrap()).is_err() {
                break;
            }
        }
    });
    let next = || {
        rx.recv_timeout(Duration::from_secs(10))
            .expect("a followed line")
    };
    let first = next();

    let mut log = std::fs::OpenOptions::new()
        .append(true)
        .open(&file)
        .unwrap();
    writeln!(log, "2026-10-23T17:03:01Z INFO auth ignored=1").unwrap();
    writeln!(log, "2026-10-23T17:03:02Z ERROR auth second=1").unwrap();
    log.flush().unwrap();
    let second = next();

    child.kill().unwrap();
    child.wait().unwrap();
    reader.join().unwrap();
    assert_eq!(first, "2026-10-23T17:03:00Z WARN auth first=1");
    assert_eq!(second, "2026-10-23T17:03:02Z ERROR auth second=1");
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
#[allow(clippy::join_absolute_paths)] // P3 does it on purpose.
fn p_predictions() {
    // P1. The file stem of a name with two extensions.
    let p1: &str = guess();
    assert_eq!(
        Some(std::ffi::OsStr::new(p1)),
        std::path::Path::new("logs/archive.tar.gz").file_stem(),
        "P1: only the last extension is removed"
    );

    // P2. After writing 5 bytes through a BufWriter, how many reached the Vec?
    let mut buffered = io::BufWriter::new(Vec::new());
    buffered.write_all(b"hello").unwrap();
    let p2: usize = guess();
    assert_eq!(
        p2,
        buffered.get_ref().len(),
        "P2: BufWriter holds data until flush"
    );

    // P3. Joining an absolute path onto another path.
    let p3: &str = guess();
    assert_eq!(
        PathBuf::from(p3),
        std::path::Path::new("/var/log").join("/etc/passwd"),
        "P3: an absolute path replaces the base: never join untrusted names"
    );
}
