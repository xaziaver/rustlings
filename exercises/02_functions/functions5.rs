// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {

    // removing semi-colon makes the function
    // return the value of this expression
    num * num
    // the value of the last expression in
    // a block {} is automatically returned

    // shorthand for
    // return num * num;
}
