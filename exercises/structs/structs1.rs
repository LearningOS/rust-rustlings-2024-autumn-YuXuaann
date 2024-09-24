// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorClassicStruct {
    fn new(r: u8, g: u8, b: u8) -> Self {
        ColorClassicStruct {
            red: r,
            green: g,
            blue: b,
        }
    }
}

struct ColorTupleStruct(u8, u8, u8);

impl ColorTupleStruct {
    fn new(r: u8, g: u8, b: u8) -> Self {
        ColorTupleStruct(r, g, b)
    }
}

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct::new(0, 255, 0);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct::new(0, 255, 0);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_like_struct = (UnitLikeStruct);
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
