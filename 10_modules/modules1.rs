// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Function to make sausage
    pub fn make_sausage() {
        let _secret = get_secret_recipe(); // Using the secret recipe
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage(); // Call to make sausage
}
