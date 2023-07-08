#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        // Stores Declaration
        value: i32
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            //Contract Constructor
            Self {
                value: init_value
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            //Contract Constructor
            Self {
                value: 0
            }
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }


        #[ink(message)]
        pub fn get(&self) -> i32 {
            //Contract Message
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            //Test Your Contract
            let contract = Incrementer::default();

            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            //Test Your Contract
            let mut contract = Incrementer::new(42);

            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
        }
    }
}
