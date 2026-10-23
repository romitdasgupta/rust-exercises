# Chapter 24 notes — Foreign Functions

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · C representations

**Predictions I got wrong, and why:**

- 

**Explain:** Explain primitive aliases, field layout, padding, and why a raw struct byte dump is not a portable wire format.

> 

**Edge case I added:**

- 

## l02 · C strings and ownership

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish owned CString, borrowed CStr, raw pointer lifetime, and UTF-8 validation.

> 

**Edge case I added:**

- 

## l03 · ABI calls and out-parameters

**Predictions I got wrong, and why:**

- 

**Explain:** Explain success-dependent initialization and why a function-pointer ABI must match exactly.

> 

**Edge case I added:**

- 

## l04 · Callbacks and userdata

**Predictions I got wrong, and why:**

- 

**Explain:** Explain callback lifetime, pointer retention, and why this provided callback uses nonpanicking saturation. The extension lab handles arbitrary closure panics.

> 

**Edge case I added:**

- 

## l05 · Opaque handle ownership

**Predictions I got wrong, and why:**

- 

**Explain:** Specify the ownership transfer at construction and exactly-once destructor call. Explain why the mock uses Box consistently and a real library needs its own free function.

> 

**Edge case I added:**

- 

## l06 · Owner-bound foreign borrows

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the lifetime of the returned borrowed object and the difference between it and an owned copy. Relate this to repositories and commits.

> 

**Edge case I added:**

- 

## l07 · Capstone: checked foreign transaction

**Predictions I got wrong, and why:**

- 

**Explain:** Audit arguments, status, initialized output, owned error copying, and no unwinding across the ABI. Explain which real libgit2 obligations remain in LABS.md.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**How do repr(C), C primitive aliases, size_t, pointers, and ABI declarations agree across languages?**

> 

**Why do CString and CStr have different ownership/lifetime roles?**

> 

**When is a foreign output parameter initialized, and what may be read after failure?**

> 

**Which allocator frees each foreign object, and how is cleanup made exception safe?**

> 

**How do borrowed foreign objects retain their owner’s lifetime?**

> 

**How do callbacks retain userdata lifetime and prevent unwinding through a C ABI boundary?**

> 

**Which initialization/shutdown, threading, error-message, version, and linker obligations must a safe wrapper document?**

>
