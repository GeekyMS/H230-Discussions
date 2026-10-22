# Topic 3 — Structs and Lifetimes

## Objective

Model a natural dependency in MiniOS with two structs, one borrowing the
other, and use a lifetime parameter so the compiler guarantees memory safety.

## Structs

- `Process` — owns its data (`pid`, `name`, `priority`).
- `ScheduleEntry<'a>` — a run-queue record holding `&'a Process` plus a
  `time_slice_ms`. It borrows the process rather than copying it.

The scheduler entry only makes sense while the process exists, so `'a`
encodes "this entry cannot outlive the process it points at."

## What `main` shows

- `Process` values are created in the outer scope; `ScheduleEntry` values in
  an inner scope, so the referenced data outlives the reference.
- `describe_entry<'a>(&ScheduleEntry<'a>) -> &'a str` takes a struct with a
  lifetime and returns a borrow tied to the *process*, not the entry.
- `higher_priority<'a>` takes two entries sharing one lifetime and returns one.
- A commented-out block shows the dangling-reference case the borrow checker
  rejects (`dead does not live long enough`).

- `Process::set_priority(&mut self)` and a `Quantum(u32)` tuple struct cover
  mutable borrowing (Ch. 4.2) and another struct form (Ch. 5). A mutable
  borrow is only allowed once the entry's shared borrow is no longer used.

## Compiler errors and how each is solved

All three were reproduced and compiled while preparing this.

1. `E0106` missing lifetime specifier, from
   `fn describe_entry(e: &ScheduleEntry) -> &str`.
   `&ScheduleEntry` has two input lifetimes (the outer `&` and the hidden
   one inside `ScheduleEntry<'_>`), so elision cannot pick which one the
   returned `&str` comes from.
   **Fix:** name the lifetime: `fn describe_entry<'a>(e: &ScheduleEntry<'a>) -> &'a str`.
   Choosing `'a` (not the outer borrow) ties the name to the *process*, so it
   stays usable after the entry is dropped.

2. `E0597` `dead` does not live long enough (commented out in `main.rs`).
   An entry outlived the process it borrowed (`dead` dropped in an inner
   scope while `entry` was used outside it).
   **Fix:** declare the `Process` in an outer scope so it is dropped after
   every entry that borrows it.

3. `E0502` cannot borrow `shell` as mutable because it is also borrowed as
   immutable (commented out in `main.rs`). `set_priority(&mut self)` was
   called while an entry's shared borrow was still used afterwards.
   **Fix:** finish using the entry first (or create the entry after the
   mutation), so the shared borrow has ended.

Note: signature `fn describe_entry<'s, 'a>(e: &'s ScheduleEntry<'a>) -> &'s str`
compiles, but then using the returned name after the entry is dropped gives
`E0597`. That is why the returned lifetime is `'a`, not the entry's borrow.

## Tests

`cargo test` runs 6 tests: the entry borrows rather than copies
(`ptr::eq`), a name borrowed through an entry outlives that entry,
`describe_entry`, `higher_priority` (including ties), mutation after the
borrow ends, and the tuple struct.

## Run

```
cargo run
cargo test
```
