#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod emf_contract {
    use ink::storage::{traits::StorageLayout, Mapping};

    #[ink(storage)]
    pub struct EmfContract {
        pub entities: Mapping<AccountId, Entity>,
        pub sub_entities: Mapping<AccountId, SubEntity>,
    }

    impl Default for EmfContract {
        fn default() -> Self {
            EmfContract::new()
        }
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Entity {}

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct SubEntity {
        pub entity: AccountId,
        pub deleted: bool,
    }

    #[ink(event)]
    pub struct EntityCreated {
        #[ink(topic)]
        pub entity: AccountId,
    }

    #[ink(event)]
    pub struct SubEntityCreated {
        #[ink(topic)]
        pub entity: AccountId,
        #[ink(topic)]
        pub sub_entity: AccountId,
    }

    #[ink(event)]
    pub struct SubEntityDeleted {
        #[ink(topic)]
        pub entity: AccountId,
        #[ink(topic)]
        pub sub_entity: AccountId,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub enum EmfError {
        EntityAlreadyExists,
        EntityNotFound,
        SubEntityAlreadyExists,
        SubEntityNotFound,
        SubEntityBelongingFailed,
    }

    impl EmfContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                entities: Mapping::new(),
                sub_entities: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn create_entity(&mut self) -> Result<(), EmfError> {
            if self.entities.get(self.env().caller()).is_some() {
                return Err(EmfError::EntityAlreadyExists);
            }
            self.entities.insert(self.env().caller(), &Entity {});
            self.env().emit_event(EntityCreated {
                entity: self.env().caller(),
            });
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
                    deleted: false,
                },
            );
            self.env().emit_event(SubEntityCreated {
                entity: self.env().caller(),
                sub_entity,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn delete_sub_entity(&mut self, sub_entity: AccountId) -> Result<(), EmfError> {
            self.entities.get(self.env().caller()).ok_or(EmfError::EntityNotFound)?;
            let sub_entity_record =
                self.sub_entities.get(sub_entity).ok_or(EmfError::SubEntityNotFound)?;
            if self.env().caller() != sub_entity_record.entity {
                return Err(EmfError::SubEntityBelongingFailed);
            }
            self.sub_entities.insert(
                sub_entity,
                &SubEntity {
                    entity: sub_entity_record.entity,
                    deleted: true,
                },
            );
            self.env().emit_event(SubEntityDeleted {
                entity: sub_entity_record.entity,
                sub_entity,
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::{env::test::EmittedEvent, primitives::AccountId};

        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let _emf_contract = EmfContract::default();
        }

        /// We test entity creation.
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

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(2, emitted_events.len());
            assert_entity_created_event(&emitted_events[0], alice);
            assert_entity_created_event(&emitted_events[1], bob);
        }

        /// We test sub-entity creation.
        #[ink::test]
        fn create_sub_entity() {
            let mut emf_contract = EmfContract::new();

            // Alice is an entity.
            let alice = default_accounts().alice;
            // Bob is a sub-entity for Alice.
            let bob = default_accounts().bob;

            set_sender(alice);

            // Test that we cannot create sub-entity before creating entity.
            let err = emf_contract.create_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::EntityNotFound, err);

            // Create entity.
            emf_contract.create_entity().unwrap();

            // Test successful creation.
            emf_contract.create_sub_entity(bob).unwrap();
            assert!(emf_contract.sub_entities.get(bob).is_some());

            // Test that we don't have other entities.
            assert!(emf_contract.sub_entities.get(default_accounts().charlie).is_none());

            // Test that sub-entity cannot be created twice.
            let err = emf_contract.create_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityAlreadyExists, err);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            // We have two events, one for entity created event and second
            // for sub-entity created event.
            assert_eq!(2, emitted_events.len());
            assert_entity_created_event(&emitted_events[0], alice);
            assert_sub_entity_created_event(&emitted_events[1], alice, bob);
        }

        /// We test sub-entity deletion.
        #[ink::test]
        fn delete_sub_entity() {
            let mut emf_contract = EmfContract::new();

            // Alice is an entity.
            let alice = default_accounts().alice;
            // Bob is a sub-entity for Alice.
            let bob = default_accounts().bob;

            set_sender(alice);

            // Test that we cannot delete sub-entity if entity is not exists.
            let err = emf_contract.delete_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::EntityNotFound, err);

            // Create entity.
            emf_contract.create_entity().unwrap();

            // Test that we cannot delete sub-entity if it is not exists.
            let err = emf_contract.delete_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityNotFound, err);

            // Test successful sub-entity creation.
            emf_contract.create_sub_entity(bob).unwrap();
            assert!(emf_contract.sub_entities.get(bob).is_some());
            assert!(!emf_contract.sub_entities.get(bob).unwrap().deleted);

            // Test that we cannot delete sub-entity by random entity.
            set_sender(default_accounts().charlie);
            emf_contract.create_entity().unwrap();
            let err = emf_contract.delete_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityBelongingFailed, err);

            // Test successfully delete sub-entity.
            set_sender(alice);
            emf_contract.delete_sub_entity(bob).unwrap();
            assert!(emf_contract.sub_entities.get(bob).unwrap().deleted);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            /*
                Entity created for Alice and Charlie.
                Sub-entity created for Alice as Bob.
                Sub-entity deleted for Alice as Bob.
            */
            assert_eq!(4, emitted_events.len());
            assert_sub_entity_deleted_event(&emitted_events[3], alice, bob);
        }

        fn assert_entity_created_event(event: &EmittedEvent, entity: AccountId) {
            let evt = decode_event::<EntityCreated>(event);
            assert_eq!(entity, evt.entity);
        }

        fn assert_sub_entity_created_event(
            event: &EmittedEvent,
            entity: AccountId,
            sub_entity: AccountId,
        ) {
            let evt = decode_event::<SubEntityCreated>(event);
            assert_eq!(entity, evt.entity);
            assert_eq!(sub_entity, evt.sub_entity);
        }

        fn assert_sub_entity_deleted_event(
            event: &EmittedEvent,
            entity: AccountId,
            sub_entity: AccountId,
        ) {
            let evt = decode_event::<SubEntityDeleted>(event);
            assert_eq!(entity, evt.entity);
            assert_eq!(sub_entity, evt.sub_entity);
        }

        fn decode_event<T>(event: &EmittedEvent) -> T
        where
            T: ink::scale::Decode,
        {
            <T as ink::scale::Decode>::decode(&mut &event.data[..]).unwrap()
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }
    }
}
