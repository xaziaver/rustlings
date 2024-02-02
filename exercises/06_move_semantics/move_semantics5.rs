// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    /*
    let mut x = 100;
    let y = &mut x;    // can't have two mutable borrows
    let z = &mut x;    // of x at the same time
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
    */

    let mut x = 100;
    let y = &mut x;
    *y += 100;          // after this line y is no longer used
    let z = &mut x;     // so x can be borrowed again
    *z += 1000;
    assert_eq!(x, 1200);
}
