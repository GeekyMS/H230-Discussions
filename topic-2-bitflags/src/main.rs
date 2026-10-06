// MiniOS — Topic 2: bit flags and endianness.
//
// A single `u32` (`ProcState`) packs the state of one MiniOS process.
//
//   bit  0      is_runnable   (1 = in the ready queue)
//   bit  1      is_privileged (1 = kernel mode)
//   bit  2      is_blocked    (1 = waiting on I/O)
//   bits 3-6    priority      (4 bits, 0..=15)
//   bit  7      unused
//   bits 8-23   mem_pages     (16 bits, 0..=65535)
//   bits 24-30  unused
//   bit  31     endian        (0 = little-endian, 1 = big-endian)
//
// The endian bit is the single designated "endianness" bit. Multi-bit
// getters consult it: when it is 1, the extracted value is converted to
// big-endian byte order before being returned.

type ProcState = u32;

const RUNNABLE_BIT: u32 = 0;
const PRIVILEGED_BIT: u32 = 1;
const BLOCKED_BIT: u32 = 2;
const PRIORITY_SHIFT: u32 = 3;
const PRIORITY_MASK: u32 = 0b1111;
const MEM_PAGES_SHIFT: u32 = 8;
const MEM_PAGES_MASK: u32 = 0xFFFF;
const ENDIAN_BIT: u32 = 31;

/// Extracts a single bit as a bool.
fn get_bit(state: ProcState, bit: u32) -> bool {
    (state >> bit) & 1 == 1
}

/// Returns `state` with `bit` set or cleared.
fn set_bit(state: ProcState, bit: u32, on: bool) -> ProcState {
    if on { state | (1 << bit) } else { state & !(1 << bit) }
}

/// Extracts `mask`-wide field starting at `shift`.
fn get_field(state: ProcState, shift: u32, mask: u32) -> u32 {
    (state >> shift) & mask
}

/// Returns `state` with the field replaced by `value` (truncated to `mask`).
fn set_field(state: ProcState, shift: u32, mask: u32, value: u32) -> ProcState {
    (state & !(mask << shift)) | ((value & mask) << shift)
}

fn is_runnable(state: ProcState) -> bool {
    get_bit(state, RUNNABLE_BIT)
}

fn is_privileged(state: ProcState) -> bool {
    get_bit(state, PRIVILEGED_BIT)
}

fn is_blocked(state: ProcState) -> bool {
    get_bit(state, BLOCKED_BIT)
}

/// True when the endian bit says the system is big-endian.
fn is_big_endian(state: ProcState) -> bool {
    get_bit(state, ENDIAN_BIT)
}

/// Reorders the bytes of `value` into big-endian layout. Uses an explicit
/// byte swap (not `to_be`) so the result is the same on any host: the
/// value's in-register bytes are reversed.
fn to_big_endian(value: u32) -> u32 {
    value.swap_bytes()
}

/// Applies the endian bit: unchanged if 0, byte-swapped if 1.
fn apply_endian(state: ProcState, value: u32) -> u32 {
    if is_big_endian(state) { to_big_endian(value) } else { value }
}

/// Priority (4 bits), converted per the endian bit.
fn priority(state: ProcState) -> u32 {
    apply_endian(state, get_field(state, PRIORITY_SHIFT, PRIORITY_MASK))
}

/// Memory page count (16 bits), converted per the endian bit.
fn mem_pages(state: ProcState) -> u32 {
    apply_endian(state, get_field(state, MEM_PAGES_SHIFT, MEM_PAGES_MASK))
}

fn main() {
    let mut state: ProcState = 0;
    state = set_bit(state, RUNNABLE_BIT, true);
    state = set_field(state, PRIORITY_SHIFT, PRIORITY_MASK, 9);
    state = set_field(state, MEM_PAGES_SHIFT, MEM_PAGES_MASK, 0x1234);

    println!("state (little-endian): {state:#034b}");
    println!("runnable={} privileged={} blocked={}",
        is_runnable(state), is_privileged(state), is_blocked(state));
    println!("priority={:#x} mem_pages={:#x}", priority(state), mem_pages(state));

    state = set_bit(state, ENDIAN_BIT, true);
    println!("state (big-endian):    {state:#034b}");
    println!("priority={:#x} mem_pages={:#x}", priority(state), mem_pages(state));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProcState {
        let mut s = set_bit(0, RUNNABLE_BIT, true);
        s = set_bit(s, BLOCKED_BIT, true);
        s = set_field(s, PRIORITY_SHIFT, PRIORITY_MASK, 9);
        set_field(s, MEM_PAGES_SHIFT, MEM_PAGES_MASK, 0x1234)
    }

    #[test]
    fn single_bit_flags() {
        let s = sample();
        assert!(is_runnable(s));
        assert!(!is_privileged(s));
        assert!(is_blocked(s));
        assert!(!is_big_endian(s));
    }

    #[test]
    fn little_endian_returns_plain_values() {
        let s = sample();
        assert_eq!(priority(s), 9);
        assert_eq!(mem_pages(s), 0x1234);
    }

    #[test]
    fn big_endian_bit_swaps_bytes() {
        let s = set_bit(sample(), ENDIAN_BIT, true);
        assert!(is_big_endian(s));
        assert_eq!(priority(s), 0x0900_0000);
        assert_eq!(mem_pages(s), 0x3412_0000);
    }

    #[test]
    fn setting_fields_does_not_disturb_neighbours() {
        let s = set_field(sample(), PRIORITY_SHIFT, PRIORITY_MASK, 0xFF); // truncated to 15
        assert_eq!(get_field(s, PRIORITY_SHIFT, PRIORITY_MASK), 15);
        assert_eq!(get_field(s, MEM_PAGES_SHIFT, MEM_PAGES_MASK), 0x1234);
        assert!(is_runnable(s) && is_blocked(s));
    }

    #[test]
    fn swap_twice_round_trips() {
        assert_eq!(to_big_endian(to_big_endian(0xDEADBEEF)), 0xDEADBEEF);
    }

    #[test]
    fn set_bit_sets_and_clears() {
        let s = set_bit(0, PRIVILEGED_BIT, true);
        assert_eq!(s, 0b10);
        assert_eq!(set_bit(s, PRIVILEGED_BIT, false), 0);
        assert_eq!(set_bit(s, PRIVILEGED_BIT, true), s); // idempotent
    }

    #[test]
    fn get_bit_every_position() {
        for bit in 0..32 {
            let s = 1u32 << bit;
            assert!(get_bit(s, bit));
            assert!(!get_bit(!s, bit));
        }
    }

    #[test]
    fn empty_state_is_all_zero() {
        let s: ProcState = 0;
        assert!(!is_runnable(s) && !is_privileged(s) && !is_blocked(s));
        assert!(!is_big_endian(s));
        assert_eq!(priority(s), 0);
        assert_eq!(mem_pages(s), 0);
    }

    #[test]
    fn priority_boundaries() {
        for p in [0, 1, 15] {
            let s = set_field(0, PRIORITY_SHIFT, PRIORITY_MASK, p);
            assert_eq!(priority(s), p);
        }
    }

    #[test]
    fn mem_pages_boundaries() {
        for m in [0, 1, 0xFF, 0x100, 0xFFFF] {
            let s = set_field(0, MEM_PAGES_SHIFT, MEM_PAGES_MASK, m);
            assert_eq!(mem_pages(s), m);
        }
    }

    #[test]
    fn mem_pages_truncates_overflow() {
        let s = set_field(0, MEM_PAGES_SHIFT, MEM_PAGES_MASK, 0x1_0005);
        assert_eq!(mem_pages(s), 5);
        assert_eq!(s >> 24, 0); // did not spill into bits 24+
    }

    #[test]
    fn unused_bits_stay_zero() {
        let s = set_field(0, PRIORITY_SHIFT, PRIORITY_MASK, 15);
        let s = set_field(s, MEM_PAGES_SHIFT, MEM_PAGES_MASK, 0xFFFF);
        assert!(!get_bit(s, 7));
        assert_eq!(get_field(s, 24, 0x7F), 0);
    }

    #[test]
    fn flags_are_independent() {
        let mut s = 0;
        s = set_bit(s, RUNNABLE_BIT, true);
        s = set_bit(s, PRIVILEGED_BIT, true);
        s = set_bit(s, RUNNABLE_BIT, false);
        assert!(!is_runnable(s) && is_privileged(s) && !is_blocked(s));
    }

    #[test]
    fn endian_bit_does_not_change_other_flags() {
        let s = sample();
        let be = set_bit(s, ENDIAN_BIT, true);
        assert_eq!(is_runnable(s), is_runnable(be));
        assert_eq!(is_blocked(s), is_blocked(be));
        assert_eq!(get_field(s, MEM_PAGES_SHIFT, MEM_PAGES_MASK),
                   get_field(be, MEM_PAGES_SHIFT, MEM_PAGES_MASK));
    }

    #[test]
    fn clearing_endian_bit_restores_plain_values() {
        let be = set_bit(sample(), ENDIAN_BIT, true);
        let le = set_bit(be, ENDIAN_BIT, false);
        assert_eq!(priority(le), 9);
        assert_eq!(mem_pages(le), 0x1234);
    }

    #[test]
    fn big_endian_conversion_is_swap_of_little() {
        let be = set_bit(sample(), ENDIAN_BIT, true);
        let le = sample();
        assert_eq!(mem_pages(be), mem_pages(le).swap_bytes());
        assert_eq!(priority(be), priority(le).swap_bytes());
    }

    #[test]
    fn to_big_endian_known_values() {
        assert_eq!(to_big_endian(0x0000_0001), 0x0100_0000);
        assert_eq!(to_big_endian(0x1234_5678), 0x7856_3412);
        assert_eq!(to_big_endian(0), 0);
        assert_eq!(to_big_endian(u32::MAX), u32::MAX);
    }

    #[test]
    fn apply_endian_selects_by_bit() {
        assert_eq!(apply_endian(0, 0x1234), 0x1234);
        assert_eq!(apply_endian(1 << ENDIAN_BIT, 0x1234), 0x3412_0000);
    }
}
