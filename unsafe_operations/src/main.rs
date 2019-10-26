// one should try to minimize the amount of unsafe code in a code base
// there are four primary things that unsafe is used for:
// dereferencing raw pointers
// calling functions or methods which are unsafe(including calling a function over FFI)
// accessing or modifying static mutable variables
// implementing unsafe traits
fn main() {
    raw_pointers();
    calling_unsafe_functions();
}

fn raw_pointers() {
    // raw pointers * are similarly the references &T, but references are always safe because they
    // are guranteed to point to valid data due to the borrow checker.
    // dereferencing a raw pointer can only be done through an unsafe block.
    let raw_p: *const u32 = &10;
    unsafe {
        assert_eq!(*raw_p, 10);
    }
}

use std::slice;
fn calling_unsafe_functions() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
