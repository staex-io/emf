#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod emf_contract {
    use ink::storage::{traits::StorageLayout, Mapping};

    #[ink(storage)]
    pub struct EmfContract {
        entities: Mapping<AccountId, Entity>,
        sub_entities: Mapping<AccountId, SubEntity>,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Entity {}

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct SubEntity {
        entity: AccountId,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub enum EmfError {
        EntityAlreadyExists,
        EntityNotFound,
        SubEntityAlreadyExists,
    }

    impl EmfContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                entities: Mapping::new(),
                sub_entities: Mapping::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn create_entity(&mut self) -> Result<(), EmfError> {
            if self.entities.get(self.env().caller()).is_some() {
                return Err(EmfError::EntityAlreadyExists);
            }
            self.entities.insert(self.env().caller(), &Entity {});
            Ok(())
        }

        #[ink(message)]
        pub fn create_sub_entity(&mut self, sub_entity: AccountId) -> Result<(), EmfError> {
            self.entities.get(self.env().caller()).ok_or(EmfError::EntityNotFound)?;
            if self.sub_entities.get(sub_entity).is_some() {
                return Err(EmfError::SubEntityAlreadyExists);
            }
            self.sub_entities.insert(
                sub_entity,
                &SubEntity {
                    entity: self.env().caller(),
                },
            );
            Ok(())
        }

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
        fn create_entity() {
            let mut emf_contract = EmfContract::new();

            let alice = default_accounts().alice;
            set_sender(alice);
            emf_contract.create_entity().unwrap();
            assert!(emf_contract.entities.get(alice).is_some());

            let bob = default_accounts().bob;
            set_sender(bob);
            emf_contract.create_entity().unwrap();
            assert!(emf_contract.entities.get(bob).is_some());

            let charlie = default_accounts().charlie;
            assert!(emf_contract.entities.get(charlie).is_none());
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn create_sub_entity() {
            let mut emf_contract = EmfContract::new();

            // Alice is an entity.
            let alice = default_accounts().alice;
            // Bob is a sub-entity for Alice.
            let bob = default_accounts().bob;

            // Test that we cannot create sub-entity before creating entity.
            set_sender(alice);
            let err = emf_contract.create_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::EntityNotFound, err,);

            // Create entity.
            set_sender(alice);
            emf_contract.create_entity().unwrap();

            // Test successful creation.
            set_sender(alice);
            emf_contract.create_sub_entity(bob).unwrap();
            assert!(emf_contract.sub_entities.get(bob).is_some());

            // Test that sub-entity cannot be created twice.
            set_sender(alice);
            let err = emf_contract.create_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityAlreadyExists, err);
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }
    }
}
