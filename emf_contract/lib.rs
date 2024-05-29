#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod emf_contract {
    use core::ops::Deref;
    use core::time::Duration;

    use ink::prelude::{collections::VecDeque, string::String};
    use ink::storage::{traits::StorageLayout, Mapping};

    // Average days in month.
    const DAYS_IN_MONTH: u8 = 30;
    // 23 hours.
    const H23: Duration = Duration::from_secs(82_800);
    // 1 minute.
    const ONE_MINUTE: Duration = Duration::from_secs(60);

    // If time between spikes are more than this diff
    // we think that spike is new and we can save it as
    // something new.
    // 6 minutes.
    const TOO_MANY_SPIKES_TIME_DIFF: Duration = Duration::from_secs(360);
    // Actually it means we need 10 spikes to spawn too many spikes event.
    const TOO_MANY_SPIKES_COUNT: u8 = 10;

    // We store duration as seconds.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EmfDuration(u64);

    impl From<Duration> for EmfDuration {
        fn from(value: Duration) -> Self {
            Self(value.as_secs())
        }
    }

    // We store it as a unix timestamp milliseconds.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EmfSystemTime(u64);

    impl From<u64> for EmfSystemTime {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl From<u128> for EmfSystemTime {
        fn from(value: u128) -> Self {
            Self(value as u64)
        }
    }

    impl Deref for EmfSystemTime {
        type Target = u64;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    type MeasurementType = u128;
    type CertificateIndexType = u128;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Entity {}

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct SubEntity {
        pub entity: AccountId,
        pub location: String,
        pub measurements: BoundedVec<Measurement>,
        pub spikes: BoundedVec<Measurement>,
        pub deleted: bool,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Certificate {
        pub index: CertificateIndexType,

        pub entity: AccountId,
        pub sub_entity: AccountId,

        pub status: CertificateStatus,

        pub min_measurement: MeasurementType,
        pub max_measurement: MeasurementType,
        pub avg_measurement: MeasurementType,

        pub first_measurement_timestamp: EmfSystemTime,
        pub last_measurement_timestamp: EmfSystemTime,
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

    #[ink(event)]
    pub struct NewSpike {
        #[ink(topic)]
        pub entity: AccountId,
        #[ink(topic)]
        pub sub_entity: AccountId,
        pub value: MeasurementType,
    }

    #[ink(event)]
    pub struct TooManySpikes {
        #[ink(topic)]
        pub entity: AccountId,
        #[ink(topic)]
        pub sub_entity: AccountId,
    }

    #[ink(event)]
    pub struct CertificateReady {
        #[ink(topic)]
        pub entity: AccountId,
        #[ink(topic)]
        pub sub_entity: AccountId,
    }

    #[ink(event)]
    pub struct CertificateIssued {
        pub index: CertificateIndexType,
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
        NotEnoughRecords,
        CertificateNotFound,
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
    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub enum CertificateStatus {
        Ok,
        Bad,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Measurement {
        pub value: MeasurementType,
        pub timestamp: EmfSystemTime,
    }

    impl Measurement {
        pub fn new(value: MeasurementType, timestamp: EmfSystemTime) -> Self {
            Self { value, timestamp }
        }
    }

    impl TimeConscious for Measurement {
        fn timestamp(&self) -> EmfSystemTime {
            self.timestamp
        }
    }

    pub trait TimeConscious {
        fn timestamp(&self) -> EmfSystemTime;
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct BoundedVec<T>(VecDeque<T>)
    where
        T: TimeConscious;

    impl<T> Default for BoundedVec<T>
    where
        T: TimeConscious,
    {
        fn default() -> Self {
            Self(VecDeque::with_capacity(DAYS_IN_MONTH as usize))
        }
    }

    impl<T> BoundedVec<T>
    where
        T: TimeConscious,
    {
        pub fn add(
            &mut self,
            data: T,
            max_values: usize,
            min_time_diff: EmfDuration,
        ) -> Result<bool, EmfError> {
            // If there are values in vector.
            // We need to check that previous value was not wrote
            // in less than 23h.
            // We check for 23h and not for 24h because we use not real timestamp
            // but block timestamp which is volatile from time to time.
            if !self.0.is_empty() {
                // Unwrap is ok because we checked length.
                // Last unwrap is ok as new timestamp cannot be less than in storage.
                let diff: EmfDuration = Duration::from_millis(
                    data.timestamp().checked_sub(*self.0.back().unwrap().timestamp()).unwrap(),
                )
                .into();
                if diff < min_time_diff {
                    return Err(EmfError::MeasurementTooFast);
                }
            }
            if self.0.len() == max_values {
                self.0.pop_front();
            }
            let mut cap_reached = false;
            #[allow(clippy::arithmetic_side_effects)]
            if self.0.len() == max_values - 1 {
                cap_reached = true;
            }
            self.0.push_back(data);
            Ok(cap_reached)
        }
    }

    #[ink(storage)]
    pub struct EmfContract {
        max_measurement_value: MeasurementType,
        max_measurements_count: u8,
        min_time_between_measurements_to_save: EmfDuration,
        min_time_between_spikes_to_save: EmfDuration,
        // If nearest spikes time difference more that such time
        // we don't need to spawn too many spikes event.
        max_time_between_spikes_to_spawn: EmfDuration,
        min_spikes_count_to_spawn: u8,

        entities: Mapping<AccountId, Entity>,
        sub_entities: Mapping<AccountId, SubEntity>,

        current_certificate_index: CertificateIndexType,
        certificates: Mapping<CertificateIndexType, Certificate>,
    }

    impl Default for EmfContract {
        fn default() -> Self {
            EmfContract::new(
                10,
                DAYS_IN_MONTH,
                H23.into(),
                ONE_MINUTE.into(),
                TOO_MANY_SPIKES_TIME_DIFF.into(),
                TOO_MANY_SPIKES_COUNT,
            )
        }
    }

    impl EmfContract {
        #[ink(constructor)]
        pub fn new(
            max_measurement_value: MeasurementType,
            max_measurements_count: u8,
            min_time_between_measurements_to_save: EmfDuration,
            min_time_between_spikes_to_save: EmfDuration,
            max_time_between_spikes_to_spawn: EmfDuration,
            min_spikes_count_to_spawn: u8,
        ) -> Self {
            Self {
                max_measurement_value,
                max_measurements_count,
                min_time_between_measurements_to_save,
                min_time_between_spikes_to_save,
                max_time_between_spikes_to_spawn,
                min_spikes_count_to_spawn,

                entities: Mapping::new(),
                sub_entities: Mapping::new(),

                current_certificate_index: 0,
                certificates: Mapping::new(),
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
                    measurements: BoundedVec::default(),
                    spikes: BoundedVec::default(),
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
        pub fn store_measurement(&mut self, value: MeasurementType) -> Result<(), EmfError> {
            let sub_entity_record = self.load_sub_entity(self.env().caller())?;
            let mut measurements = sub_entity_record.measurements;
            let cap_reached = measurements.add(
                Measurement::new(value, self.env().block_timestamp().into()),
                self.max_measurements_count as usize,
                self.min_time_between_measurements_to_save,
            )?;
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
            if cap_reached {
                self.env().emit_event(CertificateReady {
                    entity: sub_entity_record.entity,
                    sub_entity: self.env().caller(),
                });
            }
            Ok(())
        }

        #[ink(message)]
        pub fn store_measurement_spike(&mut self, value: MeasurementType) -> Result<(), EmfError> {
            let sub_entity_record = self.load_sub_entity(self.env().caller())?;
            let mut spikes = sub_entity_record.spikes;

            spikes.add(
                Measurement::new(value, self.env().block_timestamp().into()),
                self.max_measurements_count as usize,
                self.min_time_between_spikes_to_save,
            )?;

            let is_too_many_spikes = if spikes.0.len() >= self.min_spikes_count_to_spawn as usize {
                // It means in last 10 spikes we have at least one diff between two
                // nearest spikes which is more than TOO_MANY_SPIKES_TIME_DIFF.
                let mut interval_broken = false;
                #[allow(clippy::arithmetic_side_effects)]
                for i in (spikes.0.len() - self.min_spikes_count_to_spawn as usize + 1
                    ..spikes.0.len())
                    .rev()
                {
                    let spike_lt = &spikes.0[i - 1].timestamp;
                    let spike_rt = &spikes.0[i].timestamp;
                    let diff = **spike_rt - **spike_lt;
                    let diff = Duration::from_millis(diff);
                    let diff: EmfDuration = diff.into();
                    if diff > self.max_time_between_spikes_to_spawn {
                        ink_env::debug_println!(
                            "interval is broken: {:?}, {:?}, {:?}, {:?}",
                            spike_rt,
                            spike_lt,
                            diff,
                            self.max_time_between_spikes_to_spawn
                        );
                        interval_broken = true;
                        break;
                    } else {
                        ink_env::debug_println!(
                            "interval is NOT broken: {:?}, {:?}, {:?}, {:?}",
                            spike_rt,
                            spike_lt,
                            diff,
                            self.max_time_between_spikes_to_spawn
                        );
                    }
                }
                !interval_broken
            } else {
                ink_env::debug_println!("not enough spikes to calculate too many spikes data");
                false
            };

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

            self.env().emit_event(NewSpike {
                entity: sub_entity_record.entity,
                sub_entity: self.env().caller(),
                value,
            });
            if is_too_many_spikes {
                ink_env::debug_println!("too many spikes event was emitted");
                self.env().emit_event(TooManySpikes {
                    entity: sub_entity_record.entity,
                    sub_entity: self.env().caller(),
                });
            }

            Ok(())
        }

        #[ink(message)]
        pub fn check_sub_entity(&self, sub_entity: AccountId) -> Result<bool, EmfError> {
            let sub_entity_record = self.load_sub_entity(sub_entity)?;
            if sub_entity_record.measurements.0.len() != self.max_measurements_count as usize {
                return Err(EmfError::NotEnoughRecords);
            }
            for measurement in sub_entity_record.measurements.0 {
                if measurement.value > self.max_measurement_value {
                    return Ok(false);
                }
            }
            Ok(true)
        }

        #[ink(message)]
        pub fn issue_certificate(
            &mut self,
            sub_entity: AccountId,
        ) -> Result<CertificateIndexType, EmfError> {
            let sub_entity_record = self.load_sub_entity(sub_entity)?;
            if self.env().caller() != sub_entity_record.entity {
                return Err(EmfError::SubEntityBelongingFailed);
            }
            if sub_entity_record.measurements.0.len() != self.max_measurements_count as usize {
                return Err(EmfError::NotEnoughRecords);
            }

            self.current_certificate_index =
                self.current_certificate_index.checked_add(1).ok_or(EmfError::Unknown)?;
            let index = self.current_certificate_index;

            let mut status = CertificateStatus::Ok;
            let mut min_measurement: MeasurementType = MeasurementType::MAX;
            let mut max_measurement: MeasurementType = MeasurementType::MIN;
            let mut avg_measurement: MeasurementType = 0;
            for measurement in &sub_entity_record.measurements.0 {
                if measurement.value > self.max_measurement_value {
                    status = CertificateStatus::Bad;
                }
                if measurement.value < min_measurement {
                    min_measurement = measurement.value;
                }
                if measurement.value > max_measurement {
                    max_measurement = measurement.value;
                }
                avg_measurement =
                    avg_measurement.checked_add(measurement.value).ok_or(EmfError::Unknown)?;
            }
            avg_measurement = avg_measurement
                .checked_div(self.max_measurements_count as u128)
                .ok_or(EmfError::Unknown)?;

            let first_measurement_timestamp = sub_entity_record.measurements.0[0].timestamp;
            #[allow(clippy::arithmetic_side_effects)]
            let last_measurement_timestamp = sub_entity_record.measurements.0
                [self.max_measurements_count as usize - 1]
                .timestamp;

            self.certificates.try_insert(
                index,
                &Certificate {
                    index,
                    entity: self.env().caller(),
                    sub_entity,
                    status,
                    min_measurement,
                    max_measurement,
                    avg_measurement,
                    first_measurement_timestamp,
                    last_measurement_timestamp,
                },
            )?;

            self.sub_entities.try_insert(
                sub_entity,
                &SubEntity {
                    entity: sub_entity_record.entity,
                    location: sub_entity_record.location,
                    measurements: BoundedVec::default(),
                    spikes: BoundedVec::default(),
                    deleted: sub_entity_record.deleted,
                },
            )?;

            self.env().emit_event(CertificateIssued {
                index,
                entity: self.env().caller(),
                sub_entity,
            });

            Ok(index)
        }

        #[ink(message)]
        pub fn fetch_certificate(
            &mut self,
            index: CertificateIndexType,
        ) -> Result<Certificate, EmfError> {
            self.certificates.get(index).ok_or(EmfError::CertificateNotFound)
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
            let mut emf_contract = EmfContract::default();

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
            let mut emf_contract = EmfContract::default();

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
            let mut emf_contract = EmfContract::default();

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

        /// We test everything about spikes events.
        #[ink::test]
        fn test_spike_events() {
            let mut emf_contract = EmfContract::default();

            let alice = default_accounts().alice;
            let bob = default_accounts().bob;

            set_sender(alice);
            emf_contract.create_entity().unwrap();
            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();

            set_sender(bob);

            let mut timestamp = 1;
            set_timestamp(timestamp);
            emf_contract.store_measurement_spike(99).unwrap();
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            // Two events to create entity and sub-entity.
            // And event about spike.
            assert_eq!(3, emitted_events.len());

            timestamp += TOO_MANY_SPIKES_TIME_DIFF.as_millis();
            set_timestamp(timestamp);
            emf_contract.store_measurement_spike(111).unwrap();
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            // One more event for measurement spike.
            assert_eq!(3 + 1, emitted_events.len());

            // We need more 8 spikes in a row to spawn too many spikes event.
            for _ in 0..8 {
                timestamp += TOO_MANY_SPIKES_TIME_DIFF.as_millis();
                set_timestamp(timestamp);
                emf_contract.store_measurement_spike(111).unwrap();
            }
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            // +8 spikes events and +1 too many spike event.
            assert_eq!(4 + 8 + 1, emitted_events.len());

            // If there are more than 6m passed from last spike
            // we don't have too many spikes event.
            timestamp += (TOO_MANY_SPIKES_TIME_DIFF + TOO_MANY_SPIKES_TIME_DIFF).as_millis() + 1;
            set_timestamp(timestamp);
            emf_contract.store_measurement_spike(111).unwrap();
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(13 + 1, emitted_events.len());

            // Test that we cannot store too many same events on-chain.
            timestamp += ONE_MINUTE.as_millis() - 1;
            set_timestamp(timestamp);
            let err = emf_contract.store_measurement_spike(111).unwrap_err();
            assert_eq!(EmfError::MeasurementTooFast, err);
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(14, emitted_events.len());

            // Clean spikes.
            let old = emf_contract.sub_entities.get(bob).unwrap();
            emf_contract.sub_entities.insert(
                bob,
                &SubEntity {
                    entity: old.entity,
                    location: old.location,
                    measurements: old.measurements,
                    spikes: BoundedVec::default(),
                    deleted: old.deleted,
                },
            );

            // Test that we can store up to 10 spikes but in the middle
            // diff in time between some spikes are more than allowed.
            for _ in 0..8 {
                timestamp += TOO_MANY_SPIKES_TIME_DIFF.as_millis();
                set_timestamp(timestamp);
                emf_contract.store_measurement_spike(111).unwrap();
            }
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(14 + 8, emitted_events.len());

            // Save spike with time than max allowed.
            timestamp += (TOO_MANY_SPIKES_TIME_DIFF + TOO_MANY_SPIKES_TIME_DIFF).as_millis() + 1;
            set_timestamp(timestamp);
            emf_contract.store_measurement_spike(111).unwrap();
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(22 + 1, emitted_events.len());

            timestamp += TOO_MANY_SPIKES_TIME_DIFF.as_millis();
            set_timestamp(timestamp);
            emf_contract.store_measurement_spike(111).unwrap();
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            // So we do not have too many spikes event.
            assert_eq!(23 + 1, emitted_events.len());
        }

        /// We test check sub-entity smart contract method.
        #[ink::test]
        fn test_check_sub_entities_general_flow() {
            let mut emf_contract = EmfContract::default();

            let alice = default_accounts().alice;
            let bob = default_accounts().bob;

            set_sender(alice);
            emf_contract.create_entity().unwrap();

            let err = emf_contract.check_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityNotFound, err);

            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();

            let err = emf_contract.check_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::NotEnoughRecords, err);

            set_sender(bob);
            let mut timestamp = 0;
            for _ in 0..30 {
                timestamp += H23.as_millis();
                set_timestamp(timestamp);
                emf_contract.store_measurement(2).unwrap();
            }

            set_sender(alice);

            assert!(emf_contract.check_sub_entity(bob).unwrap());

            emf_contract.delete_sub_entity(bob).unwrap();

            let err = emf_contract.check_sub_entity(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityAlreadyDeleted, err);
        }

        /// We test check sub-entity smart contract method in bad case.
        #[ink::test]
        fn test_check_sub_entities_bad_case() {
            let mut emf_contract = EmfContract::default();

            let alice = default_accounts().alice;
            let bob = default_accounts().bob;

            set_sender(alice);
            emf_contract.create_entity().unwrap();
            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();

            set_sender(bob);
            let mut timestamp = 0;
            for _ in 0..30 {
                timestamp += H23.as_millis();
                set_timestamp(timestamp);
                emf_contract.store_measurement(11).unwrap();
            }

            set_sender(alice);
            assert!(!emf_contract.check_sub_entity(bob).unwrap());
        }

        /// We test successful certification issue with ok status.
        #[ink::test]
        fn test_issue_certificate_ok() {
            let mut emf_contract = EmfContract::default();

            let alice = default_accounts().alice;
            let bob = default_accounts().bob;

            set_sender(alice);
            emf_contract.create_entity().unwrap();
            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();

            set_sender(default_accounts().charlie);
            let err = emf_contract.issue_certificate(bob).unwrap_err();
            assert_eq!(EmfError::SubEntityBelongingFailed, err);

            set_sender(alice);

            let err = emf_contract.issue_certificate(bob).unwrap_err();
            assert_eq!(EmfError::NotEnoughRecords, err);

            set_sender(bob);
            let mut timestamp = 0;
            for i in 0..30 {
                let measurement: MeasurementType = if i < 10 {
                    4
                } else if (10..20).contains(&i) {
                    5
                } else {
                    6
                };
                timestamp += H23.as_millis();
                set_timestamp(timestamp);
                emf_contract.store_measurement(measurement).unwrap();
            }
            emf_contract.store_measurement_spike(111).unwrap();

            set_sender(alice);
            assert_eq!(0, emf_contract.current_certificate_index);
            let index = emf_contract.issue_certificate(bob).unwrap();
            assert_eq!(1, emf_contract.current_certificate_index);

            let certificate = emf_contract.certificates.get(index).unwrap();
            assert_eq!(certificate.index, 1);
            assert_eq!(certificate.entity, alice);
            assert_eq!(certificate.sub_entity, bob);
            assert_eq!(certificate.status, CertificateStatus::Ok);
            assert_eq!(certificate.min_measurement, 4);
            assert_eq!(certificate.max_measurement, 6);
            assert_eq!(certificate.avg_measurement, 5);
            assert_eq!(
                certificate.first_measurement_timestamp,
                (timestamp - H23.as_millis() * 29).into()
            );
            assert_eq!(certificate.last_measurement_timestamp, timestamp.into());

            assert!(emf_contract.sub_entities.get(bob).unwrap().measurements.0.is_empty());
            assert!(emf_contract.sub_entities.get(bob).unwrap().spikes.0.is_empty());

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(5, emitted_events.len());
            let certificated_issued: CertificateIssued = decode_event(&emitted_events[4]);
            assert_eq!(certificated_issued.index, 1);
            assert_eq!(certificated_issued.entity, alice);
            assert_eq!(certificated_issued.sub_entity, bob);
        }

        /// We test successful certification issue with bad status.
        #[ink::test]
        fn test_issue_certificate_bad() {
            let mut emf_contract = EmfContract::default();

            let alice = default_accounts().alice;
            let bob = default_accounts().bob;

            set_sender(alice);
            emf_contract.create_entity().unwrap();
            emf_contract.create_sub_entity(bob, LOCATION.into()).unwrap();

            set_sender(bob);
            let mut timestamp = 0;
            for _ in 0..30 {
                timestamp += H23.as_millis();
                set_timestamp(timestamp);
                emf_contract.store_measurement(55).unwrap();
            }
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(3, emitted_events.len());
            let certificated_ready: CertificateReady = decode_event(&emitted_events[2]);
            assert_eq!(certificated_ready.entity, alice);
            assert_eq!(certificated_ready.sub_entity, bob);

            set_sender(alice);
            emf_contract.issue_certificate(bob).unwrap();

            let certificate = emf_contract.certificates.get(1).unwrap();
            assert_eq!(certificate.status, CertificateStatus::Bad);
        }

        fn generic_measurements_test<WriteFn, ReadFn>(write_fn: WriteFn, read_fn: ReadFn)
        where
            WriteFn: Fn(&mut EmfContract, MeasurementType) -> Result<(), EmfError>,
            ReadFn: Fn(&EmfContract, AccountId, usize) -> Measurement,
        {
            let mut emf_contract = EmfContract::default();

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
                timestamp += H23.as_millis();
                set_timestamp(timestamp);
                write_fn(&mut emf_contract, i).unwrap();
            }
            assert_eq!(2, read_fn(&emf_contract, bob, 0).value);
            assert_eq!(31, read_fn(&emf_contract, bob, 29).value);
            assert_eq!(timestamp, *read_fn(&emf_contract, bob, 29).timestamp as u128);

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

        fn store_measurement(
            emf_contract: &mut EmfContract,
            value: MeasurementType,
        ) -> Result<(), EmfError> {
            emf_contract.store_measurement(value)
        }

        fn store_measurement_spike(
            emf_contract: &mut EmfContract,
            value: MeasurementType,
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

        // We use u128 because we use milliseconds.
        fn set_timestamp(timestamp: u128) {
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(timestamp as u64);
        }
    }
}
