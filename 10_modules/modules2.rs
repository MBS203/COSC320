// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    pub mod fruits {
        pub const PEAR: &str = "Pear";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
    }

    // Correcting the use statements within the module
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
}

// Bringing the aliases into the main scope
use delicious_snacks::{fruit, veggie};

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie,
    );
}

