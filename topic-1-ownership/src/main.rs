// MiniOS — Topic 1: Rust ownership foundation.
//
// This models a single process record and a handful of functions that
// exercise Rust's ownership rules. Nothing here is a real scheduler yet;
// that arrives in later H230 discussions.

#[derive(Debug)]
struct Process {
    pid: u32,
    name: String,
}

/// Takes ownership of `name`. The caller's binding is moved and can no
/// longer be used after this call returns.
fn load_process(name: String) {
    println!("loaded process: {name}");
}

/// Borrows `name` immutably. The caller keeps ownership and can keep
/// using its `String` after this call.
fn inspect_process(name: &String) {
    println!("inspecting process: {name}");
}

/// Borrows `name` mutably, so it can modify the caller's `String` in
/// place without taking ownership of it.
fn rename_process(name: &mut String) {
    name.push_str("_v2");
}

/// Takes ownership of `name`, inspects it, and then hands ownership
/// back to the caller through the return value.
fn inspect_and_return(name: String) -> String {
    println!("inspecting and returning: {name}");
    name
}

/// `i32` implements `Copy`, so passing it by value implicitly copies
/// the value and leaves the caller's original binding usable.
fn inspect_capacity(capacity: i32) {
    println!("scheduler capacity: {capacity}");
}

fn main() {
    let process = Process {
        pid: 1,
        name: String::from("compiler"),
    };

    println!("pid={} name={}", process.pid, process.name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_is_moved_into_load_process() {
        let name = String::from("compiler");

        // `name` is moved into `load_process` here; the binding above
        // no longer owns the String after this call.
        load_process(name);

        // This would fail to compile: `name` was moved into
        // `load_process` and is no longer valid in this scope.
        // println!("{name}");
    }

    #[test]
    fn i32_is_copied_not_moved() {
        let capacity: i32 = 4;

        inspect_capacity(capacity);

        // `i32` is `Copy`, so `inspect_capacity` received a copy of
        // `capacity` and the original is still usable here.
        assert_eq!(capacity, 4);
    }

    #[test]
    fn immutable_borrow_leaves_owner_usable() {
        let name = String::from("compiler");

        inspect_process(&name);

        // Only a reference was lent out, so `name` still owns its data.
        assert_eq!(name, "compiler");
    }

    #[test]
    fn mutable_borrow_modifies_in_place() {
        let mut name = String::from("compiler");

        rename_process(&mut name);

        assert_eq!(name, "compiler_v2");
    }

    #[test]
    fn clone_creates_a_separate_owner() {
        let name = String::from("compiler");
        let name_clone = name.clone();

        // The clone is a distinct owned String with its own heap
        // allocation, so moving it away does not affect `name`.
        load_process(name_clone);

        assert_eq!(name, "compiler");
    }

    #[test]
    fn ownership_can_be_returned() {
        let name = String::from("compiler");

        let name = inspect_and_return(name);

        assert_eq!(name, "compiler");
    }

    #[test]
    fn slice_borrows_part_of_the_string() {
        let name = String::from("compiler");

        // `prefix` borrows part of `name`'s bytes; it owns nothing.
        let prefix: &str = &name[..3];

        assert_eq!(prefix, "com");
        assert_eq!(name, "compiler");
    }

    #[test]
    fn fixed_width_integers_need_explicit_overflow_handling() {
        let capacity: u8 = 255;

        assert_eq!(capacity.checked_add(1), None);
        assert_eq!(capacity.wrapping_add(1), 0);
    }
}
