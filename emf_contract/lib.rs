#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod emf_contract {
    use ink::storage::{traits::StorageLayout, Mapping};

    #[ink(storage)]
    pub struct EmfContract {
        entities: Mapping<AccountId, Entity>,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Entity {}

    impl EmfContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                entities: Mapping::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn create_entity(&mut self) -> Result<(), String> {
            if self.entities.get(self.env().caller()).is_some() {
                return Err("entity is already exists".to_string());
            }
            self.entities.insert(self.env().caller(), &Entity {});
            Ok(())
        }

        #[ink(message)]
        pub fn create_sub_entity(&mut self) {}

        #[ink(message)]
        pub fn delete_sub_entity(&mut self) {}
    }

    #[cfg(test)]
    mod tests {
        use ink::primitives::AccountId;

        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let _emf_contract = EmfContract::default();
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut emf_contract = EmfContract::new();
            set_sender(default_accounts().alice);
            emf_contract.create_entity().unwrap();
            assert!(emf_contract.entities.get(default_accounts().alice).is_some());
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }
    }

    // /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    // ///
    // /// When running these you need to make sure that you:
    // /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    // /// - Are running a Substrate node which contains `pallet-contracts` in the background
    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// A helper function used for calling contract messages.
    //     use ink_e2e::ContractsBackend;

    //     /// The End-to-End test `Result` type.
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    //     /// We test that we can upload and instantiate the contract using its default constructor.
    //     #[ink_e2e::test]
    //     async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let mut constructor = EmfContractRef::default();

    //         // When
    //         let contract = client
    //             .instantiate("emf_contract", &ink_e2e::alice(), &mut constructor)
    //             .submit()
    //             .await
    //             .expect("instantiate failed");
    //         let call_builder = contract.call_builder::<EmfContract>();

    //         // Then
    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    //         assert!(!get_result.return_value());

    //         Ok(())
    //     }

    //     /// We test that we can read and write a value from the on-chain contract contract.
    //     #[ink_e2e::test]
    //     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let mut constructor = EmfContractRef::new(false);
    //         let contract = client
    //             .instantiate("emf_contract", &ink_e2e::bob(), &mut constructor)
    //             .submit()
    //             .await
    //             .expect("instantiate failed");
    //         let mut call_builder = contract.call_builder::<EmfContract>();

    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
    //         assert!(!get_result.return_value());

    //         // When
    //         let flip = call_builder.flip();
    //         let _flip_result =
    //             client.call(&ink_e2e::bob(), &flip).submit().await.expect("flip failed");

    //         // Then
    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
    //         assert!(get_result.return_value());

    //         Ok(())
    //     }
    // }
}
