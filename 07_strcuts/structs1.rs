// Allow unused code warning for the structs
#[allow(dead_code)]
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
#[allow(dead_code)]
struct UnitStruct;

fn main() {
    // The main function doesn't need to use the structs, but it's here to make the program complete.
    println!("The main function is running.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // Instantiate a regular struct with RGB values for green color
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        // Test the fields of the regular struct
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct with RGB values for green color
        let green = ColorTupleStruct(0, 255, 0);

        // Test the fields of the tuple struct
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit struct
        let unit_struct = UnitStruct;

        // Use Debug format to print a message
        let message = format!("{:?}s are fun!", unit_struct);

        // Test the message
        assert_eq!(message, "UnitStructs are fun!");
    }
}
