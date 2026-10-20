// MiniOS — Topic 3: references, structs, and lifetimes.
//
// A `Process` owns its data (name, pid, priority). A `ScheduleEntry` is a
// short-lived record in the scheduler's run queue that *borrows* a
// `Process` instead of copying it. The lifetime `'a` ties the entry to the
// process it points at: the compiler guarantees the process outlives every
// entry that references it, so the scheduler can never hold a dangling
// pointer to a process that has been torn down.

/// Owns its data; the "referenced" struct.
struct Process {
    pid: u32,
    name: String,
    priority: u8,
}

/// Borrows a `Process`; the "referencing" struct.
/// `'a` means: a `ScheduleEntry<'a>` cannot outlive the `Process` it borrows.
struct ScheduleEntry<'a> {
    process: &'a Process,
    time_slice_ms: u32,
}

impl Process {
    fn new(pid: u32, name: &str, priority: u8) -> Self {
        Process { pid, name: name.to_string(), priority }
    }

    /// Needs `&mut self`: only legal while no shared borrow (e.g. a
    /// `ScheduleEntry`) of this process is still in use.
    fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }
}

/// Tuple struct (Ch. 5): a newtype around a time slice in milliseconds.
struct Quantum(u32);

impl<'a> ScheduleEntry<'a> {
    fn new(process: &'a Process, time_slice_ms: u32) -> Self {
        ScheduleEntry { process, time_slice_ms }
    }

    /// Returns a borrow of the process name. Elided, this is
    /// `fn process_name<'s>(&'s self) -> &'s str`; the explicit version below
    /// shows the stronger guarantee that the name lives as long as the
    /// *process*, not just as long as this entry.
    fn process_name(&self) -> &'a str {
        &self.process.name
    }
}

/// Takes an entry (which carries a lifetime) and reports it.
/// The `'a` is required because the struct has a lifetime parameter and the
/// return value borrows from the process behind the entry, not from `entry`.
fn describe_entry<'a>(entry: &ScheduleEntry<'a>) -> &'a str {
    println!(
        "  entry: pid={} name={} priority={} slice={}ms",
        entry.process.pid, entry.process.name, entry.process.priority, entry.time_slice_ms
    );
    &entry.process.name
}

/// Picks the entry whose process has the higher priority. Both entries and
/// the result share one lifetime `'a`, so the result is valid only while
/// both underlying processes are.
fn higher_priority<'a>(a: ScheduleEntry<'a>, b: ScheduleEntry<'a>) -> ScheduleEntry<'a> {
    if a.process.priority >= b.process.priority { a } else { b }
}

fn main() {
    // The referenced objects live in the OUTER scope, so they outlive the
    // entries created in the inner scope.
    let init = Process::new(1, "init", 10);
    let shell = Process::new(2, "shell", 5);

    {
        // Inner scope: entries borrow from `init` and `shell`.
        let e1 = ScheduleEntry::new(&init, 20);
        let e2 = ScheduleEntry::new(&shell, 10);

        let name = describe_entry(&e1);
        println!("  borrowed name: {name}");
        println!("  via method:    {}", e2.process_name());

        let winner = higher_priority(e1, e2);
        println!("winner: {}", winner.process.name);
    } // e1, e2, winner dropped here; init and shell are still alive.

    println!("processes still valid: {} and {}", init.name, shell.name);

    // Ch. 4.2: the entry's shared borrow has ended (last use was above), so
    // a mutable borrow of `shell` is now allowed.
    let mut shell = shell;
    shell.set_priority(8);
    let e3 = ScheduleEntry::new(&shell, Quantum(15).0);
    describe_entry(&e3);

    // Uncommenting this fails (E0502: cannot borrow `shell` as mutable because
    // it is also borrowed as immutable):
    //
    // let e4 = ScheduleEntry::new(&shell, 5);
    // shell.set_priority(1);
    // println!("{}", e4.process.name);

    // Uncommenting this fails to compile (`dead` does not live long enough):
    //
    // let entry;
    // {
    //     let dead = Process::new(3, "dead", 1);
    //     entry = ScheduleEntry::new(&dead, 5);
    // } // `dead` dropped while still borrowed
    // println!("{}", entry.process.name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_borrows_without_copying() {
        let p = Process::new(1, "init", 10);
        let e = ScheduleEntry::new(&p, 20);
        assert!(std::ptr::eq(e.process, &p));
        assert_eq!(e.time_slice_ms, 20);
    }

    #[test]
    fn process_name_outlives_entry() {
        let p = Process::new(1, "init", 10);
        let name;
        {
            let e = ScheduleEntry::new(&p, 20);
            name = e.process_name(); // tied to `p`, not to `e`
        }
        assert_eq!(name, "init");
    }

    #[test]
    fn describe_entry_returns_process_name() {
        let p = Process::new(2, "shell", 5);
        let e = ScheduleEntry::new(&p, 10);
        assert_eq!(describe_entry(&e), "shell");
    }

    #[test]
    fn higher_priority_picks_larger_and_ties_pick_first() {
        let a = Process::new(1, "a", 10);
        let b = Process::new(2, "b", 5);
        let c = Process::new(3, "c", 10);
        let w = higher_priority(ScheduleEntry::new(&a, 1), ScheduleEntry::new(&b, 1));
        assert_eq!(w.process.pid, 1);
        let w = higher_priority(ScheduleEntry::new(&b, 1), ScheduleEntry::new(&a, 1));
        assert_eq!(w.process.pid, 1);
        let w = higher_priority(ScheduleEntry::new(&a, 1), ScheduleEntry::new(&c, 1));
        assert_eq!(w.process.pid, 1);
    }

    #[test]
    fn mutation_allowed_after_borrow_ends() {
        let mut p = Process::new(1, "init", 10);
        {
            let e = ScheduleEntry::new(&p, 20);
            assert_eq!(e.process.priority, 10);
        }
        p.set_priority(3);
        let e = ScheduleEntry::new(&p, 20);
        assert_eq!(e.process.priority, 3);
    }

    #[test]
    fn quantum_tuple_struct() {
        assert_eq!(Quantum(15).0, 15);
    }
}
