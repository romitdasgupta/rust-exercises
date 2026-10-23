//! The practice coach: compiler drills, progress, and spaced review.
//!
//! Use the Cargo aliases defined in `.cargo/config.toml`:
//!
//!   cargo drill                 summary of every drill chapter
//!   cargo drill ch04            work through chapter 4's drills
//!   cargo drill reset ch04      restore chapter 4's drills for another round
//!   cargo progress              lesson status, next step, reviews due
//!   cargo progress reviewed ch04    record that you reviewed chapter 4
//!   cargo progress redo ch04 l08    archive your lesson and restore its starter
//!
//! State lives in `.progress/` (ignored by Git). Delete it to start over.

use std::collections::BTreeMap;
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

const STATE: &str = ".progress";
const REVIEW_DAYS: [u64; 5] = [1, 3, 7, 21, 60];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match args.as_slice() {
        ["drill"] => drill_summary(),
        ["drill", "reset", chapter] => chapter_number(chapter).and_then(drill_reset),
        ["drill", target] => drill_target(target),
        ["progress"] => progress(),
        ["progress", "reviewed", chapter] => chapter_number(chapter).and_then(reviewed),
        ["progress", "redo", chapter, lesson] => {
            chapter_number(chapter).and_then(|c| redo(c, lesson))
        }
        _ => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

const USAGE: &str = "usage:
  cargo drill [chNN | path/to/drill.rs | reset chNN]
  cargo progress [reviewed chNN | redo chNN lNN]";

// ---------------------------------------------------------------- drills

/// What actually happened when a drill was compiled and run.
#[derive(Clone, PartialEq, Debug)]
enum Outcome {
    Ok,
    Panic,
    /// Error codes in order of appearance. Errors without a code are not listed.
    Errors(Vec<String>),
}

impl Outcome {
    fn label(&self) -> String {
        match self {
            Outcome::Ok => "ok".into(),
            Outcome::Panic => "panic".into(),
            Outcome::Errors(codes) if codes.is_empty() => "error (no error code)".into(),
            Outcome::Errors(codes) => codes.join(" "),
        }
    }
}

struct Run {
    outcome: Outcome,
    /// Compiler diagnostics or program output, ready to print.
    output: String,
}

#[derive(PartialEq)]
enum Verdict {
    Right,
    Partial,
    Wrong,
}

/// Compares an `EXPECT:` line with what happened.
fn judge(expect: &str, actual: &Outcome) -> Verdict {
    let words: Vec<String> = expect
        .split([' ', ','])
        .filter(|w| !w.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    let predicted_codes: Vec<String> = words
        .iter()
        .filter(|w| w.starts_with('e') && w.len() == 5)
        .map(|w| w.to_ascii_uppercase())
        .collect();
    match actual {
        Outcome::Ok if words == ["ok"] => Verdict::Right,
        Outcome::Panic if words == ["panic"] => Verdict::Right,
        Outcome::Errors(codes) => {
            let mut actual_codes = codes.clone();
            actual_codes.sort();
            actual_codes.dedup();
            let mut predicted = predicted_codes.clone();
            predicted.sort();
            predicted.dedup();
            let exact = !predicted.is_empty() && predicted == actual_codes;
            if exact || (words == ["error"] && actual_codes.is_empty()) {
                Verdict::Right
            } else if words == ["error"] || predicted.iter().any(|code| actual_codes.contains(code))
            {
                Verdict::Partial
            } else {
                Verdict::Wrong
            }
        }
        _ => Verdict::Wrong,
    }
}

fn expect_line(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        let rest = line
            .trim_start_matches(['/', '!', ' '])
            .strip_prefix("EXPECT:")?;
        let value = rest.trim();
        (!value.is_empty() && value != "?").then(|| value.to_string())
    })
}

fn keep_lines(source: &str) -> Vec<String> {
    source
        .lines()
        .filter(|line| line.trim_end().ends_with("// KEEP"))
        .map(|line| line.trim().to_string())
        .collect()
}

fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn compile_and_run(path: &Path) -> Result<Run, String> {
    let out_dir = std::env::temp_dir().join("rust-exercises-drills");
    fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = path.parent().and_then(Path::file_name).unwrap_or_default();
    let exe = out_dir.join(format!(
        "{}_{stem}{}",
        parent.to_string_lossy(),
        std::env::consts::EXE_SUFFIX
    ));
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let color = if std::io::stdout().is_terminal() {
        "always"
    } else {
        "never"
    };
    let compiled = Command::new(rustc)
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "bin",
            "-C",
            "debuginfo=0",
        ])
        .args([
            "-A",
            "dead_code",
            "-A",
            "unused_variables",
            "-A",
            "unused_assignments",
        ])
        .args(["-A", "unused_mut", "-A", "unused_imports", "--color", color])
        .arg("-o")
        .arg(&exe)
        .arg(path)
        .output()
        .map_err(|e| format!("could not start rustc: {e}"))?;
    let diagnostics = String::from_utf8_lossy(&compiled.stderr).into_owned();
    if !compiled.status.success() {
        let codes = strip_ansi(&diagnostics)
            .lines()
            .filter_map(|line| line.strip_prefix("error[")?.split(']').next())
            .map(str::to_string)
            .collect();
        return Ok(Run {
            outcome: Outcome::Errors(codes),
            output: diagnostics,
        });
    }
    let ran = Command::new(&exe)
        .output()
        .map_err(|e| format!("could not run drill: {e}"))?;
    let mut output = String::from_utf8_lossy(&ran.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&ran.stderr));
    let outcome = if ran.status.success() {
        Outcome::Ok
    } else {
        Outcome::Panic
    };
    Ok(Run { outcome, output })
}

/// A recorded first prediction for one drill.
struct Record {
    expect: String,
    actual: String,
    verdict: String,
}

fn records_path() -> PathBuf {
    Path::new(STATE).join("drills.tsv")
}

fn load_records() -> BTreeMap<String, Record> {
    let text = fs::read_to_string(records_path()).unwrap_or_default();
    text.lines()
        .filter_map(|line| {
            let mut f = line.split('\t');
            let key = f.next()?.to_string();
            let expect = f.next()?.to_string();
            let actual = f.next()?.to_string();
            let verdict = f.next()?.to_string();
            Some((
                key,
                Record {
                    expect,
                    actual,
                    verdict,
                },
            ))
        })
        .collect()
}

fn save_records(records: &BTreeMap<String, Record>) -> Result<(), String> {
    fs::create_dir_all(STATE).map_err(|e| e.to_string())?;
    let text: String = records
        .iter()
        .map(|(k, r)| format!("{k}\t{}\t{}\t{}\n", r.expect, r.actual, r.verdict))
        .collect();
    fs::write(records_path(), text).map_err(|e| e.to_string())
}

fn original_path(drill: &Path) -> PathBuf {
    Path::new(STATE).join("originals").join(drill)
}

fn drill_files(chapter: u32) -> Vec<PathBuf> {
    let dir = Path::new("drills").join(format!("ch{chapter:02}"));
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        .collect();
    files.sort();
    files
}

fn key(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[derive(PartialEq)]
enum DrillState {
    NeedsPrediction,
    NeedsFix,
    Done,
}

/// Advances one drill as far as it can go and prints what the learner should
/// do next. `verbose` prints compiler output for unfinished drills.
fn step_drill(path: &Path, verbose: bool) -> Result<DrillState, String> {
    let source = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let name = key(path);
    let Some(expect) = expect_line(&source) else {
        println!("  ·  {name}: write your prediction on the EXPECT: line first");
        return Ok(DrillState::NeedsPrediction);
    };
    let mut records = load_records();
    let just_judged = !records.contains_key(&name);
    if just_judged {
        // First run after predicting: judge the untouched drill.
        let run = compile_and_run(path)?;
        let verdict = judge(&expect, &run.outcome);
        let word = match verdict {
            Verdict::Right => "right",
            Verdict::Partial => "partly right",
            Verdict::Wrong => "wrong",
        };
        let original = original_path(path);
        if let Some(dir) = original.parent() {
            fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        fs::write(&original, &source).map_err(|e| e.to_string())?;
        println!(
            "\n━━ {name}\n   You predicted: {expect}\n   Actual:        {}\n   Verdict:       {word}",
            run.outcome.label()
        );
        print!("{}", run.output);
        if verdict != Verdict::Right {
            println!(
                "   Before fixing: explain in one sentence why the compiler or program behaved this way."
            );
        }
        records.insert(
            name.clone(),
            Record {
                expect,
                actual: run.outcome.label(),
                verdict: word.into(),
            },
        );
        save_records(&records)?;
    }
    let original = fs::read_to_string(original_path(path)).unwrap_or_else(|_| source.clone());
    let missing: Vec<String> = keep_lines(&original)
        .into_iter()
        .filter(|line| !source.lines().any(|l| l.trim() == line))
        .collect();
    if !missing.is_empty() {
        println!("  ✗  {name}: restore these lines; they must stay in the program:");
        for line in missing {
            println!("       {line}");
        }
        return Ok(DrillState::NeedsFix);
    }
    let run = compile_and_run(path)?;
    if run.outcome == Outcome::Ok {
        println!("  ✓  {name}");
        return Ok(DrillState::Done);
    }
    println!(
        "  ✗  {name}: {} — fix it so it compiles and runs cleanly",
        run.outcome.label()
    );
    if verbose && !just_judged {
        print!("{}", run.output);
    }
    Ok(DrillState::NeedsFix)
}

fn drill_target(target: &str) -> Result<(), String> {
    if target.ends_with(".rs") {
        step_drill(Path::new(target), true)?;
        return Ok(());
    }
    let chapter = chapter_number(target)?;
    let files = drill_files(chapter);
    if files.is_empty() {
        return Err(format!("no drills in drills/ch{chapter:02}"));
    }
    println!("Chapter {chapter:02} drills (stops at the first unfinished one)");
    for file in &files {
        if step_drill(file, true)? != DrillState::Done {
            println!("\nEdit {} and run `cargo drill {target}` again.", key(file));
            return Ok(());
        }
    }
    println!(
        "\nAll chapter {chapter:02} drills done. Retry them later with `cargo drill reset ch{chapter:02}`."
    );
    Ok(())
}

fn drill_summary() -> Result<(), String> {
    let records = load_records();
    println!("Drills: predicted / total, first-try accuracy");
    for chapter in 3..=24 {
        let files = drill_files(chapter);
        if files.is_empty() {
            continue;
        }
        let mine: Vec<&Record> = files.iter().filter_map(|f| records.get(&key(f))).collect();
        let right = mine.iter().filter(|r| r.verdict == "right").count();
        println!(
            "  ch{chapter:02}  {:>2}/{:<2}  {right} right",
            mine.len(),
            files.len()
        );
    }
    println!("Run `cargo drill chNN` to continue a chapter.");
    Ok(())
}

fn drill_reset(chapter: u32) -> Result<(), String> {
    let mut records = load_records();
    let mut restored = 0;
    for file in drill_files(chapter) {
        if let Ok(original) = fs::read_to_string(original_path(&file)) {
            let blank: String = original
                .lines()
                .map(|line| match line.find("EXPECT:") {
                    Some(i) => format!("{}EXPECT: ?\n", &line[..i]),
                    None => format!("{line}\n"),
                })
                .collect();
            fs::write(&file, blank).map_err(|e| e.to_string())?;
            restored += 1;
        }
        records.remove(&key(&file));
    }
    save_records(&records)?;
    println!(
        "Restored {restored} drills in ch{chapter:02}. Predict them again without looking at your notes."
    );
    Ok(())
}

// -------------------------------------------------------------- progress

#[derive(Default)]
struct Lesson {
    passed: usize,
    failed: usize,
    file: Option<PathBuf>,
}

fn lesson_file(chapter: u32, module: &str) -> Option<PathBuf> {
    let path = Path::new("exercises")
        .join(format!("chapter{chapter:02}"))
        .join(format!("{module}.rs"));
    path.exists().then_some(path)
}

/// Runs one chapter's tests and groups the results by lesson module.
fn chapter_status(chapter: u32) -> Result<BTreeMap<String, Lesson>, String> {
    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args([
            "test",
            "--test",
            &format!("chapter{chapter:02}"),
            "--no-fail-fast",
            "--",
        ])
        .args(["--format", "pretty", "--color", "never"])
        .output()
        .map_err(|e| format!("could not run cargo test: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.contains("running ") {
        return Err(format!(
            "chapter {chapter:02} does not compile:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let mut lessons: BTreeMap<String, Lesson> = BTreeMap::new();
    for line in stdout.lines() {
        let Some(rest) = line.strip_prefix("test ") else {
            continue;
        };
        let Some((name, result)) = rest.rsplit_once(" ... ") else {
            continue;
        };
        let module = name.split("::").next().unwrap_or(name).to_string();
        let lesson = lessons.entry(module.clone()).or_default();
        lesson.file = lesson_file(chapter, &module);
        match result.trim() {
            "ok" => lesson.passed += 1,
            r if r.starts_with("FAILED") => lesson.failed += 1,
            _ => {}
        }
    }
    Ok(lessons)
}

fn today() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() / 86_400)
        .unwrap_or(0)
}

/// Days since 1970-01-01 to YYYY-MM-DD (Howard Hinnant's civil_from_days).
fn date(days: u64) -> String {
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// chapter -> (completed day, reviews done, last review day)
fn load_reviews() -> BTreeMap<u32, (u64, usize, u64)> {
    let text = fs::read_to_string(Path::new(STATE).join("chapters.tsv")).unwrap_or_default();
    text.lines()
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            Some((
                f.first()?.parse().ok()?,
                (
                    f.get(1)?.parse().ok()?,
                    f.get(2)?.parse().ok()?,
                    f.get(3)?.parse().ok()?,
                ),
            ))
        })
        .collect()
}

fn save_reviews(reviews: &BTreeMap<u32, (u64, usize, u64)>) -> Result<(), String> {
    fs::create_dir_all(STATE).map_err(|e| e.to_string())?;
    let text: String = reviews
        .iter()
        .map(|(c, (d, n, l))| format!("{c}\t{d}\t{n}\t{l}\n"))
        .collect();
    fs::write(Path::new(STATE).join("chapters.tsv"), text).map_err(|e| e.to_string())
}

fn starter_path(file: &Path) -> PathBuf {
    Path::new(STATE).join("starters").join(file)
}

fn progress() -> Result<(), String> {
    let mut reviews = load_reviews();
    let now = today();
    let mut next: Option<String> = None;
    println!("Lessons: ✓ all tests pass · ◐ started · · not started\n");
    for chapter in 3..=24 {
        let lessons = match chapter_status(chapter) {
            Ok(lessons) => lessons,
            Err(_) => {
                println!(
                    "ch{chapter:02}  does not compile: run `cargo test --test chapter{chapter:02}` to see why"
                );
                if next.is_none() {
                    next = Some(format!("cargo test --test chapter{chapter:02}"));
                }
                continue;
            }
        };
        let mut line = String::new();
        let mut done = 0;
        // Numbered lessons first, then the review and code-review lessons.
        let mut ordered: Vec<(&String, &Lesson)> = lessons.iter().collect();
        ordered.sort_by_key(|(module, _)| {
            let rank = match module.as_bytes().first() {
                Some(b'l') => 0,
                Some(b'r') => 1,
                Some(b'c') => 2,
                _ => 3,
            };
            (rank, module.as_str())
        });
        for (module, lesson) in ordered {
            let short = module.split('_').next().unwrap_or(module);
            let mark = if lesson.failed == 0 {
                done += 1;
                "✓"
            } else if lesson.passed > 0 {
                "◐"
            } else {
                "·"
            };
            line.push_str(&format!(" {mark}{short}"));
            if lesson.failed > 0 && next.is_none() {
                next = Some(format!("cargo test --test chapter{chapter:02} {short}_"));
            }
            // Keep an untouched copy so `cargo progress redo` can restore it.
            if let Some(file) = &lesson.file {
                let starter = starter_path(file);
                let expected_passes = fs::read_to_string(file)
                    .map(|t| t.matches("should_panic").count())
                    .unwrap_or(0);
                if lesson.failed > 0 && lesson.passed <= expected_passes && !starter.exists() {
                    if let Some(dir) = starter.parent() {
                        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
                    }
                    fs::copy(file, &starter).map_err(|e| e.to_string())?;
                }
            }
        }
        if done == lessons.len() && !lessons.is_empty() {
            reviews.entry(chapter).or_insert((now, 0, now));
        }
        println!("ch{chapter:02} {done:>2}/{:<2}{line}", lessons.len());
    }
    save_reviews(&reviews)?;
    let due: Vec<(u32, usize)> = reviews
        .iter()
        .filter(|(_, (_, n, last))| *n < REVIEW_DAYS.len() && now >= last + REVIEW_DAYS[*n])
        .map(|(c, (_, n, _))| (*c, *n))
        .collect();
    println!();
    if due.is_empty() {
        let upcoming = reviews
            .iter()
            .filter(|(_, (_, n, _))| *n < REVIEW_DAYS.len())
            .map(|(c, (_, n, last))| (last + REVIEW_DAYS[*n], *c))
            .min();
        if let Some((day, chapter)) = upcoming {
            println!("Next review: ch{chapter:02} on {}", date(day));
        }
    } else {
        println!("Reviews due today (spaced repetition beats rereading):");
        for (chapter, n) in due {
            println!(
                "  ch{chapter:02} (review {} of {}): answer its NOTES.md questions from memory, \
                 `cargo drill reset ch{chapter:02}`, redo the capstone with `cargo progress redo ch{chapter:02} <lesson>`, \
                 then `cargo progress reviewed ch{chapter:02}`",
                n + 1,
                REVIEW_DAYS.len()
            );
        }
    }
    match next {
        Some(command) => println!("Next lesson: {command}"),
        None => println!("Every lesson passes. Keep the reviews going."),
    }
    Ok(())
}

fn reviewed(chapter: u32) -> Result<(), String> {
    let mut reviews = load_reviews();
    let now = today();
    let Some(entry) = reviews.get_mut(&chapter) else {
        return Err(format!(
            "ch{chapter:02} is not complete yet; run `cargo progress` after finishing it"
        ));
    };
    entry.1 += 1;
    entry.2 = now;
    let message = match REVIEW_DAYS.get(entry.1) {
        Some(days) => format!(
            "Recorded. Next ch{chapter:02} review on {}.",
            date(now + days)
        ),
        None => format!("Recorded. ch{chapter:02} has finished its review schedule."),
    };
    save_reviews(&reviews)?;
    println!("{message}");
    Ok(())
}

fn redo(chapter: u32, lesson: &str) -> Result<(), String> {
    let dir = Path::new("exercises").join(format!("chapter{chapter:02}"));
    let prefix = format!("{}_", lesson.trim_end_matches('_'));
    let file = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with(&prefix))
        })
        .ok_or_else(|| format!("no lesson {lesson} in {}", dir.display()))?;
    let starter = starter_path(&file);
    let text = fs::read_to_string(&starter).map_err(|_| {
        format!(
            "no saved starter for {}; the coach saves one when it first sees the lesson unstarted",
            file.display()
        )
    })?;
    let archive = Path::new(STATE).join("attempts");
    fs::create_dir_all(&archive).map_err(|e| e.to_string())?;
    let name = file.file_name().unwrap_or_default().to_string_lossy();
    let saved = archive.join(format!("ch{chapter:02}_{}_{name}", date(today())));
    fs::copy(&file, &saved).map_err(|e| e.to_string())?;
    fs::write(&file, text).map_err(|e| e.to_string())?;
    println!(
        "Saved your solution to {} and restored the starter.",
        saved.display()
    );
    println!("Solve it again from the contract, then compare the two versions.");
    Ok(())
}

fn chapter_number(text: &str) -> Result<u32, String> {
    let digits = text.trim_start_matches("chapter").trim_start_matches("ch");
    match digits.parse() {
        Ok(n @ 3..=24) => Ok(n),
        _ => Err(format!("expected a chapter like ch04, got {text}\n{USAGE}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn errors(codes: &[&str]) -> Outcome {
        Outcome::Errors(codes.iter().map(|c| c.to_string()).collect())
    }

    #[test]
    fn judge_compares_predictions_with_outcomes() {
        assert!(judge("ok", &Outcome::Ok) == Verdict::Right);
        assert!(judge("OK", &Outcome::Panic) == Verdict::Wrong);
        assert!(judge("panic", &Outcome::Panic) == Verdict::Right);
        assert!(judge("E0382", &errors(&["E0382", "E0382"])) == Verdict::Right);
        assert!(judge("e0382, E0499", &errors(&["E0499", "E0382"])) == Verdict::Right);
        assert!(judge("E0382", &errors(&["E0382", "E0499"])) == Verdict::Partial);
        assert!(judge("error", &errors(&["E0382"])) == Verdict::Partial);
        assert!(judge("error", &errors(&[])) == Verdict::Right);
        assert!(judge("E0505", &errors(&["E0382"])) == Verdict::Wrong);
        assert!(judge("ok", &errors(&["E0382"])) == Verdict::Wrong);
    }

    #[test]
    fn expect_line_ignores_placeholders() {
        assert_eq!(expect_line("//! EXPECT: ?\nfn main() {}"), None);
        assert_eq!(expect_line("//! EXPECT:\n"), None);
        assert_eq!(expect_line("//! EXPECT: E0382\n").as_deref(), Some("E0382"));
        assert_eq!(expect_line("// EXPECT: ok\n").as_deref(), Some("ok"));
    }

    #[test]
    fn keep_lines_and_ansi() {
        let src = "let a = 1;\n    assert!(a == 1); // KEEP\n";
        assert_eq!(keep_lines(src), ["assert!(a == 1); // KEEP"]);
        assert_eq!(
            strip_ansi("\u{1b}[1m\u{1b}[38;5;9merror[E0382]\u{1b}[0m: x"),
            "error[E0382]: x"
        );
    }

    #[test]
    fn dates_and_chapters() {
        assert_eq!(date(0), "1970-01-01");
        assert_eq!(date(20_384), "2025-10-23");
        assert_eq!(date(11_016), "2000-02-29");
        assert_eq!(chapter_number("ch04"), Ok(4));
        assert_eq!(chapter_number("chapter24"), Ok(24));
        assert!(chapter_number("ch2").is_err());
        assert!(chapter_number("ch25").is_err());
    }
}
