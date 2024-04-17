#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod emf_contract {
    use ink::prelude::{collections::VecDeque, string::String};
    use ink::storage::{traits::StorageLayout, Mapping};

    const AVG_DAYS_IN_MONTH: usize = 30;
    const SECS_IN_23H: u64 = 82_800;

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
        pub location: String,
        pub measurements: Measurements,
        pub spikes: Measurements,
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
        pub location: String,
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
        SubEntityAlreadyDeleted,
        StorageExceeded,
        MeasurementTooFast,
        Unknown,
    }

    impl From<ink_env::Error> for EmfError {
        fn from(value: ink_env::Error) -> Self {
            match value {
                ink_env::Error::BufferTooSmall => Self::StorageExceeded,
                _ => Self::Unknown,
            }
        }
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Measurement {
        pub value: u128,
        pub timestamp: u64,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Measurements(VecDeque<Measurement>);

    impl Default for Measurements {
        fn default() -> Self {
            Self(VecDeque::with_capacity(AVG_DAYS_IN_MONTH))
        }
    }

    impl Measurements {
        pub fn add(&mut self, value: u128, timestamp: u64) -> Result<bool, EmfError> {
            // If there are values in vector.
            // We need to check that previous value was not wrote
            // in less than 23h.
            // We check for 23h and not for 24h because we use not real timestamp
            // but block timestamp which is volatile from time to time.
            if !self.0.is_empty() {
                // Unwrap is ok because we checked length.
                let diff = self.0.back().unwrap().timestamp.checked_sub(timestamp);
                if diff.is_some() && diff.unwrap() < SECS_IN_23H {
                    return Err(EmfError::MeasurementTooFast);
                }
            }
            let mut cap_exceeded = false;
            if self.0.len() == AVG_DAYS_IN_MONTH {
                cap_exceeded = true;
                self.0.pop_front();
            }
            self.0.push_back(Measurement { value, timestamp });
            Ok(cap_exceeded)
        }
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
            self.entities.try_insert(self.env().caller(), &Entity {})?;
            self.env().emit_event(EntityCreated {
                entity: self.env().caller(),
            });
            Ok(())
        }

        #[ink(message)]
        pub fn create_sub_entity(
            &mut self,
            sub_entity: AccountId,
            location: String,
        ) -> Result<(), EmfError> {
            self.entities.get(self.env().caller()).ok_or(EmfError::EntityNotFound)?;
            if self.sub_entities.get(sub_entity).is_some() {
                return Err(EmfError::SubEntityAlreadyExists);
            }
            self.sub_entities.try_insert(
                sub_entity,
                &SubEntity {
                    entity: self.env().caller(),
                    location: location.clone(),
                    measurements: Measurements::default(),
                    spikes: Measurements::default(),
                    deleted: false,
                },
            )?;
            self.env().emit_event(SubEntityCreated {
                entity: self.env().caller(),
                sub_entity,
                location,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn delete_sub_entity(&mut self, sub_entity: AccountId) -> Result<(), EmfError> {
            self.entities.get(self.env().caller()).ok_or(EmfError::EntityNotFound)?;
            let sub_entity_record = self.load_sub_entity(sub_entity)?;
            if self.env().caller() != sub_entity_record.entity {
                return Err(EmfError::SubEntityBelongingFailed);
            }
            self.sub_entities.try_insert(
                sub_entity,
                &SubEntity {
                    entity: sub_entity_record.entity,
                    location: sub_entity_record.location,
                    measurements: sub_entity_record.measurements,
                    spikes: sub_entity_record.spikes,
                    deleted: true,
                },
            )?;
            self.env().emit_event(SubEntityDeleted {
                entity: sub_entity_record.entity,
                sub_entity,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn store_measurement(&mut self, value: u128) -> Result<(), EmfError> {
            let sub_entity_record = self.load_sub_entity(self.env().caller())?;
            let mut measurements = sub_entity_record.measurements;
            measurements.add(value, self.env().block_timestamp())?;
            self.sub_entities.try_insert(
                self.env().caller(),
                &SubEntity {
                    entity: sub_entity_record.entity,
                    location: sub_entity_record.location,
                    measurements,
                    spikes: sub_entity_record.spikes,
                    deleted: sub_entity_record.deleted,
                },
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn store_measurement_spike(&mut self, value: u128) -> Result<(), EmfError> {
            let sub_entity_record = self.load_sub_entity(self.env().caller())?;
            let mut spikes = sub_entity_record.spikes;
            spikes.add(value, self.env().block_timestamp())?;
            self.sub_entities.try_insert(
                self.env().caller(),
                &SubEntity {
                    entity: sub_entity_record.entity,
                    location: sub_entity_record.location,
                    measurements: sub_entity_record.measurements,
                    spikes,
                    deleted: sub_entity_record.deleted,
                },
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn check_sub_entity(&mut self) -> Result<(), EmfError> {
            let sub_entity_record = self.load_sub_entity(self.env().caller())?;
            todo!()
        }

        fn load_sub_entity(&self, sub_entity: AccountId) -> Result<SubEntity, EmfError> {
            let sub_entity_record =
                self.sub_entities.get(sub_entity).ok_or(EmfError::SubEntityNotFound)?;
            if sub_entity_record.deleted {
                return Err(EmfError::SubEntityAlreadyDeleted);
            }
            Ok(sub_entity_record)
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::{env::test::EmittedEvent, primitives::AccountId};

        use super::*;

        /// Default location for the sub-entity.
        const LOCATION: &str = "123,321";

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
            let err = emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap_err();
            assert_eq!(EmfError::EntityNotFound, err);

            // Create entity.
            emf_contract.create_entity().unwrap();

            // Test successful creation.
            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();
            assert!(emf_contract.sub_entities.get(bob).is_some());

            // Test that we don't have other entities.
            assert!(emf_contract.sub_entities.get(default_accounts().charlie).is_none());

            // Test that sub-entity cannot be created twice.
            let err = emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap_err();
            assert_eq!(EmfError::SubEntityAlreadyExists, err);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            // We have two events, one for entity created event and second
            // for sub-entity created event.
            assert_eq!(2, emitted_events.len());
            assert_entity_created_event(&emitted_events[0], alice);
            assert_sub_entity_created_event(&emitted_events[1], alice, bob, LOCATION.into());
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
            emf_contract.create_sub_entity(bob, String::new()).unwrap();
            assert!(emf_contract.sub_entities.get(bob).is_some());
            assert!(!emf_contract.sub_entities.get(bob).unwrap().deleted);

            // Test that we cannot delete sub-entity by random entity.
            set_sender(default_accounts().charlie);
            emf_contract.create_entity().unwrap();
            let err = emf_contract.delete_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityBelongingFailed, err);

            set_sender(alice);

            // Test successfully delete sub-entity.
            emf_contract.delete_sub_entity(bob).unwrap();
            assert!(emf_contract.sub_entities.get(bob).unwrap().deleted);

            // Test that we cannot delete sub-entity twice.
            let err = emf_contract.delete_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityAlreadyDeleted, err);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            /*
                Entity created for Alice and Charlie.
                Sub-entity created for Alice as Bob.
                Sub-entity deleted for Alice as Bob.
            */
            assert_eq!(4, emitted_events.len());
            assert_sub_entity_deleted_event(&emitted_events[3], alice, bob);
        }

        /// We test sub-entity measurements storage.
        #[ink::test]
        fn sub_entity_store_measurements() {
            generic_measurements_test(store_measurement, get_measurement)
        }

        /// We test sub-entity measurement spikes storage.
        #[ink::test]
        fn sub_entity_store_measurement_spikes() {
            generic_measurements_test(store_measurement_spike, get_spike)
        }

        fn generic_measurements_test<WriteFn, ReadFn>(write_fn: WriteFn, read_fn: ReadFn)
        where
            WriteFn: Fn(&mut EmfContract, u128) -> Result<(), EmfError>,
            ReadFn: Fn(&EmfContract, AccountId, usize) -> Measurement,
        {
            let mut emf_contract = EmfContract::new();

            let alice = default_accounts().alice;
            let bob = default_accounts().bob;

            set_sender(alice);
            emf_contract.create_entity().unwrap();

            // Check that we cannot store measurement without created sub-entity.
            set_sender(bob);
            let err = write_fn(&mut emf_contract, 1).unwrap_err();
            assert_eq!(EmfError::SubEntityNotFound, err);

            set_sender(alice);
            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();

            set_sender(bob);

            let mut timestamp = 1;

            set_timestamp(timestamp);
            write_fn(&mut emf_contract, 1).unwrap();
            assert_eq!(1, read_fn(&emf_contract, bob, 0).value);

            // We need to test that we store exactly 30 values and no more.
            for i in 2..32 {
                timestamp += SECS_IN_23H;
                set_timestamp(timestamp);
                write_fn(&mut emf_contract, i).unwrap();
            }
            assert_eq!(2, read_fn(&emf_contract, bob, 0).value);
            assert_eq!(31, read_fn(&emf_contract, bob, 29).value);
            assert_eq!(timestamp, read_fn(&emf_contract, bob, 29).timestamp);

            // Check write measurements too fast.
            let err = write_fn(&mut emf_contract, 99).unwrap_err();
            assert_eq!(EmfError::MeasurementTooFast, err);

            // Check that we can't write measurements to delete sub-entity.
            set_sender(alice);
            emf_contract.delete_sub_entity(bob).unwrap();
            set_sender(bob);
            let err = write_fn(&mut emf_contract, 111).unwrap_err();
            assert_eq!(EmfError::SubEntityAlreadyDeleted, err);
        }

        fn store_measurement(emf_contract: &mut EmfContract, value: u128) -> Result<(), EmfError> {
            emf_contract.store_measurement(value)
        }

        fn store_measurement_spike(
            emf_contract: &mut EmfContract,
            value: u128,
        ) -> Result<(), EmfError> {
            emf_contract.store_measurement_spike(value)
        }

        fn assert_entity_created_event(event: &EmittedEvent, entity: AccountId) {
            let evt = decode_event::<EntityCreated>(event);
            assert_eq!(entity, evt.entity);
        }

        fn assert_sub_entity_created_event(
            event: &EmittedEvent,
            entity: AccountId,
            sub_entity: AccountId,
            location: String,
        ) {
            let evt = decode_event::<SubEntityCreated>(event);
            assert_eq!(entity, evt.entity);
            assert_eq!(sub_entity, evt.sub_entity);
            assert_eq!(location, evt.location);
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

        fn get_measurement(
            emf_contract: &EmfContract,
            address: AccountId,
            index: usize,
        ) -> Measurement {
            let sub_entity = emf_contract.sub_entities.get(address).unwrap();
            let inner = sub_entity.measurements.0.get(index).unwrap();
            Measurement {
                value: inner.value,
                timestamp: inner.timestamp,
            }
        }

        fn get_spike(emf_contract: &EmfContract, address: AccountId, index: usize) -> Measurement {
            let sub_entity = emf_contract.sub_entities.get(address).unwrap();
            let inner = sub_entity.spikes.0.get(index).unwrap();
            Measurement {
                value: inner.value,
                timestamp: inner.timestamp,
            }
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

        fn set_timestamp(timestamp: u64) {
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(timestamp);
        }
    }
}
