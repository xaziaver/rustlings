// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // vec0's ownership is transferred to fill_vec(), 
    // making vec0 invalid after this point.
    let vec1 = fill_vec(vec0);

    // this would lead to a compile-time error
    // assert_eq!(vec0, vec![22, 44, 66]);
    
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // needs to be mutable to use push()
    let mut vec = vec;
    // push() potentially allocates new memory
    // and adds a value to the end of the vector
    vec.push(88);

    vec
}
