#[derive(Debug)]
#[allow(dead_code)] // Allow unused struct warning
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

#[allow(dead_code)] // Allow unused function warning
fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // Main function is provided to satisfy compiler requirement, but you can leave it empty
    println!("Main function running");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // Create your own order using the update syntax and template above
        let your_order = Order {
            name: String::from("Hacker in Rust"), // Updating the name field
            count: 1,                             // Setting count to 1
            ..order_template // Copying the rest of the fields from the template
        };

        // Test the assertions
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
