#!/usr/bin/env python3
"""Generate exercises/chapterNN/NOTES.md from lesson headers and the chapter
guide's mastery check. Maintainer tool: run it after adding or renaming a
lesson. It never overwrites an existing NOTES.md unless --force is given,
because learners write their answers there."""
import pathlib, re, sys

KEYS = ("PREDICT", "EXPLAIN", "IMPLEMENT", "COMPILER", "TRANSFER", "BEFORE CODING", "Use examples", "Run:", "Then:", "Do this")

def header(path):
    lines = []
    for line in path.read_text().splitlines():
        if not line.startswith("//!"):
            break
        lines.append(line[3:].strip())
    return lines

def section(lines, key):
    out, on = [], False
    for line in lines:
        starts = any(line.startswith(k) for k in KEYS)
        if line.startswith(key):
            on = True
            text = re.sub(r"^%s[^:]*:\s*" % key, "", line)
            if text:
                out.append(text)
            continue
        if on and (not line or starts):
            break
        if on:
            out.append(line)
    return " ".join(out)

def chapter(dir, force):
    notes = dir / "NOTES.md"
    if notes.exists() and not force:
        return
    n = int(dir.name[-2:])
    readme = (dir / "README.md").read_text()
    title = readme.splitlines()[0].split("—")[-1].strip()
    out = [f"# Chapter {n} notes — {title}", "",
           "Your workbook. Write answers in your own words, *before* checking the hints. "
           "When `cargo progress` says this chapter is due for review, answer the mastery "
           "questions at the bottom again from memory, then compare with what you wrote here.", ""]
    order = {"l": 0, "r": 1, "c": 2}
    for f in sorted(dir.glob("*.rs"), key=lambda f: (order.get(f.name[0], 3), f.name)):
        if f.name == "mod.rs":
            continue
        lines = header(f)
        if not lines:
            continue
        name = re.sub(r"^(LESSON|REVIEW|CODE REVIEW)\s+", "", lines[0])
        out += [f"## {f.stem.split('_')[0]} · {name.split('—', 1)[-1].strip()}", ""]
        if f.name.startswith("c"):
            out += ["**My review, written before running the tests:**", "", "- ", ""]
        else:
            out += ["**Predictions I got wrong, and why:**", "", "- ", ""]
        explain = section(lines, "EXPLAIN")
        if explain:
            out += [f"**Explain:** {explain}", "", "> ", ""]
        out += ["**Edge case I added:**", "", "- ", ""]
    out += ["## Drills", "", "Drills whose verdict surprised me, and the rule I learned:", "", "- ", ""]
    m = re.search(r"## Mastery check\n(.*?)(\n## |\Z)", readme, re.S)
    if m:
        out += ["## Mastery check (answer from memory)", ""]
        for item in re.findall(r"^(?:- \[ \]|\d+\.)\s*(.+)$", m.group(1), re.M):
            out += [f"**{item.strip()}**", "", "> ", ""]
    notes.write_text("\n".join(out).rstrip() + "\n")

force = "--force" in sys.argv
targets = [a for a in sys.argv[1:] if a != "--force"]
root = pathlib.Path(__file__).resolve().parent.parent / "exercises"
for d in sorted(root.glob("chapter*")):
    if not targets or d.name[-2:] in targets or d.name in targets:
        chapter(d, force)
