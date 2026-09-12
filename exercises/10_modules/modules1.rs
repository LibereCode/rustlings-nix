// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() -> String {
        get_secret_recipe(); // just ignored, lol?
        String::from("sausage!")
    }
}

fn main() {
    println!("{}", sausage_factory::make_sausage());
}
