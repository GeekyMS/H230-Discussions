# Topic 1 — Ownership

## Objective

Explore Rust's ownership model — moves, `Copy`, borrowing, cloning, and
slices — using process-oriented examples from MiniOS, a userspace
process-management and scheduling system that will grow across the
semester's discussions.

## Project Pitch

MiniOS is a userspace process-management and scheduling system written in
Rust. The system will be extended incrementally as new H230 topics are
introduced.

It is designed to naturally support the three required directions:

- **Linked data structure:** a linked ready queue for runnable processes.
- **Concurrent control flows:** scheduler, process-monitor, and other
  components running concurrently using threads and synchronization.
- **Client-server networking:** a TCP interface through which a separate
  client can inspect or control MiniOS.

For Topic 1, only the Rust ownership foundation is implemented.

## What was implemented

A `Process` struct plus a small set of functions in [src/main.rs](src/main.rs)
that each exercise one ownership concept:

- `load_process(name: String)` — takes ownership of a `String` (move).
- `inspect_process(name: &String)` — immutable borrow.
- `rename_process(name: &mut String)` — mutable borrow.
- `inspect_and_return(name: String) -> String` — takes and returns ownership.
- `inspect_capacity(capacity: i32)` — pass-by-value of a `Copy` type.

Unit tests cover each behavior, including a string slice example and
explicit fixed-width integer overflow handling (`checked_add`,
`wrapping_add`).

## Observations

- Passing a `String` by value moves it — the original binding becomes
  unusable unless ownership is explicitly returned.
- `i32` implements `Copy`, so passing it by value copies the value; the
  original remains usable after the call.
- A `&String` / `&mut String` borrow never copies the heap-allocated
  contents — it only grants temporary access to the existing allocation.
  `.clone()` is the one that actually duplicates the heap data.
- A string slice (`&name[..3]`) borrows part of the original `String`'s
  bytes; it owns nothing and is only valid as long as the `String` is.
- Fixed-width integers (`u8`, etc.) wrap or panic on overflow depending
  on build mode; `checked_add` and `wrapping_add` make the behavior
  explicit instead of relying on debug/release differences.

### Move error, concretely

```rust
let name = String::from("compiler");
load_process(name);
println!("{name}");
```

fails to compile with `error[E0382]: borrow of moved value: 'name'` —
`name`'s only owner moved into `load_process`, so the compiler rejects
the later use.

## Results

No quantitative experiments this week — ownership/borrowing is a
compile-time, not a runtime, property. Quantitative results (e.g.
scheduling parameter sweeps with generated plots) begin in later
topics once MiniOS has runtime behavior to measure.
