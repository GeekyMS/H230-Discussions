# Topic 2 — Bit Flags and Endianness

## Objective

Practice bitwise operators (`&`, `|`, `^`, `!`, `<<`, `>>`) and endian
conversion by packing MiniOS process state into a single `u32`.

## State layout

| Bits  | Field           | Meaning                          |
|-------|-----------------|----------------------------------|
| 0     | `is_runnable`   | in the ready queue               |
| 1     | `is_privileged` | kernel mode                      |
| 2     | `is_blocked`    | waiting on I/O                   |
| 3-6   | `priority`      | 0..=15                           |
| 8-23  | `mem_pages`     | 0..=65535                        |
| 31    | `endian`        | 0 = little-endian, 1 = big-endian|

## Endian behavior

`priority()` and `mem_pages()` extract their field with shift + mask, then
read bit 31. If it is 1 the value is byte-swapped to big-endian layout
(`0x1234` → `0x3412_0000`); if 0 it is returned unchanged.

## Run

```
cargo run
cargo test
```
