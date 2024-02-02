// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // array with 100 elements with the value 0
    let a = [0; 100];
    // aray with explicit identifier 
    // 5 i32 elements, 1 to 5
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else if a.len() == 5 {
        println!("Array has {} values {} to {}", a.len(), a[0], a[a.len() - 1]);
    } else {
        println!("Meh, I eat arrays with size {} for breakfast.", a.len());
        panic!("Array not big enough, more elements needed")
    }
}
