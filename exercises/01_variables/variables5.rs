// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);

    /* 
        using `let` here allows the use of "shadowing" in Rust
        where you can reassign the value and type of a named variable
        without needing to make it mutable
    */
    let number = 3; // don't rename this 
    // we can change type because a new variable is effectively created
    {
        let number = number + 5;
        println!("Number plus two inner scope is : {}", number + 2);
    }
    /*
        shadowing is valid until the end of the scope
    */
    println!("Number plus two outer scope is : {}", number + 2);
}
