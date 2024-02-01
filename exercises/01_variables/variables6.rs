// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.


/*
    const differences:
        (1) can't use mut keyword, always immutable
        (1) declare with const, uppercase, underscores
        (3) the type MUST be annotated, not inferred 
        (4) may only be set to a constant expression 
*/
const NUMBER_I_PICKED: i32 = 3;
fn main() {
    println!("Number {}", (NUMBER_I_PICKED * 2) + 2);
}
