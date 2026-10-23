//! E07 capstone integration tests: the library API and the real `jobq` binary.
//! Run: cargo test --manifest-path ecosystem/Cargo.toml --test e07_capstone
//!
//! The tracing output is checked separately in `tests/e07_tracing.rs`.

use ecosystem::e07_capstone::*;
use ecosystem::guess;
use std::error::Error as _;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tokio::time::Instant;

const MS: Duration = Duration::from_millis(1);

fn write_config(name: &str, contents: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ecosystem-e07-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, contents).unwrap();
    path
}

fn job(id: &str, task: Task) -> Job {
    Job {
        id: id.to_owned(),
        task,
    }
}

fn sleep(id: &str, ms: u64) -> Job {
    job(id, Task::Sleep { ms })
}

fn config(workers: usize, job_timeout_ms: u64, jobs: Vec<Job>) -> Config {
    Config {
        workers,
        queue_capacity: 2,
        job_timeout_ms,
        jobs,
    }
}

fn statuses(report: &Report) -> Vec<(&str, &Outcome)> {
    report
        .jobs
        .iter()
        .map(|j| (j.id.as_str(), &j.outcome))
        .collect()
}

fn ok(output: &str) -> Outcome {
    Outcome::Succeeded {
        output: output.to_owned(),
    }
}

const MIXED: &str = r#"{
  "workers": 2,
  "jobs": [
    {"id": "a", "task": {"type": "sleep", "ms": 5}},
    {"id": "b", "task": {"type": "checksum", "data": "abc"}},
    {"id": "c", "task": {"type": "fail", "message": "boom"}}
  ]
}"#;

const GOOD: &str = r#"{"workers": 2, "jobs": [
  {"id": "a", "task": {"type": "sleep", "ms": 5}},
  {"id": "b", "task": {"type": "checksum", "data": "abc"}}
]}"#;

#[test]
fn a_config_defaults_and_tags() {
    let parsed =
        parse_config(r#"{"jobs": [{"id": "x", "task": {"type": "sleep", "ms": 3}}]}"#).unwrap();
    assert_eq!(
        parsed,
        Config {
            workers: 4,
            queue_capacity: 16,
            job_timeout_ms: 1000,
            jobs: vec![sleep("x", 3)],
        }
    );
    let parsed = parse_config(MIXED).unwrap();
    assert_eq!(parsed.workers, 2);
    assert_eq!(parsed.jobs[1].task, Task::Checksum { data: "abc".into() });
    assert_eq!(
        parsed.jobs[2].task,
        Task::Fail {
            message: "boom".into()
        }
    );
}

#[test]
fn a_config_rejects_unknown_fields_everywhere() {
    for text in [
        r#"{"worker": 2, "jobs": []}"#,
        r#"{"jobs": [{"id": "x", "name": "y", "task": {"type": "sleep", "ms": 1}}]}"#,
        r#"{"jobs": [{"id": "x", "task": {"type": "sleep", "ms": 1, "jitter": 2}}]}"#,
        r#"{"jobs": [{"id": "x", "task": {"type": "reboot"}}]}"#,
    ] {
        let err = parse_config(text).unwrap_err();
        assert!(matches!(err, ConfigError::Json(_)), "{text}: {err:?}");
        assert_eq!(err.to_string(), "invalid config JSON");
        assert!(err.source().unwrap().is::<serde_json::Error>());
    }
}

#[test]
fn b_validation_reports_the_first_broken_rule() {
    let reason = |text: &str| match parse_config(text) {
        Err(ConfigError::Invalid(reason)) => reason,
        other => panic!("expected Invalid for {text}, got {other:?}"),
    };
    assert_eq!(
        reason(r#"{"workers": 0, "queue_capacity": 0, "jobs": []}"#),
        "workers must be between 1 and 64"
    );
    assert_eq!(
        reason(r#"{"workers": 65, "jobs": []}"#),
        "workers must be between 1 and 64"
    );
    assert_eq!(
        reason(r#"{"queue_capacity": 0, "jobs": []}"#),
        "queue_capacity must be at least 1"
    );
    assert_eq!(
        reason(r#"{"job_timeout_ms": 0, "jobs": []}"#),
        "job_timeout_ms must be at least 1"
    );
    let jobs = r#"{"jobs": [
        {"id": "a", "task": {"type": "sleep", "ms": 1}},
        {"id": "b", "task": {"type": "sleep", "ms": 1}},
        {"id": "a", "task": {"type": "sleep", "ms": 1}},
        {"id": "b", "task": {"type": "sleep", "ms": 1}}]}"#;
    assert_eq!(reason(jobs), "duplicate job id `a`");
    assert_eq!(
        reason(r#"{"jobs": [{"id": "", "task": {"type": "fail", "message": ""}}]}"#),
        "job id must not be empty"
    );
    let err = ConfigError::Invalid("x".into());
    assert_eq!(err.to_string(), "invalid config: x");
    assert!(err.source().is_none());
    assert!(parse_config(r#"{"workers": 64, "jobs": []}"#).is_ok());
}

#[test]
fn b_load_config_keeps_sources() {
    let missing = std::env::temp_dir().join("ecosystem-e07-missing.json");
    let err = load_config(&missing).unwrap_err();
    assert_eq!(err.to_string(), "could not read config file");
    let io = err
        .source()
        .unwrap()
        .downcast_ref::<std::io::Error>()
        .unwrap();
    assert_eq!(io.kind(), std::io::ErrorKind::NotFound);

    let path = write_config("broken.json", "{");
    assert!(matches!(load_config(&path), Err(ConfigError::Json(_))));
    let path = write_config("good-load.json", GOOD);
    assert_eq!(load_config(&path).unwrap().jobs.len(), 2);
}

#[tokio::test(start_paused = true)]
async fn c_workers_share_one_queue() {
    let cfg = config(
        2,
        1000,
        vec![
            sleep("a", 30),
            sleep("b", 10),
            sleep("c", 20),
            sleep("d", 10),
        ],
    );
    let start = Instant::now();
    let report = run_jobs(&cfg).await;
    assert_eq!(start.elapsed(), MS * 40, "2 workers: a | b, c, d");
    let ids: Vec<&str> = report.jobs.iter().map(|j| j.id.as_str()).collect();
    assert_eq!(
        ids,
        ["a", "b", "c", "d"],
        "config order, not completion order"
    );
    let elapsed: Vec<u64> = report.jobs.iter().map(|j| j.elapsed_ms).collect();
    assert_eq!(elapsed, [30, 10, 20, 10]);
    assert_eq!(
        (report.succeeded, report.failed, report.timed_out),
        (4, 0, 0)
    );
    assert_eq!(report.jobs[0].outcome, ok("slept 30ms"));

    let start = Instant::now();
    run_jobs(&Config { workers: 1, ..cfg }).await;
    assert_eq!(start.elapsed(), MS * 70, "1 worker runs jobs one at a time");
}

#[tokio::test(start_paused = true)]
async fn c_outcomes_timeouts_and_counts() {
    let cfg = config(
        3,
        50,
        vec![
            sleep("slow", 100),
            job("sum", Task::Checksum { data: "abc".into() }),
            job(
                "bad",
                Task::Fail {
                    message: "boom".into(),
                },
            ),
            sleep("quick", 49),
        ],
    );
    let start = Instant::now();
    let report = run_jobs(&cfg).await;
    assert_eq!(
        start.elapsed(),
        MS * 50,
        "the timeout cuts the slow job short"
    );
    assert_eq!(
        statuses(&report),
        [
            ("slow", &Outcome::TimedOut),
            ("sum", &ok("checksum 294")),
            (
                "bad",
                &Outcome::Failed {
                    error: "boom".into()
                }
            ),
            ("quick", &ok("slept 49ms")),
        ]
    );
    assert_eq!(report.jobs[0].elapsed_ms, 50);
    assert_eq!(
        (report.succeeded, report.failed, report.timed_out),
        (2, 1, 1)
    );
}

#[tokio::test(start_paused = true)]
async fn c_many_jobs_through_a_tiny_queue() {
    let jobs = (0..100)
        .map(|i| sleep(&format!("j{i:03}"), i % 7))
        .collect();
    let cfg = Config {
        queue_capacity: 1,
        ..config(3, 1000, jobs)
    };
    let report = run_jobs(&cfg).await;
    assert_eq!(report.succeeded, 100);
    assert!(report.jobs.windows(2).all(|w| w[0].id < w[1].id));
    assert!(run_jobs(&config(2, 10, vec![])).await.jobs.is_empty());
}

fn cli(args: &[&str]) -> Cli {
    use clap::Parser;
    Cli::try_parse_from(std::iter::once("jobq").chain(args.iter().copied())).unwrap()
}

async fn execute_to_string(args: &[&str]) -> (anyhow::Result<i32>, String) {
    let mut out = Vec::new();
    let result = execute(cli(args), &mut out).await;
    (result, String::from_utf8(out).unwrap())
}

#[tokio::test(start_paused = true)]
async fn d_execute_check_and_text_report() {
    let path = write_config("mixed.json", MIXED);
    let path = path.to_str().unwrap();
    let (result, out) = execute_to_string(&["check", path]).await;
    assert_eq!(
        (result.unwrap(), out.as_str()),
        (0, "ok: 3 jobs, 2 workers\n")
    );

    let (result, out) = execute_to_string(&["run", path]).await;
    assert_eq!(result.unwrap(), 1, "a failed job makes the run fail");
    assert_eq!(
        out,
        "a succeeded: slept 5ms\nb succeeded: checksum 294\nc failed: boom\n2 succeeded, 1 failed, 0 timed out\n"
    );

    let good = write_config("good.json", GOOD);
    let (result, _) = execute_to_string(&["run", good.to_str().unwrap()]).await;
    assert_eq!(result.unwrap(), 0);
}

#[tokio::test(start_paused = true)]
async fn d_execute_json_report_and_worker_override() {
    let path = write_config(
        "serial.json",
        r#"{"workers": 4, "jobs": [
            {"id": "a", "task": {"type": "sleep", "ms": 10}},
            {"id": "b", "task": {"type": "sleep", "ms": 10}}]}"#,
    );
    let start = Instant::now();
    let (result, out) =
        execute_to_string(&["run", path.to_str().unwrap(), "--workers", "1", "--json"]).await;
    assert_eq!(result.unwrap(), 0);
    assert_eq!(start.elapsed(), MS * 20, "--workers 1 overrides the config");
    assert_eq!(out.lines().count(), 1, "one line of JSON");
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "jobs": [
                {"id": "a", "status": "succeeded", "output": "slept 10ms", "elapsed_ms": 10},
                {"id": "b", "status": "succeeded", "output": "slept 10ms", "elapsed_ms": 10}
            ],
            "succeeded": 2, "failed": 0, "timed_out": 0
        })
    );
}

#[tokio::test]
async fn d_errors_carry_context_and_exit_codes() {
    let missing = std::env::temp_dir().join("ecosystem-e07-nowhere.json");
    let (result, out) = execute_to_string(&["run", missing.to_str().unwrap()]).await;
    let err = result.unwrap_err();
    assert_eq!(out, "");
    let text = format!("{err:#}");
    assert!(
        text.starts_with(&format!(
            "loading {}: could not read config file: ",
            missing.display()
        )),
        "{text}"
    );
    assert_eq!(error_exit_code(&err), 66);

    let invalid = write_config("invalid.json", r#"{"workers": 0, "jobs": []}"#);
    let (result, _) = execute_to_string(&["check", invalid.to_str().unwrap()]).await;
    let err = result.unwrap_err();
    assert_eq!(error_exit_code(&err), 78);
    assert!(format!("{err:#}").ends_with("invalid config: workers must be between 1 and 64"));

    let other = anyhow::anyhow!("disk full").context("writing report");
    assert_eq!(error_exit_code(&other), 1);
    let denied = anyhow::Error::new(ConfigError::Io(std::io::ErrorKind::PermissionDenied.into()));
    assert_eq!(error_exit_code(&denied), 78);
}

fn jobq(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_jobq"))
        .args(args)
        .output()
        .expect("failed to start jobq");
    (
        output.status.code().expect("jobq was killed by a signal"),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

#[test]
fn e_binary_runs_end_to_end() {
    let good = write_config("bin-good.json", GOOD);
    let good = good.to_str().unwrap();
    let (code, out, err) = jobq(&["check", good]);
    assert_eq!(
        (code, out.as_str(), err.as_str()),
        (0, "ok: 2 jobs, 2 workers\n", "")
    );

    let (code, out, err) = jobq(&["run", good, "--json"]);
    assert_eq!(code, 0, "{err}");
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(json["succeeded"], 2);
    assert_eq!(err, "", "WARN is the default level: nothing to report");

    let mixed = write_config("bin-mixed.json", MIXED);
    let (code, _, err) = jobq(&["run", mixed.to_str().unwrap()]);
    assert_eq!(code, 1);
    assert!(
        err.contains("WARN job{id=c}"),
        "WARN events keep their span: {err}"
    );
    assert!(!err.contains("INFO"), "{err}");
    let (code, out, err) = jobq(&["-v", "run", mixed.to_str().unwrap()]);
    assert_eq!(code, 1);
    assert!(
        out.ends_with("2 succeeded, 1 failed, 0 timed out\n"),
        "{out}"
    );
    assert!(
        err.contains("INFO job{id=a}"),
        "-v enables INFO on stderr: {err}"
    );
    assert!(err.contains("job started"), "{err}");
    assert!(err.contains("WARN"), "{err}");
}

#[test]
fn e_binary_exit_codes() {
    let missing = std::env::temp_dir().join("ecosystem-e07-not-here.json");
    let (code, out, err) = jobq(&["check", missing.to_str().unwrap()]);
    assert_eq!((code, out.as_str()), (66, ""));
    assert!(err.starts_with("error: loading "), "{err}");
    assert!(err.contains("could not read config file: "), "{err}");

    let invalid = write_config("bin-invalid.json", r#"{"jobs": [], "extra": 1}"#);
    let (code, _, err) = jobq(&["run", invalid.to_str().unwrap()]);
    assert_eq!(code, 78);
    assert!(
        err.contains("invalid config JSON: unknown field `extra`"),
        "{err}"
    );

    let (code, out, err) = jobq(&["run", "x.json", "--workers", "0"]);
    assert_eq!((code, out.as_str()), (2, ""));
    assert!(err.contains("--workers"), "{err}");

    let (code, out, _) = jobq(&["--version"]);
    assert_eq!((code, out.as_str()), (0, "jobq 0.0.0\n"));
}

/// P. Predict BEFORE running, from the contracts in `src/e07_capstone.rs`:
/// replace each `guess()` with your answer. Then implement and check.
#[tokio::test(start_paused = true)]
async fn p_predictions() {
    // P1. 3 workers, 50ms timeout. Jobs sleep 40, 60, 10, 30, 20 ms (config order).
    // How long does the whole run take?
    let cfg = config(
        3,
        50,
        vec![
            sleep("a", 40),
            sleep("b", 60),
            sleep("c", 10),
            sleep("d", 30),
            sleep("e", 20),
        ],
    );
    let start = Instant::now();
    let report = run_jobs(&cfg).await;
    let p1: Duration = guess();
    assert_eq!(
        p1,
        start.elapsed(),
        "P1: workers pick the next job as soon as they are free"
    );

    // P2. ...and how many jobs time out?
    let p2: usize = guess();
    assert_eq!(
        p2, report.timed_out,
        "P2: the timeout is per job, not per run"
    );

    // P3. Exit code of `jobq run` with no config path?
    let p3: i32 = guess();
    assert_eq!(
        p3,
        jobq(&["run"]).0,
        "P3: clap usage errors exit before your code runs"
    );
}
