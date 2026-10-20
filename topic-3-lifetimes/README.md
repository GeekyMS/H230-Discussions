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

## Compile errors (verified, commented out in `main.rs`)

- Dangling reference: `E0597`: `dead` does not live long enough
- Aliasing: `E0502`: cannot borrow `shell` as mutable because it is also borrowed as immutable

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
