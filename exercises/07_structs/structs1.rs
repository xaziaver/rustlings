// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.


// structure name describes the 
// significance of data being grouped
// in this case 3 RGB values constitute the color
struct ColorClassicStruct {
    red: i32,
    blue: i32,
    green: i32,
}
// "tuple structs" can be created to add meaning
// to a group of data when names are
// not needed for the associated fields
struct ColorTupleStruct(u8, u8, u8);
// u8 is an unsigned 8-bit integer
// holds values 0 to 255 

#[derive(Debug)]
// "unit-like structs" behave similarly to
// the unit type () and used when you want to 
// implement a trait on some type, but don't
// have any data you need to store in the type
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // you must understand that the value for red is the
        // first value, green is the second and blue is the third
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
