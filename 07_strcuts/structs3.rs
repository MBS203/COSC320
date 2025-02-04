#[derive(Debug)]
#[allow(dead_code)] // Allow unused struct warning
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    #[allow(dead_code)] // Allow unused method warning
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // This method returns true if the sender and recipient countries are different.
    #[allow(dead_code)] // Allow unused method warning
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    // This method calculates the transport fee based on the weight and cents per gram.
    #[allow(dead_code)] // Allow unused method warning
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        self.weight_in_grams * cents_per_gram
    }
}

fn main() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Russia");

    // Instantiate the package
    let package = Package::new(sender_country, recipient_country, 1200);

    // Optionally print the package and check if it's international
    println!("{:?}", package);
    println!("Is the package international? {}", package.is_international());

    // Calculate and print fees
    let cents_per_gram = 5;
    println!("Fees: {}", package.get_fees(cents_per_gram));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
