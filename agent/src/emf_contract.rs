#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 10usize] = [
        "System",
        "RandomnessCollectiveFlip",
        "Utility",
        "Timestamp",
        "Balances",
        "Authorship",
        "TransactionPayment",
        "Sudo",
        "Contracts",
        "Assets",
    ];
    pub static RUNTIME_APIS: [&str; 11usize] = [
        "Core",
        "Metadata",
        "BlockBuilder",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "SessionKeys",
        "AccountNonceApi",
        "TransactionPaymentApi",
        "TransactionPaymentCallApi",
        "ContractsApi",
        "GenesisBuilder",
    ];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::contracts_node_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::contracts_node_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::subxt_core::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn core(&self) -> core::Core {
                core::Core
            }
            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }
            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }
            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }
            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }
            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }
            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }
            pub fn transaction_payment_api(
                &self,
            ) -> transaction_payment_api::TransactionPaymentApi {
                transaction_payment_api::TransactionPaymentApi
            }
            pub fn transaction_payment_call_api(
                &self,
            ) -> transaction_payment_call_api::TransactionPaymentCallApi {
                transaction_payment_call_api::TransactionPaymentCallApi
            }
            pub fn contracts_api(&self) -> contracts_api::ContractsApi {
                contracts_api::ContractsApi
            }
            pub fn genesis_builder(&self) -> genesis_builder::GenesisBuilder {
                genesis_builder::GenesisBuilder
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            pub struct Core;
            impl Core {
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Version,
                    types::version::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "version",
                        types::Version {},
                        [
                            76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
                            125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
                            116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
                            6u8,
                        ],
                    )
                }
                pub fn execute_block(
                    &self,
                    block: types::execute_block::Block,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ExecuteBlock,
                    types::execute_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "execute_block",
                        types::ExecuteBlock { block },
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }
                pub fn initialize_block(
                    &self,
                    header: types::initialize_block::Header,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::InitializeBlock,
                    types::initialize_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "initialize_block",
                        types::InitializeBlock { header },
                        [
                            132u8, 169u8, 113u8, 112u8, 80u8, 139u8, 113u8, 35u8, 41u8, 81u8, 36u8,
                            35u8, 37u8, 202u8, 29u8, 207u8, 205u8, 229u8, 145u8, 7u8, 133u8, 94u8,
                            25u8, 108u8, 233u8, 86u8, 234u8, 29u8, 236u8, 57u8, 56u8, 186u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod version {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_version::RuntimeVersion;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Version {}
                pub mod execute_block {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExecuteBlock {
                    pub block: execute_block::Block,
                }
                pub mod initialize_block {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::ExtrinsicInclusionMode;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InitializeBlock {
                    pub header: initialize_block::Header,
                }
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            pub struct Metadata;
            impl Metadata {
                pub fn metadata(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Metadata,
                    types::metadata::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata",
                        types::Metadata {},
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }
                pub fn metadata_at_version(
                    &self,
                    version: types::metadata_at_version::Version,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::MetadataAtVersion,
                    types::metadata_at_version::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        types::MetadataAtVersion { version },
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::MetadataVersions,
                    types::metadata_versions::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata_versions",
                        types::MetadataVersions {},
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod metadata {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_core::OpaqueMetadata;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Metadata {}
                pub mod metadata_at_version {
                    use super::runtime_types;
                    pub type Version = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct MetadataAtVersion {
                    pub version: metadata_at_version::Version,
                }
                pub mod metadata_versions {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct MetadataVersions {}
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            pub struct BlockBuilder;
            impl BlockBuilder {
                pub fn apply_extrinsic(
                    &self,
                    extrinsic: types::apply_extrinsic::Extrinsic,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ApplyExtrinsic,
                    types::apply_extrinsic::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        types::ApplyExtrinsic { extrinsic },
                        [
                            72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
                            223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
                            97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
                        ],
                    )
                }
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::FinalizeBlock,
                    types::finalize_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        types::FinalizeBlock {},
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }
                pub fn inherent_extrinsics(
                    &self,
                    inherent: types::inherent_extrinsics::Inherent,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::InherentExtrinsics,
                    types::inherent_extrinsics::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        types::InherentExtrinsics { inherent },
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }
                pub fn check_inherents(
                    &self,
                    block: types::check_inherents::Block,
                    data: types::check_inherents::Data,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CheckInherents,
                    types::check_inherents::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        types::CheckInherents { block, data },
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod apply_extrinsic {
                    use super::runtime_types;
                    pub type Extrinsic = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < :: core :: result :: Result < () , runtime_types :: sp_runtime :: DispatchError > , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ApplyExtrinsic {
                    pub extrinsic: apply_extrinsic::Extrinsic,
                }
                pub mod finalize_block {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FinalizeBlock {}
                pub mod inherent_extrinsics {
                    use super::runtime_types;
                    pub type Inherent = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InherentExtrinsics {
                    pub inherent: inherent_extrinsics::Inherent,
                }
                pub mod check_inherents {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    pub type Data = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_inherents::CheckInherentsResult;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CheckInherents {
                    pub block: check_inherents::Block,
                    pub data: check_inherents::Data,
                }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                pub fn validate_transaction(
                    &self,
                    source: types::validate_transaction::Source,
                    tx: types::validate_transaction::Tx,
                    block_hash: types::validate_transaction::BlockHash,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ValidateTransaction,
                    types::validate_transaction::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        types::ValidateTransaction {
                            source,
                            tx,
                            block_hash,
                        },
                        [
                            196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
                            143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
                            162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod validate_transaction {
                    use super::runtime_types;
                    pub type Source =
                        runtime_types::sp_runtime::transaction_validity::TransactionSource;
                    pub type Tx = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < runtime_types :: sp_runtime :: transaction_validity :: ValidTransaction , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidateTransaction {
                    pub source: validate_transaction::Source,
                    pub tx: validate_transaction::Tx,
                    pub block_hash: validate_transaction::BlockHash,
                }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                pub fn offchain_worker(
                    &self,
                    header: types::offchain_worker::Header,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::OffchainWorker,
                    types::offchain_worker::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        types::OffchainWorker { header },
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod offchain_worker {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct OffchainWorker {
                    pub header: offchain_worker::Header,
                }
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            pub struct SessionKeys;
            impl SessionKeys {
                pub fn generate_session_keys(
                    &self,
                    seed: types::generate_session_keys::Seed,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GenerateSessionKeys,
                    types::generate_session_keys::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        types::GenerateSessionKeys { seed },
                        [
                            96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
                            102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
                            190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
                        ],
                    )
                }
                pub fn decode_session_keys(
                    &self,
                    encoded: types::decode_session_keys::Encoded,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::DecodeSessionKeys,
                    types::decode_session_keys::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        types::DecodeSessionKeys { encoded },
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod generate_session_keys {
                    use super::runtime_types;
                    pub type Seed = ::core::option::Option<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GenerateSessionKeys {
                    pub seed: generate_session_keys::Seed,
                }
                pub mod decode_session_keys {
                    use super::runtime_types;
                    pub type Encoded =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<(
                                ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                                runtime_types::sp_core::crypto::KeyTypeId,
                            )>,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DecodeSessionKeys {
                    pub encoded: decode_session_keys::Encoded,
                }
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                pub fn account_nonce(
                    &self,
                    account: types::account_nonce::Account,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::AccountNonce,
                    types::account_nonce::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        types::AccountNonce { account },
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod account_nonce {
                    use super::runtime_types;
                    pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u32;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountNonce {
                    pub account: account_nonce::Account,
                }
            }
        }
        pub mod transaction_payment_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentApi;
            impl TransactionPaymentApi {
                pub fn query_info(
                    &self,
                    uxt: types::query_info::Uxt,
                    len: types::query_info::Len,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryInfo,
                    types::query_info::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_info",
                        types::QueryInfo { uxt, len },
                        [
                            56u8, 30u8, 174u8, 34u8, 202u8, 24u8, 177u8, 189u8, 145u8, 36u8, 1u8,
                            156u8, 98u8, 209u8, 178u8, 49u8, 198u8, 23u8, 150u8, 173u8, 35u8,
                            205u8, 147u8, 129u8, 42u8, 22u8, 69u8, 3u8, 129u8, 8u8, 196u8, 139u8,
                        ],
                    )
                }
                pub fn query_fee_details(
                    &self,
                    uxt: types::query_fee_details::Uxt,
                    len: types::query_fee_details::Len,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryFeeDetails,
                    types::query_fee_details::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_fee_details",
                        types::QueryFeeDetails { uxt, len },
                        [
                            117u8, 60u8, 137u8, 159u8, 237u8, 252u8, 216u8, 238u8, 232u8, 1u8,
                            100u8, 152u8, 26u8, 185u8, 145u8, 125u8, 68u8, 189u8, 4u8, 30u8, 125u8,
                            7u8, 196u8, 153u8, 235u8, 51u8, 219u8, 108u8, 185u8, 254u8, 100u8,
                            201u8,
                        ],
                    )
                }
                pub fn query_weight_to_fee(
                    &self,
                    weight: types::query_weight_to_fee::Weight,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryWeightToFee,
                    types::query_weight_to_fee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            206u8, 243u8, 189u8, 83u8, 231u8, 244u8, 247u8, 52u8, 126u8, 208u8,
                            224u8, 5u8, 163u8, 108u8, 254u8, 114u8, 214u8, 156u8, 227u8, 217u8,
                            211u8, 198u8, 121u8, 164u8, 110u8, 54u8, 181u8, 146u8, 50u8, 146u8,
                            146u8, 23u8,
                        ],
                    )
                }
                pub fn query_length_to_fee(
                    &self,
                    length: types::query_length_to_fee::Length,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryLengthToFee,
                    types::query_length_to_fee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            92u8, 132u8, 29u8, 119u8, 66u8, 11u8, 196u8, 224u8, 129u8, 23u8, 249u8,
                            12u8, 32u8, 28u8, 92u8, 50u8, 188u8, 101u8, 203u8, 229u8, 248u8, 216u8,
                            130u8, 150u8, 212u8, 161u8, 81u8, 254u8, 116u8, 89u8, 162u8, 48u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod query_info {
                    use super::runtime_types;
                    pub type Uxt = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                                ::core::primitive::u128,
                                runtime_types::sp_weights::weight_v2::Weight,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryInfo {
                    pub uxt: query_info::Uxt,
                    pub len: query_info::Len,
                }
                pub mod query_fee_details {
                    use super::runtime_types;
                    pub type Uxt = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: contracts_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::FeeDetails<
                                ::core::primitive::u128,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryFeeDetails {
                    pub uxt: query_fee_details::Uxt,
                    pub len: query_fee_details::Len,
                }
                pub mod query_weight_to_fee {
                    use super::runtime_types;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryWeightToFee {
                    pub weight: query_weight_to_fee::Weight,
                }
                pub mod query_length_to_fee {
                    use super::runtime_types;
                    pub type Length = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryLengthToFee {
                    pub length: query_length_to_fee::Length,
                }
            }
        }
        pub mod transaction_payment_call_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentCallApi;
            impl TransactionPaymentCallApi {
                pub fn query_call_info(
                    &self,
                    call: types::query_call_info::Call,
                    len: types::query_call_info::Len,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryCallInfo,
                    types::query_call_info::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_info",
                        types::QueryCallInfo { call, len },
                        [
                            183u8, 49u8, 195u8, 181u8, 220u8, 5u8, 253u8, 211u8, 114u8, 14u8,
                            252u8, 18u8, 172u8, 144u8, 5u8, 1u8, 82u8, 130u8, 1u8, 4u8, 113u8,
                            60u8, 2u8, 130u8, 142u8, 192u8, 89u8, 15u8, 201u8, 63u8, 89u8, 230u8,
                        ],
                    )
                }
                pub fn query_call_fee_details(
                    &self,
                    call: types::query_call_fee_details::Call,
                    len: types::query_call_fee_details::Len,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryCallFeeDetails,
                    types::query_call_fee_details::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_fee_details",
                        types::QueryCallFeeDetails { call, len },
                        [
                            111u8, 36u8, 208u8, 106u8, 35u8, 158u8, 52u8, 194u8, 55u8, 232u8, 92u8,
                            20u8, 122u8, 162u8, 109u8, 84u8, 72u8, 23u8, 43u8, 120u8, 76u8, 96u8,
                            191u8, 216u8, 149u8, 183u8, 226u8, 244u8, 212u8, 32u8, 106u8, 23u8,
                        ],
                    )
                }
                pub fn query_weight_to_fee(
                    &self,
                    weight: types::query_weight_to_fee::Weight,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryWeightToFee,
                    types::query_weight_to_fee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            117u8, 91u8, 94u8, 22u8, 248u8, 212u8, 15u8, 23u8, 97u8, 116u8, 64u8,
                            228u8, 83u8, 123u8, 87u8, 77u8, 97u8, 7u8, 98u8, 181u8, 6u8, 165u8,
                            114u8, 141u8, 164u8, 113u8, 126u8, 88u8, 174u8, 171u8, 224u8, 35u8,
                        ],
                    )
                }
                pub fn query_length_to_fee(
                    &self,
                    length: types::query_length_to_fee::Length,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryLengthToFee,
                    types::query_length_to_fee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            246u8, 40u8, 4u8, 160u8, 152u8, 94u8, 170u8, 53u8, 205u8, 122u8, 5u8,
                            69u8, 70u8, 25u8, 128u8, 156u8, 119u8, 134u8, 116u8, 147u8, 14u8,
                            164u8, 65u8, 140u8, 86u8, 13u8, 250u8, 218u8, 89u8, 95u8, 234u8, 228u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod query_call_info {
                    use super::runtime_types;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                                ::core::primitive::u128,
                                runtime_types::sp_weights::weight_v2::Weight,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryCallInfo {
                    pub call: query_call_info::Call,
                    pub len: query_call_info::Len,
                }
                pub mod query_call_fee_details {
                    use super::runtime_types;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::FeeDetails<
                                ::core::primitive::u128,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryCallFeeDetails {
                    pub call: query_call_fee_details::Call,
                    pub len: query_call_fee_details::Len,
                }
                pub mod query_weight_to_fee {
                    use super::runtime_types;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryWeightToFee {
                    pub weight: query_weight_to_fee::Weight,
                }
                pub mod query_length_to_fee {
                    use super::runtime_types;
                    pub type Length = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryLengthToFee {
                    pub length: query_length_to_fee::Length,
                }
            }
        }
        pub mod contracts_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct ContractsApi;
            impl ContractsApi {
                pub fn call(
                    &self,
                    origin: types::call::Origin,
                    dest: types::call::Dest,
                    value: types::call::Value,
                    gas_limit: types::call::GasLimit,
                    storage_deposit_limit: types::call::StorageDepositLimit,
                    input_data: types::call::InputData,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Call,
                    types::call::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "ContractsApi",
                        "call",
                        types::Call {
                            origin,
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            input_data,
                        },
                        [
                            254u8, 227u8, 151u8, 218u8, 119u8, 180u8, 147u8, 79u8, 71u8, 58u8,
                            141u8, 5u8, 9u8, 84u8, 146u8, 242u8, 70u8, 125u8, 176u8, 236u8, 246u8,
                            31u8, 161u8, 82u8, 17u8, 227u8, 21u8, 93u8, 189u8, 219u8, 224u8, 240u8,
                        ],
                    )
                }
                pub fn instantiate(
                    &self,
                    origin: types::instantiate::Origin,
                    value: types::instantiate::Value,
                    gas_limit: types::instantiate::GasLimit,
                    storage_deposit_limit: types::instantiate::StorageDepositLimit,
                    code: types::instantiate::Code,
                    data: types::instantiate::Data,
                    salt: types::instantiate::Salt,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Instantiate,
                    types::instantiate::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "ContractsApi",
                        "instantiate",
                        types::Instantiate {
                            origin,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            219u8, 247u8, 184u8, 148u8, 252u8, 182u8, 63u8, 12u8, 85u8, 60u8, 38u8,
                            144u8, 98u8, 30u8, 62u8, 10u8, 106u8, 126u8, 105u8, 171u8, 130u8,
                            250u8, 240u8, 15u8, 0u8, 183u8, 213u8, 111u8, 173u8, 35u8, 26u8, 163u8,
                        ],
                    )
                }
                pub fn upload_code(
                    &self,
                    origin: types::upload_code::Origin,
                    code: types::upload_code::Code,
                    storage_deposit_limit: types::upload_code::StorageDepositLimit,
                    determinism: types::upload_code::Determinism,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::UploadCode,
                    types::upload_code::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "ContractsApi",
                        "upload_code",
                        types::UploadCode {
                            origin,
                            code,
                            storage_deposit_limit,
                            determinism,
                        },
                        [
                            231u8, 114u8, 110u8, 91u8, 142u8, 108u8, 124u8, 161u8, 13u8, 8u8,
                            127u8, 134u8, 133u8, 152u8, 137u8, 67u8, 59u8, 78u8, 120u8, 75u8,
                            172u8, 211u8, 23u8, 227u8, 90u8, 203u8, 204u8, 129u8, 142u8, 226u8,
                            32u8, 213u8,
                        ],
                    )
                }
                pub fn get_storage(
                    &self,
                    address: types::get_storage::Address,
                    key: types::get_storage::Key,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetStorage,
                    types::get_storage::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "ContractsApi",
                        "get_storage",
                        types::GetStorage { address, key },
                        [
                            49u8, 103u8, 100u8, 132u8, 135u8, 193u8, 145u8, 48u8, 154u8, 78u8,
                            41u8, 43u8, 81u8, 109u8, 146u8, 199u8, 6u8, 111u8, 184u8, 102u8, 46u8,
                            76u8, 174u8, 148u8, 106u8, 184u8, 131u8, 137u8, 194u8, 98u8, 179u8,
                            45u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod call {
                    use super::runtime_types;
                    pub type Origin = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Dest = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit =
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>;
                    pub type StorageDepositLimit = ::core::option::Option<::core::primitive::u128>;
                    pub type InputData =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_contracts::primitives::ContractResult<
                                ::core::result::Result<
                                    runtime_types::pallet_contracts::primitives::ExecReturnValue,
                                    runtime_types::sp_runtime::DispatchError,
                                >,
                                ::core::primitive::u128,
                                runtime_types::frame_system::EventRecord<
                                    runtime_types::contracts_node_runtime::RuntimeEvent,
                                    ::subxt::ext::subxt_core::utils::H256,
                                >,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Call {
                    pub origin: call::Origin,
                    pub dest: call::Dest,
                    pub value: call::Value,
                    pub gas_limit: call::GasLimit,
                    pub storage_deposit_limit: call::StorageDepositLimit,
                    pub input_data: call::InputData,
                }
                pub mod instantiate {
                    use super::runtime_types;
                    pub type Origin = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit =
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>;
                    pub type StorageDepositLimit = ::core::option::Option<::core::primitive::u128>;
                    pub type Code = runtime_types::pallet_contracts::primitives::Code<
                        ::subxt::ext::subxt_core::utils::H256,
                    >;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Salt =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types :: pallet_contracts :: primitives :: ContractResult < :: core :: result :: Result < runtime_types :: pallet_contracts :: primitives :: InstantiateReturnValue < :: subxt :: ext :: subxt_core :: utils :: AccountId32 > , runtime_types :: sp_runtime :: DispatchError > , :: core :: primitive :: u128 , runtime_types :: frame_system :: EventRecord < runtime_types :: contracts_node_runtime :: RuntimeEvent , :: subxt :: ext :: subxt_core :: utils :: H256 > > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Instantiate {
                    pub origin: instantiate::Origin,
                    pub value: instantiate::Value,
                    pub gas_limit: instantiate::GasLimit,
                    pub storage_deposit_limit: instantiate::StorageDepositLimit,
                    pub code: instantiate::Code,
                    pub data: instantiate::Data,
                    pub salt: instantiate::Salt,
                }
                pub mod upload_code {
                    use super::runtime_types;
                    pub type Origin = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type StorageDepositLimit = ::core::option::Option<::core::primitive::u128>;
                    pub type Determinism = runtime_types::pallet_contracts::wasm::Determinism;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            runtime_types::pallet_contracts::primitives::CodeUploadReturnValue<
                                ::subxt::ext::subxt_core::utils::H256,
                                ::core::primitive::u128,
                            >,
                            runtime_types::sp_runtime::DispatchError,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct UploadCode {
                    pub origin: upload_code::Origin,
                    pub code: upload_code::Code,
                    pub storage_deposit_limit: upload_code::StorageDepositLimit,
                    pub determinism: upload_code::Determinism,
                }
                pub mod get_storage {
                    use super::runtime_types;
                    pub type Address = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Key = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            ::core::option::Option<
                                ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            >,
                            runtime_types::pallet_contracts::primitives::ContractAccessError,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetStorage {
                    pub address: get_storage::Address,
                    pub key: get_storage::Key,
                }
            }
        }
        pub mod genesis_builder {
            use super::root_mod;
            use super::runtime_types;
            pub struct GenesisBuilder;
            impl GenesisBuilder {
                pub fn create_default_config(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CreateDefaultConfig,
                    types::create_default_config::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "create_default_config",
                        types::CreateDefaultConfig {},
                        [
                            238u8, 5u8, 139u8, 81u8, 184u8, 155u8, 221u8, 118u8, 190u8, 76u8,
                            229u8, 67u8, 132u8, 89u8, 83u8, 80u8, 56u8, 171u8, 169u8, 64u8, 123u8,
                            20u8, 129u8, 159u8, 28u8, 135u8, 84u8, 52u8, 192u8, 98u8, 104u8, 214u8,
                        ],
                    )
                }
                pub fn build_config(
                    &self,
                    json: types::build_config::Json,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::BuildConfig,
                    types::build_config::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "build_config",
                        types::BuildConfig { json },
                        [
                            6u8, 98u8, 68u8, 125u8, 157u8, 26u8, 107u8, 86u8, 213u8, 227u8, 26u8,
                            229u8, 122u8, 161u8, 229u8, 114u8, 123u8, 192u8, 66u8, 231u8, 148u8,
                            175u8, 5u8, 185u8, 248u8, 88u8, 40u8, 122u8, 230u8, 209u8, 170u8,
                            254u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod create_default_config {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CreateDefaultConfig {}
                pub mod build_config {
                    use super::runtime_types;
                    pub type Json =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            (),
                            ::subxt::ext::subxt_core::alloc::string::String,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BuildConfig {
                    pub json: build_config::Json,
                }
            }
        }
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn contracts(&self) -> contracts::constants::ConstantsApi {
            contracts::constants::ConstantsApi
        }
        pub fn assets(&self) -> assets::constants::ConstantsApi {
            assets::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi {
            authorship::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn contracts(&self) -> contracts::storage::StorageApi {
            contracts::storage::StorageApi
        }
        pub fn assets(&self) -> assets::storage::StorageApi {
            assets::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn utility(&self) -> utility::calls::TransactionApi {
            utility::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn contracts(&self) -> contracts::calls::TransactionApi {
            contracts::calls::TransactionApi
        }
        pub fn assets(&self) -> assets::calls::TransactionApi {
            assets::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::ext::subxt_core::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                232u8, 190u8, 122u8, 25u8, 183u8, 236u8, 109u8, 196u8, 203u8, 79u8, 143u8, 131u8,
                136u8, 73u8, 149u8, 237u8, 212u8, 126u8, 41u8, 99u8, 66u8, 236u8, 135u8, 164u8,
                193u8, 9u8, 57u8, 140u8, 52u8, 142u8, 60u8, 151u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::frame_system::pallet::Error;
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AuthorizeUpgrade {
                    pub code_hash: authorize_upgrade::CodeHash,
                }
                pub mod authorize_upgrade {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AuthorizeUpgradeWithoutChecks {
                    pub code_hash: authorize_upgrade_without_checks::CodeHash,
                }
                pub mod authorize_upgrade_without_checks {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ApplyAuthorizedUpgrade {
                    pub code: apply_authorized_upgrade::Code,
                }
                pub mod apply_authorized_upgrade {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "apply_authorized_upgrade";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Remark>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetHeapPages>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCodeWithoutChecks>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillPrefix>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemarkWithEvent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
                pub fn authorize_upgrade(
                    &self,
                    code_hash: types::authorize_upgrade::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade",
                        types::AuthorizeUpgrade { code_hash },
                        [
                            4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
                            254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
                            58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
                            172u8,
                        ],
                    )
                }
                pub fn authorize_upgrade_without_checks(
                    &self,
                    code_hash: types::authorize_upgrade_without_checks::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AuthorizeUpgradeWithoutChecks,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade_without_checks",
                        types::AuthorizeUpgradeWithoutChecks { code_hash },
                        [
                            126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
                            136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
                            197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
                            233u8,
                        ],
                    )
                }
                pub fn apply_authorized_upgrade(
                    &self,
                    code: types::apply_authorized_upgrade::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ApplyAuthorizedUpgrade,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "apply_authorized_upgrade",
                        types::ApplyAuthorizedUpgrade { code },
                        [
                            232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
                            156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
                            32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpdated;
            impl ::subxt::ext::subxt_core::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Hash = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct UpgradeAuthorized {
                pub code_hash: upgrade_authorized::CodeHash,
                pub check_version: upgrade_authorized::CheckVersion,
            }
            pub mod upgrade_authorized {
                use super::runtime_types;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type CheckVersion = ::core::primitive::bool;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod inherents_applied {
                    use super::runtime_types;
                    pub type InherentsApplied = ::core::primitive::bool;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u32;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::contracts_node_runtime::RuntimeEvent,
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
                pub mod authorized_upgrade {
                    use super::runtime_types;
                    pub type AuthorizedUpgrade =
                        runtime_types::frame_system::CodeUpgradeAuthorization;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        (),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicCount",
                        (),
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                pub fn inherents_applied(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inherents_applied::InherentsApplied,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "InherentsApplied",
                        (),
                        [
                            132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
                            223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
                            109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
                        ],
                    )
                }
                pub fn block_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_weight::BlockWeight,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockWeight",
                        (),
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        (),
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        (),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                pub fn block_hash(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::block_hash::Param0,
                    >,
                    types::block_hash::BlockHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        (),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::extrinsic_data::Param0,
                    >,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                pub fn number(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::number::Number,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Number",
                        (),
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::parent_hash::ParentHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ParentHash",
                        (),
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                pub fn digest(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::digest::Digest,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Digest",
                        (),
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                pub fn events(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::events::Events,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Events",
                        (),
                        [
                            0u8, 136u8, 84u8, 39u8, 20u8, 223u8, 28u8, 146u8, 178u8, 9u8, 127u8,
                            12u8, 254u8, 139u8, 19u8, 160u8, 188u8, 247u8, 3u8, 42u8, 28u8, 205u8,
                            228u8, 51u8, 18u8, 59u8, 22u8, 133u8, 115u8, 232u8, 112u8, 221u8,
                        ],
                    )
                }
                pub fn event_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_count::EventCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventCount",
                        (),
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_topics::EventTopics,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        (),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                pub fn event_topics(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::event_topics::Param0,
                    >,
                    types::event_topics::EventTopics,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        (),
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        (),
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        (),
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::execution_phase::ExecutionPhase,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExecutionPhase",
                        (),
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorized_upgrade::AuthorizedUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AuthorizedUpgrade",
                        (),
                        [
                            165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
                            169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
                            214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockWeights,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                pub fn block_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockLength,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn db_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_weights::RuntimeDbWeight,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod random_material {
                    use super::runtime_types;
                    pub type RandomMaterial =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn random_material(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::random_material::RandomMaterial,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        (),
                        [
                            195u8, 232u8, 244u8, 162u8, 110u8, 137u8, 66u8, 57u8, 51u8, 221u8,
                            143u8, 38u8, 51u8, 183u8, 105u8, 245u8, 175u8, 13u8, 33u8, 192u8, 53u8,
                            16u8, 161u8, 76u8, 219u8, 177u8, 144u8, 192u8, 96u8, 166u8, 117u8,
                            247u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod utility {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_utility::pallet::Error;
        pub type Call = runtime_types::pallet_utility::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Batch {
                    pub calls: batch::Calls,
                }
                pub mod batch {
                    use super::runtime_types;
                    pub type Calls = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::contracts_node_runtime::RuntimeCall,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Batch {
                    const PALLET: &'static str = "Utility";
                    const CALL: &'static str = "batch";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AsDerivative {
                    pub index: as_derivative::Index,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<as_derivative::Call>,
                }
                pub mod as_derivative {
                    use super::runtime_types;
                    pub type Index = ::core::primitive::u16;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AsDerivative {
                    const PALLET: &'static str = "Utility";
                    const CALL: &'static str = "as_derivative";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BatchAll {
                    pub calls: batch_all::Calls,
                }
                pub mod batch_all {
                    use super::runtime_types;
                    pub type Calls = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::contracts_node_runtime::RuntimeCall,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for BatchAll {
                    const PALLET: &'static str = "Utility";
                    const CALL: &'static str = "batch_all";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DispatchAs {
                    pub as_origin:
                        ::subxt::ext::subxt_core::alloc::boxed::Box<dispatch_as::AsOrigin>,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<dispatch_as::Call>,
                }
                pub mod dispatch_as {
                    use super::runtime_types;
                    pub type AsOrigin = runtime_types::contracts_node_runtime::OriginCaller;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DispatchAs {
                    const PALLET: &'static str = "Utility";
                    const CALL: &'static str = "dispatch_as";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceBatch {
                    pub calls: force_batch::Calls,
                }
                pub mod force_batch {
                    use super::runtime_types;
                    pub type Calls = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::contracts_node_runtime::RuntimeCall,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceBatch {
                    const PALLET: &'static str = "Utility";
                    const CALL: &'static str = "force_batch";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WithWeight {
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<with_weight::Call>,
                    pub weight: with_weight::Weight,
                }
                pub mod with_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for WithWeight {
                    const PALLET: &'static str = "Utility";
                    const CALL: &'static str = "with_weight";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn batch(
                    &self,
                    calls: types::batch::Calls,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Batch>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Utility",
                        "batch",
                        types::Batch { calls },
                        [
                            77u8, 150u8, 50u8, 13u8, 51u8, 20u8, 32u8, 180u8, 21u8, 65u8, 139u8,
                            164u8, 146u8, 87u8, 227u8, 157u8, 9u8, 140u8, 203u8, 161u8, 173u8,
                            226u8, 252u8, 12u8, 234u8, 195u8, 44u8, 29u8, 120u8, 10u8, 203u8,
                            159u8,
                        ],
                    )
                }
                pub fn as_derivative(
                    &self,
                    index: types::as_derivative::Index,
                    call: types::as_derivative::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AsDerivative>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Utility",
                        "as_derivative",
                        types::AsDerivative {
                            index,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            61u8, 51u8, 151u8, 251u8, 140u8, 15u8, 53u8, 172u8, 158u8, 124u8,
                            246u8, 41u8, 78u8, 219u8, 182u8, 45u8, 111u8, 206u8, 26u8, 24u8, 247u8,
                            192u8, 60u8, 47u8, 149u8, 46u8, 236u8, 241u8, 142u8, 251u8, 46u8,
                            239u8,
                        ],
                    )
                }
                pub fn batch_all(
                    &self,
                    calls: types::batch_all::Calls,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::BatchAll>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Utility",
                        "batch_all",
                        types::BatchAll { calls },
                        [
                            201u8, 38u8, 228u8, 83u8, 182u8, 196u8, 212u8, 58u8, 217u8, 11u8,
                            155u8, 35u8, 115u8, 74u8, 173u8, 209u8, 67u8, 245u8, 115u8, 85u8,
                            182u8, 9u8, 139u8, 169u8, 111u8, 84u8, 244u8, 193u8, 188u8, 98u8, 64u8,
                            34u8,
                        ],
                    )
                }
                pub fn dispatch_as(
                    &self,
                    as_origin: types::dispatch_as::AsOrigin,
                    call: types::dispatch_as::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::DispatchAs>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Utility",
                        "dispatch_as",
                        types::DispatchAs {
                            as_origin: ::subxt::ext::subxt_core::alloc::boxed::Box::new(as_origin),
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            47u8, 102u8, 133u8, 42u8, 224u8, 254u8, 82u8, 0u8, 76u8, 220u8, 248u8,
                            47u8, 218u8, 173u8, 51u8, 67u8, 116u8, 237u8, 193u8, 173u8, 38u8,
                            130u8, 178u8, 69u8, 184u8, 177u8, 127u8, 38u8, 205u8, 12u8, 139u8,
                            222u8,
                        ],
                    )
                }
                pub fn force_batch(
                    &self,
                    calls: types::force_batch::Calls,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceBatch>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Utility",
                        "force_batch",
                        types::ForceBatch { calls },
                        [
                            70u8, 136u8, 14u8, 139u8, 119u8, 84u8, 179u8, 93u8, 95u8, 134u8, 187u8,
                            51u8, 148u8, 99u8, 204u8, 147u8, 244u8, 125u8, 58u8, 206u8, 14u8,
                            208u8, 39u8, 30u8, 183u8, 219u8, 60u8, 44u8, 135u8, 220u8, 99u8, 131u8,
                        ],
                    )
                }
                pub fn with_weight(
                    &self,
                    call: types::with_weight::Call,
                    weight: types::with_weight::Weight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::WithWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Utility",
                        "with_weight",
                        types::WithWeight {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                            weight,
                        },
                        [
                            53u8, 62u8, 188u8, 67u8, 199u8, 106u8, 88u8, 194u8, 253u8, 139u8,
                            216u8, 144u8, 185u8, 10u8, 87u8, 225u8, 237u8, 73u8, 75u8, 207u8, 51u8,
                            238u8, 54u8, 218u8, 202u8, 255u8, 23u8, 76u8, 213u8, 120u8, 249u8,
                            11u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct BatchInterrupted {
                pub index: batch_interrupted::Index,
                pub error: batch_interrupted::Error,
            }
            pub mod batch_interrupted {
                use super::runtime_types;
                pub type Index = ::core::primitive::u32;
                pub type Error = runtime_types::sp_runtime::DispatchError;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct BatchCompleted;
            impl ::subxt::ext::subxt_core::events::StaticEvent for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct BatchCompletedWithErrors;
            impl ::subxt::ext::subxt_core::events::StaticEvent for BatchCompletedWithErrors {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompletedWithErrors";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ItemCompleted;
            impl ::subxt::ext::subxt_core::events::StaticEvent for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ItemFailed {
                pub error: item_failed::Error,
            }
            pub mod item_failed {
                use super::runtime_types;
                pub type Error = runtime_types::sp_runtime::DispatchError;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ItemFailed {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct DispatchedAs {
                pub result: dispatched_as::Result,
            }
            pub mod dispatched_as {
                use super::runtime_types;
                pub type Result =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn batched_calls_limit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Utility",
                        "batched_calls_limit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Set {
                    #[codec(compact)]
                    pub now: set::Now,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set(
                    &self,
                    now: types::set::Now,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Set>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod now {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                pub mod did_update {
                    use super::runtime_types;
                    pub type DidUpdate = ::core::primitive::bool;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn now(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::now::Now,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "Now",
                        (),
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                pub fn did_update(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::did_update::DidUpdate,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "DidUpdate",
                        (),
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type NewFree = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceAdjustTotalIssuance {
                    pub direction: force_adjust_total_issuance::Direction,
                    #[codec(compact)]
                    pub delta: force_adjust_total_issuance::Delta,
                }
                pub mod force_adjust_total_issuance {
                    use super::runtime_types;
                    pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
                    pub type Delta = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_adjust_total_issuance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAllowDeath>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAll>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceUnreserve>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpgradeAccounts>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSetBalance>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
                pub fn force_adjust_total_issuance(
                    &self,
                    direction: types::force_adjust_total_issuance::Direction,
                    delta: types::force_adjust_total_issuance::Delta,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ForceAdjustTotalIssuance,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_adjust_total_issuance",
                        types::ForceAdjustTotalIssuance { direction, delta },
                        [
                            208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
                            190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
                            176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
                            202u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct TotalIssuanceForced {
                pub old: total_issuance_forced::Old,
                pub new: total_issuance_forced::New,
            }
            pub mod total_issuance_forced {
                use super::runtime_types;
                pub type Old = ::core::primitive::u128;
                pub type New = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TotalIssuanceForced {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "TotalIssuanceForced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u128;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u128;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::contracts_node_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_issuance::TotalIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "TotalIssuance",
                        (),
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "InactiveIssuance",
                        (),
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        (),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::locks::Locks,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        (),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                pub fn locks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::locks::Param0,
                    >,
                    types::locks::Locks,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reserves::Reserves,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        (),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                pub fn reserves(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::reserves::Param0,
                    >,
                    types::reserves::Reserves,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::holds::Holds,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        (),
                        [
                            103u8, 123u8, 15u8, 82u8, 63u8, 13u8, 111u8, 80u8, 97u8, 230u8, 203u8,
                            129u8, 200u8, 147u8, 55u8, 64u8, 135u8, 165u8, 129u8, 143u8, 54u8,
                            191u8, 10u8, 34u8, 73u8, 199u8, 250u8, 131u8, 38u8, 2u8, 142u8, 204u8,
                        ],
                    )
                }
                pub fn holds(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::holds::Param0,
                    >,
                    types::holds::Holds,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            103u8, 123u8, 15u8, 82u8, 63u8, 13u8, 111u8, 80u8, 97u8, 230u8, 203u8,
                            129u8, 200u8, 147u8, 55u8, 64u8, 135u8, 165u8, 129u8, 143u8, 54u8,
                            191u8, 10u8, 34u8, 73u8, 199u8, 250u8, 131u8, 38u8, 2u8, 142u8, 204u8,
                        ],
                    )
                }
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::freezes::Freezes,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        (),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                pub fn freezes(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::freezes::Param0,
                    >,
                    types::freezes::Freezes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn max_locks(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_freezes(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod author {
                    use super::runtime_types;
                    pub type Author = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn author(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::author::Author,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Authorship",
                        "Author",
                        (),
                        [
                            247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
                            220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
                            61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
                            18u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct TransactionFeePaid {
                pub who: transaction_fee_paid::Who,
                pub actual_fee: transaction_fee_paid::ActualFee,
                pub tip: transaction_fee_paid::Tip,
            }
            pub mod transaction_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type ActualFee = ::core::primitive::u128;
                pub type Tip = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod next_fee_multiplier {
                    use super::runtime_types;
                    pub type NextFeeMultiplier =
                        runtime_types::sp_arithmetic::fixed_point::FixedU128;
                }
                pub mod storage_version {
                    use super::runtime_types;
                    pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_fee_multiplier::NextFeeMultiplier,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        (),
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::storage_version::StorageVersion,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        (),
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u8,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Sudo {
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo::Call>,
                }
                pub mod sudo {
                    use super::runtime_types;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SudoUncheckedWeight {
                    pub call:
                        ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_unchecked_weight::Call>,
                    pub weight: sudo_unchecked_weight::Weight,
                }
                pub mod sudo_unchecked_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetKey {
                    pub new: set_key::New,
                }
                pub mod set_key {
                    use super::runtime_types;
                    pub type New = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SudoAs {
                    pub who: sudo_as::Who,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_as::Call>,
                }
                pub mod sudo_as {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Call = runtime_types::contracts_node_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RemoveKey;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "remove_key";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn sudo(
                    &self,
                    call: types::sudo::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Sudo>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            190u8, 48u8, 62u8, 79u8, 138u8, 96u8, 21u8, 42u8, 211u8, 204u8, 147u8,
                            20u8, 9u8, 6u8, 156u8, 60u8, 246u8, 176u8, 89u8, 32u8, 184u8, 41u8,
                            215u8, 46u8, 229u8, 32u8, 74u8, 61u8, 194u8, 6u8, 121u8, 24u8,
                        ],
                    )
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: types::sudo_unchecked_weight::Call,
                    weight: types::sudo_unchecked_weight::Weight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoUncheckedWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                            weight,
                        },
                        [
                            199u8, 209u8, 93u8, 144u8, 250u8, 128u8, 14u8, 250u8, 239u8, 154u8,
                            164u8, 176u8, 197u8, 74u8, 172u8, 32u8, 57u8, 225u8, 94u8, 94u8, 251u8,
                            5u8, 18u8, 87u8, 100u8, 5u8, 22u8, 103u8, 31u8, 239u8, 243u8, 170u8,
                        ],
                    )
                }
                pub fn set_key(
                    &self,
                    new: types::set_key::New,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                pub fn sudo_as(
                    &self,
                    who: types::sudo_as::Who,
                    call: types::sudo_as::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoAs>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            224u8, 216u8, 108u8, 171u8, 69u8, 174u8, 205u8, 197u8, 212u8, 154u8,
                            91u8, 163u8, 108u8, 219u8, 142u8, 130u8, 175u8, 174u8, 253u8, 33u8,
                            182u8, 187u8, 191u8, 105u8, 171u8, 118u8, 178u8, 147u8, 205u8, 116u8,
                            123u8, 75u8,
                        ],
                    )
                }
                pub fn remove_key(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "remove_key",
                        types::RemoveKey {},
                        [
                            133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
                            25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
                            191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
                            79u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct KeyChanged {
                pub old: key_changed::Old,
                pub new: key_changed::New,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type Old = ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
                pub type New = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct KeyRemoved;
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyRemoved {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod key {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn key(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key::Key,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sudo",
                        "Key",
                        (),
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod contracts {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_contracts::pallet::Error;
        pub type Call = runtime_types::pallet_contracts::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CallOldWeight {
                    pub dest: call_old_weight::Dest,
                    #[codec(compact)]
                    pub value: call_old_weight::Value,
                    #[codec(compact)]
                    pub gas_limit: call_old_weight::GasLimit,
                    pub storage_deposit_limit: call_old_weight::StorageDepositLimit,
                    pub data: call_old_weight::Data,
                }
                pub mod call_old_weight {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for CallOldWeight {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "call_old_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InstantiateWithCodeOldWeight {
                    #[codec(compact)]
                    pub value: instantiate_with_code_old_weight::Value,
                    #[codec(compact)]
                    pub gas_limit: instantiate_with_code_old_weight::GasLimit,
                    pub storage_deposit_limit:
                        instantiate_with_code_old_weight::StorageDepositLimit,
                    pub code: instantiate_with_code_old_weight::Code,
                    pub data: instantiate_with_code_old_weight::Data,
                    pub salt: instantiate_with_code_old_weight::Salt,
                }
                pub mod instantiate_with_code_old_weight {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Salt =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for InstantiateWithCodeOldWeight {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate_with_code_old_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InstantiateOldWeight {
                    #[codec(compact)]
                    pub value: instantiate_old_weight::Value,
                    #[codec(compact)]
                    pub gas_limit: instantiate_old_weight::GasLimit,
                    pub storage_deposit_limit: instantiate_old_weight::StorageDepositLimit,
                    pub code_hash: instantiate_old_weight::CodeHash,
                    pub data: instantiate_old_weight::Data,
                    pub salt: instantiate_old_weight::Salt,
                }
                pub mod instantiate_old_weight {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Salt =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for InstantiateOldWeight {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate_old_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct UploadCode {
                    pub code: upload_code::Code,
                    pub storage_deposit_limit: upload_code::StorageDepositLimit,
                    pub determinism: upload_code::Determinism,
                }
                pub mod upload_code {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Determinism = runtime_types::pallet_contracts::wasm::Determinism;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UploadCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "upload_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RemoveCode {
                    pub code_hash: remove_code::CodeHash,
                }
                pub mod remove_code {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "remove_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetCode {
                    pub dest: set_code::Dest,
                    pub code_hash: set_code::CodeHash,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Call {
                    pub dest: call::Dest,
                    #[codec(compact)]
                    pub value: call::Value,
                    pub gas_limit: call::GasLimit,
                    pub storage_deposit_limit: call::StorageDepositLimit,
                    pub data: call::Data,
                }
                pub mod call {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = runtime_types::sp_weights::weight_v2::Weight;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Call {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "call";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InstantiateWithCode {
                    #[codec(compact)]
                    pub value: instantiate_with_code::Value,
                    pub gas_limit: instantiate_with_code::GasLimit,
                    pub storage_deposit_limit: instantiate_with_code::StorageDepositLimit,
                    pub code: instantiate_with_code::Code,
                    pub data: instantiate_with_code::Data,
                    pub salt: instantiate_with_code::Salt,
                }
                pub mod instantiate_with_code {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = runtime_types::sp_weights::weight_v2::Weight;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Salt =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for InstantiateWithCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate_with_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Instantiate {
                    #[codec(compact)]
                    pub value: instantiate::Value,
                    pub gas_limit: instantiate::GasLimit,
                    pub storage_deposit_limit: instantiate::StorageDepositLimit,
                    pub code_hash: instantiate::CodeHash,
                    pub data: instantiate::Data,
                    pub salt: instantiate::Salt,
                }
                pub mod instantiate {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = runtime_types::sp_weights::weight_v2::Weight;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Salt =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Instantiate {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Migrate {
                    pub weight_limit: migrate::WeightLimit,
                }
                pub mod migrate {
                    use super::runtime_types;
                    pub type WeightLimit = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Migrate {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "migrate";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn call_old_weight(
                    &self,
                    dest: types::call_old_weight::Dest,
                    value: types::call_old_weight::Value,
                    gas_limit: types::call_old_weight::GasLimit,
                    storage_deposit_limit: types::call_old_weight::StorageDepositLimit,
                    data: types::call_old_weight::Data,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::CallOldWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "call_old_weight",
                        types::CallOldWeight {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        },
                        [
                            159u8, 201u8, 54u8, 189u8, 207u8, 238u8, 0u8, 63u8, 0u8, 188u8, 150u8,
                            113u8, 13u8, 9u8, 199u8, 250u8, 77u8, 35u8, 174u8, 97u8, 13u8, 249u8,
                            21u8, 172u8, 49u8, 32u8, 228u8, 13u8, 229u8, 107u8, 135u8, 182u8,
                        ],
                    )
                }
                pub fn instantiate_with_code_old_weight(
                    &self,
                    value: types::instantiate_with_code_old_weight::Value,
                    gas_limit: types::instantiate_with_code_old_weight::GasLimit,
                    storage_deposit_limit : types :: instantiate_with_code_old_weight :: StorageDepositLimit,
                    code: types::instantiate_with_code_old_weight::Code,
                    data: types::instantiate_with_code_old_weight::Data,
                    salt: types::instantiate_with_code_old_weight::Salt,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::InstantiateWithCodeOldWeight,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "instantiate_with_code_old_weight",
                        types::InstantiateWithCodeOldWeight {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            48u8, 125u8, 188u8, 220u8, 158u8, 122u8, 158u8, 63u8, 0u8, 249u8,
                            164u8, 200u8, 199u8, 2u8, 21u8, 168u8, 84u8, 158u8, 120u8, 17u8, 82u8,
                            54u8, 115u8, 185u8, 69u8, 236u8, 64u8, 176u8, 187u8, 201u8, 230u8,
                            98u8,
                        ],
                    )
                }
                pub fn instantiate_old_weight(
                    &self,
                    value: types::instantiate_old_weight::Value,
                    gas_limit: types::instantiate_old_weight::GasLimit,
                    storage_deposit_limit: types::instantiate_old_weight::StorageDepositLimit,
                    code_hash: types::instantiate_old_weight::CodeHash,
                    data: types::instantiate_old_weight::Data,
                    salt: types::instantiate_old_weight::Salt,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::InstantiateOldWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "instantiate_old_weight",
                        types::InstantiateOldWeight {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        },
                        [
                            145u8, 119u8, 37u8, 211u8, 172u8, 215u8, 72u8, 110u8, 71u8, 230u8,
                            212u8, 56u8, 78u8, 221u8, 239u8, 159u8, 110u8, 219u8, 71u8, 10u8,
                            248u8, 112u8, 237u8, 188u8, 198u8, 0u8, 28u8, 255u8, 147u8, 152u8,
                            162u8, 83u8,
                        ],
                    )
                }
                pub fn upload_code(
                    &self,
                    code: types::upload_code::Code,
                    storage_deposit_limit: types::upload_code::StorageDepositLimit,
                    determinism: types::upload_code::Determinism,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UploadCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "upload_code",
                        types::UploadCode {
                            code,
                            storage_deposit_limit,
                            determinism,
                        },
                        [
                            159u8, 17u8, 234u8, 83u8, 162u8, 68u8, 117u8, 80u8, 64u8, 251u8, 31u8,
                            38u8, 214u8, 227u8, 235u8, 74u8, 97u8, 72u8, 83u8, 197u8, 7u8, 57u8,
                            212u8, 217u8, 219u8, 139u8, 182u8, 248u8, 92u8, 91u8, 56u8, 2u8,
                        ],
                    )
                }
                pub fn remove_code(
                    &self,
                    code_hash: types::remove_code::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "remove_code",
                        types::RemoveCode { code_hash },
                        [
                            99u8, 184u8, 12u8, 208u8, 123u8, 158u8, 140u8, 21u8, 190u8, 152u8,
                            95u8, 79u8, 217u8, 131u8, 161u8, 160u8, 21u8, 56u8, 167u8, 27u8, 90u8,
                            255u8, 75u8, 0u8, 133u8, 111u8, 119u8, 217u8, 157u8, 67u8, 238u8, 69u8,
                        ],
                    )
                }
                pub fn set_code(
                    &self,
                    dest: types::set_code::Dest,
                    code_hash: types::set_code::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "set_code",
                        types::SetCode { dest, code_hash },
                        [
                            66u8, 53u8, 180u8, 182u8, 167u8, 134u8, 212u8, 45u8, 162u8, 121u8,
                            89u8, 105u8, 7u8, 166u8, 112u8, 13u8, 250u8, 115u8, 128u8, 235u8,
                            124u8, 55u8, 166u8, 5u8, 158u8, 163u8, 159u8, 113u8, 243u8, 103u8,
                            214u8, 108u8,
                        ],
                    )
                }
                pub fn call(
                    &self,
                    dest: types::call::Dest,
                    value: types::call::Value,
                    gas_limit: types::call::GasLimit,
                    storage_deposit_limit: types::call::StorageDepositLimit,
                    data: types::call::Data,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Call>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "call",
                        types::Call {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        },
                        [
                            38u8, 98u8, 23u8, 7u8, 215u8, 169u8, 8u8, 156u8, 72u8, 172u8, 166u8,
                            189u8, 34u8, 9u8, 193u8, 204u8, 20u8, 152u8, 48u8, 40u8, 106u8, 109u8,
                            23u8, 64u8, 48u8, 131u8, 99u8, 37u8, 49u8, 26u8, 210u8, 196u8,
                        ],
                    )
                }
                pub fn instantiate_with_code(
                    &self,
                    value: types::instantiate_with_code::Value,
                    gas_limit: types::instantiate_with_code::GasLimit,
                    storage_deposit_limit: types::instantiate_with_code::StorageDepositLimit,
                    code: types::instantiate_with_code::Code,
                    data: types::instantiate_with_code::Data,
                    salt: types::instantiate_with_code::Salt,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::InstantiateWithCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "instantiate_with_code",
                        types::InstantiateWithCode {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            34u8, 182u8, 171u8, 163u8, 86u8, 205u8, 184u8, 72u8, 117u8, 214u8,
                            11u8, 24u8, 73u8, 6u8, 158u8, 16u8, 5u8, 212u8, 209u8, 64u8, 66u8,
                            98u8, 47u8, 14u8, 96u8, 132u8, 22u8, 37u8, 202u8, 148u8, 83u8, 125u8,
                        ],
                    )
                }
                pub fn instantiate(
                    &self,
                    value: types::instantiate::Value,
                    gas_limit: types::instantiate::GasLimit,
                    storage_deposit_limit: types::instantiate::StorageDepositLimit,
                    code_hash: types::instantiate::CodeHash,
                    data: types::instantiate::Data,
                    salt: types::instantiate::Salt,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Instantiate>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "instantiate",
                        types::Instantiate {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        },
                        [
                            221u8, 142u8, 55u8, 187u8, 6u8, 98u8, 228u8, 231u8, 38u8, 81u8, 222u8,
                            86u8, 205u8, 122u8, 32u8, 236u8, 237u8, 50u8, 201u8, 140u8, 111u8,
                            23u8, 242u8, 212u8, 118u8, 212u8, 98u8, 247u8, 166u8, 196u8, 206u8,
                            232u8,
                        ],
                    )
                }
                pub fn migrate(
                    &self,
                    weight_limit: types::migrate::WeightLimit,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Migrate>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Contracts",
                        "migrate",
                        types::Migrate { weight_limit },
                        [
                            11u8, 183u8, 183u8, 30u8, 18u8, 17u8, 58u8, 145u8, 254u8, 126u8, 21u8,
                            155u8, 27u8, 218u8, 95u8, 35u8, 38u8, 102u8, 234u8, 241u8, 67u8, 99u8,
                            183u8, 164u8, 5u8, 66u8, 186u8, 77u8, 234u8, 76u8, 206u8, 248u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_contracts::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Instantiated {
                pub deployer: instantiated::Deployer,
                pub contract: instantiated::Contract,
            }
            pub mod instantiated {
                use super::runtime_types;
                pub type Deployer = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Contract = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Instantiated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Instantiated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Terminated {
                pub contract: terminated::Contract,
                pub beneficiary: terminated::Beneficiary,
            }
            pub mod terminated {
                use super::runtime_types;
                pub type Contract = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Beneficiary = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Terminated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Terminated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeStored {
                pub code_hash: code_stored::CodeHash,
                pub deposit_held: code_stored::DepositHeld,
                pub uploader: code_stored::Uploader,
            }
            pub mod code_stored {
                use super::runtime_types;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type DepositHeld = ::core::primitive::u128;
                pub type Uploader = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for CodeStored {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeStored";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ContractEmitted {
                pub contract: contract_emitted::Contract,
                pub data: contract_emitted::Data,
            }
            pub mod contract_emitted {
                use super::runtime_types;
                pub type Contract = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Data = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ContractEmitted {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractEmitted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeRemoved {
                pub code_hash: code_removed::CodeHash,
                pub deposit_released: code_removed::DepositReleased,
                pub remover: code_removed::Remover,
            }
            pub mod code_removed {
                use super::runtime_types;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type DepositReleased = ::core::primitive::u128;
                pub type Remover = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for CodeRemoved {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ContractCodeUpdated {
                pub contract: contract_code_updated::Contract,
                pub new_code_hash: contract_code_updated::NewCodeHash,
                pub old_code_hash: contract_code_updated::OldCodeHash,
            }
            pub mod contract_code_updated {
                use super::runtime_types;
                pub type Contract = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type NewCodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type OldCodeHash = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ContractCodeUpdated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractCodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Called {
                pub caller: called::Caller,
                pub contract: called::Contract,
            }
            pub mod called {
                use super::runtime_types;
                pub type Caller = runtime_types::pallet_contracts::Origin<
                    runtime_types::contracts_node_runtime::Runtime,
                >;
                pub type Contract = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Called {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Called";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct DelegateCalled {
                pub contract: delegate_called::Contract,
                pub code_hash: delegate_called::CodeHash,
            }
            pub mod delegate_called {
                use super::runtime_types;
                pub type Contract = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DelegateCalled {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "DelegateCalled";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct StorageDepositTransferredAndHeld {
                pub from: storage_deposit_transferred_and_held::From,
                pub to: storage_deposit_transferred_and_held::To,
                pub amount: storage_deposit_transferred_and_held::Amount,
            }
            pub mod storage_deposit_transferred_and_held {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for StorageDepositTransferredAndHeld {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "StorageDepositTransferredAndHeld";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct StorageDepositTransferredAndReleased {
                pub from: storage_deposit_transferred_and_released::From,
                pub to: storage_deposit_transferred_and_released::To,
                pub amount: storage_deposit_transferred_and_released::Amount,
            }
            pub mod storage_deposit_transferred_and_released {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for StorageDepositTransferredAndReleased {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "StorageDepositTransferredAndReleased";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod pristine_code {
                    use super::runtime_types;
                    pub type PristineCode =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod code_info_of {
                    use super::runtime_types;
                    pub type CodeInfoOf = runtime_types::pallet_contracts::wasm::CodeInfo;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod nonce {
                    use super::runtime_types;
                    pub type Nonce = ::core::primitive::u64;
                }
                pub mod contract_info_of {
                    use super::runtime_types;
                    pub type ContractInfoOf =
                        runtime_types::pallet_contracts::storage::ContractInfo;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod deletion_queue {
                    use super::runtime_types;
                    pub type DeletionQueue =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod deletion_queue_counter {
                    use super::runtime_types;
                    pub type DeletionQueueCounter =
                        runtime_types::pallet_contracts::storage::DeletionQueueManager;
                }
                pub mod migration_in_progress {
                    use super::runtime_types;
                    pub type MigrationInProgress =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn pristine_code_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pristine_code::PristineCode,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "PristineCode",
                        (),
                        [
                            6u8, 31u8, 218u8, 40u8, 203u8, 188u8, 155u8, 242u8, 11u8, 64u8, 196u8,
                            23u8, 70u8, 117u8, 21u8, 42u8, 68u8, 254u8, 90u8, 190u8, 155u8, 117u8,
                            153u8, 198u8, 119u8, 35u8, 52u8, 217u8, 209u8, 144u8, 1u8, 66u8,
                        ],
                    )
                }
                pub fn pristine_code(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::pristine_code::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::pristine_code::Param0,
                    >,
                    types::pristine_code::PristineCode,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "PristineCode",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            6u8, 31u8, 218u8, 40u8, 203u8, 188u8, 155u8, 242u8, 11u8, 64u8, 196u8,
                            23u8, 70u8, 117u8, 21u8, 42u8, 68u8, 254u8, 90u8, 190u8, 155u8, 117u8,
                            153u8, 198u8, 119u8, 35u8, 52u8, 217u8, 209u8, 144u8, 1u8, 66u8,
                        ],
                    )
                }
                pub fn code_info_of_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::code_info_of::CodeInfoOf,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "CodeInfoOf",
                        (),
                        [
                            16u8, 119u8, 167u8, 116u8, 213u8, 33u8, 175u8, 218u8, 170u8, 250u8,
                            110u8, 248u8, 215u8, 25u8, 10u8, 143u8, 21u8, 37u8, 88u8, 239u8, 35u8,
                            53u8, 133u8, 126u8, 97u8, 32u8, 60u8, 8u8, 180u8, 123u8, 229u8, 163u8,
                        ],
                    )
                }
                pub fn code_info_of(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::code_info_of::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::code_info_of::Param0,
                    >,
                    types::code_info_of::CodeInfoOf,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "CodeInfoOf",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            16u8, 119u8, 167u8, 116u8, 213u8, 33u8, 175u8, 218u8, 170u8, 250u8,
                            110u8, 248u8, 215u8, 25u8, 10u8, 143u8, 21u8, 37u8, 88u8, 239u8, 35u8,
                            53u8, 133u8, 126u8, 97u8, 32u8, 60u8, 8u8, 180u8, 123u8, 229u8, 163u8,
                        ],
                    )
                }
                pub fn nonce(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::nonce::Nonce,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "Nonce",
                        (),
                        [
                            47u8, 101u8, 89u8, 252u8, 98u8, 25u8, 178u8, 154u8, 17u8, 57u8, 185u8,
                            10u8, 133u8, 94u8, 73u8, 160u8, 137u8, 150u8, 97u8, 119u8, 8u8, 146u8,
                            149u8, 146u8, 212u8, 60u8, 141u8, 24u8, 124u8, 28u8, 57u8, 19u8,
                        ],
                    )
                }
                pub fn contract_info_of_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::contract_info_of::ContractInfoOf,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "ContractInfoOf",
                        (),
                        [
                            107u8, 167u8, 76u8, 126u8, 66u8, 83u8, 0u8, 226u8, 45u8, 212u8, 83u8,
                            148u8, 196u8, 99u8, 26u8, 78u8, 231u8, 152u8, 82u8, 47u8, 174u8, 142u8,
                            85u8, 32u8, 246u8, 22u8, 96u8, 60u8, 186u8, 250u8, 176u8, 49u8,
                        ],
                    )
                }
                pub fn contract_info_of(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::contract_info_of::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::contract_info_of::Param0,
                    >,
                    types::contract_info_of::ContractInfoOf,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "ContractInfoOf",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            107u8, 167u8, 76u8, 126u8, 66u8, 83u8, 0u8, 226u8, 45u8, 212u8, 83u8,
                            148u8, 196u8, 99u8, 26u8, 78u8, 231u8, 152u8, 82u8, 47u8, 174u8, 142u8,
                            85u8, 32u8, 246u8, 22u8, 96u8, 60u8, 186u8, 250u8, 176u8, 49u8,
                        ],
                    )
                }
                pub fn deletion_queue_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::deletion_queue::DeletionQueue,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "DeletionQueue",
                        (),
                        [
                            233u8, 193u8, 191u8, 44u8, 151u8, 46u8, 124u8, 188u8, 132u8, 227u8,
                            107u8, 210u8, 37u8, 110u8, 172u8, 95u8, 12u8, 114u8, 63u8, 83u8, 60u8,
                            163u8, 58u8, 174u8, 160u8, 47u8, 198u8, 156u8, 216u8, 182u8, 65u8,
                            229u8,
                        ],
                    )
                }
                pub fn deletion_queue(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::deletion_queue::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::deletion_queue::Param0,
                    >,
                    types::deletion_queue::DeletionQueue,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "DeletionQueue",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            233u8, 193u8, 191u8, 44u8, 151u8, 46u8, 124u8, 188u8, 132u8, 227u8,
                            107u8, 210u8, 37u8, 110u8, 172u8, 95u8, 12u8, 114u8, 63u8, 83u8, 60u8,
                            163u8, 58u8, 174u8, 160u8, 47u8, 198u8, 156u8, 216u8, 182u8, 65u8,
                            229u8,
                        ],
                    )
                }
                pub fn deletion_queue_counter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::deletion_queue_counter::DeletionQueueCounter,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "DeletionQueueCounter",
                        (),
                        [
                            124u8, 63u8, 32u8, 109u8, 8u8, 113u8, 105u8, 172u8, 87u8, 88u8, 244u8,
                            191u8, 252u8, 196u8, 10u8, 137u8, 101u8, 87u8, 124u8, 220u8, 178u8,
                            155u8, 163u8, 214u8, 116u8, 121u8, 129u8, 129u8, 173u8, 76u8, 188u8,
                            41u8,
                        ],
                    )
                }
                pub fn migration_in_progress(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::migration_in_progress::MigrationInProgress,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Contracts",
                        "MigrationInProgress",
                        (),
                        [
                            238u8, 96u8, 248u8, 141u8, 247u8, 233u8, 27u8, 21u8, 187u8, 56u8,
                            195u8, 67u8, 21u8, 215u8, 30u8, 236u8, 151u8, 163u8, 115u8, 117u8,
                            154u8, 54u8, 37u8, 240u8, 136u8, 240u8, 35u8, 192u8, 168u8, 250u8,
                            132u8, 63u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn schedule(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::pallet_contracts::schedule::Schedule,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "Schedule",
                        [
                            27u8, 65u8, 3u8, 180u8, 55u8, 42u8, 107u8, 120u8, 75u8, 240u8, 182u8,
                            69u8, 255u8, 140u8, 113u8, 64u8, 104u8, 33u8, 92u8, 155u8, 238u8,
                            101u8, 118u8, 168u8, 161u8, 139u8, 138u8, 54u8, 62u8, 208u8, 68u8,
                            220u8,
                        ],
                    )
                }
                pub fn deposit_per_byte(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "DepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_deposit_limit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "DefaultDepositLimit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn deposit_per_item(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "DepositPerItem",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn code_hash_lockup_deposit_percent(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "CodeHashLockupDepositPercent",
                        [
                            65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
                            114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
                            200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
                        ],
                    )
                }
                pub fn max_code_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "MaxCodeLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_storage_key_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "MaxStorageKeyLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_delegate_dependencies(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "MaxDelegateDependencies",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn unsafe_unstable_interface(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::bool,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "UnsafeUnstableInterface",
                        [
                            165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
                            252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
                            100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
                        ],
                    )
                }
                pub fn max_debug_buffer_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "MaxDebugBufferLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn environment(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::pallet_contracts::Environment,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "Environment",
                        [
                            182u8, 82u8, 245u8, 21u8, 180u8, 187u8, 81u8, 79u8, 86u8, 189u8, 26u8,
                            84u8, 118u8, 253u8, 81u8, 12u8, 20u8, 85u8, 197u8, 131u8, 60u8, 37u8,
                            85u8, 26u8, 249u8, 76u8, 8u8, 40u8, 67u8, 30u8, 105u8, 1u8,
                        ],
                    )
                }
                pub fn api_version(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::pallet_contracts::ApiVersion,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Contracts",
                        "ApiVersion",
                        [
                            68u8, 240u8, 8u8, 124u8, 194u8, 179u8, 162u8, 203u8, 5u8, 230u8, 72u8,
                            173u8, 165u8, 230u8, 234u8, 96u8, 225u8, 71u8, 224u8, 92u8, 120u8,
                            110u8, 207u8, 208u8, 119u8, 254u8, 196u8, 192u8, 152u8, 253u8, 164u8,
                            117u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod assets {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_assets::pallet::Error;
        pub type Call = runtime_types::pallet_assets::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Create {
                    #[codec(compact)]
                    pub id: create::Id,
                    pub admin: create::Admin,
                    pub min_balance: create::MinBalance,
                }
                pub mod create {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Admin = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Create {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "create";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceCreate {
                    #[codec(compact)]
                    pub id: force_create::Id,
                    pub owner: force_create::Owner,
                    pub is_sufficient: force_create::IsSufficient,
                    #[codec(compact)]
                    pub min_balance: force_create::MinBalance,
                }
                pub mod force_create {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type IsSufficient = ::core::primitive::bool;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceCreate {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_create";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct StartDestroy {
                    #[codec(compact)]
                    pub id: start_destroy::Id,
                }
                pub mod start_destroy {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for StartDestroy {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "start_destroy";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DestroyAccounts {
                    #[codec(compact)]
                    pub id: destroy_accounts::Id,
                }
                pub mod destroy_accounts {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DestroyAccounts {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "destroy_accounts";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DestroyApprovals {
                    #[codec(compact)]
                    pub id: destroy_approvals::Id,
                }
                pub mod destroy_approvals {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DestroyApprovals {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "destroy_approvals";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FinishDestroy {
                    #[codec(compact)]
                    pub id: finish_destroy::Id,
                }
                pub mod finish_destroy {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for FinishDestroy {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "finish_destroy";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Mint {
                    #[codec(compact)]
                    pub id: mint::Id,
                    pub beneficiary: mint::Beneficiary,
                    #[codec(compact)]
                    pub amount: mint::Amount,
                }
                pub mod mint {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Beneficiary = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Mint {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "mint";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Burn {
                    #[codec(compact)]
                    pub id: burn::Id,
                    pub who: burn::Who,
                    #[codec(compact)]
                    pub amount: burn::Amount,
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Burn {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "burn";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Transfer {
                    #[codec(compact)]
                    pub id: transfer::Id,
                    pub target: transfer::Target,
                    #[codec(compact)]
                    pub amount: transfer::Amount,
                }
                pub mod transfer {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Target = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransferKeepAlive {
                    #[codec(compact)]
                    pub id: transfer_keep_alive::Id,
                    pub target: transfer_keep_alive::Target,
                    #[codec(compact)]
                    pub amount: transfer_keep_alive::Amount,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Target = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceTransfer {
                    #[codec(compact)]
                    pub id: force_transfer::Id,
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub amount: force_transfer::Amount,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Source = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Freeze {
                    #[codec(compact)]
                    pub id: freeze::Id,
                    pub who: freeze::Who,
                }
                pub mod freeze {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Freeze {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "freeze";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Thaw {
                    #[codec(compact)]
                    pub id: thaw::Id,
                    pub who: thaw::Who,
                }
                pub mod thaw {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Thaw {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "thaw";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FreezeAsset {
                    #[codec(compact)]
                    pub id: freeze_asset::Id,
                }
                pub mod freeze_asset {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for FreezeAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "freeze_asset";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ThawAsset {
                    #[codec(compact)]
                    pub id: thaw_asset::Id,
                }
                pub mod thaw_asset {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ThawAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "thaw_asset";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransferOwnership {
                    #[codec(compact)]
                    pub id: transfer_ownership::Id,
                    pub owner: transfer_ownership::Owner,
                }
                pub mod transfer_ownership {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferOwnership {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer_ownership";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetTeam {
                    #[codec(compact)]
                    pub id: set_team::Id,
                    pub issuer: set_team::Issuer,
                    pub admin: set_team::Admin,
                    pub freezer: set_team::Freezer,
                }
                pub mod set_team {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Issuer = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Admin = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Freezer = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetTeam {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "set_team";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetMetadata {
                    #[codec(compact)]
                    pub id: set_metadata::Id,
                    pub name: set_metadata::Name,
                    pub symbol: set_metadata::Symbol,
                    pub decimals: set_metadata::Decimals,
                }
                pub mod set_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Name =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Symbol =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Decimals = ::core::primitive::u8;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "set_metadata";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ClearMetadata {
                    #[codec(compact)]
                    pub id: clear_metadata::Id,
                }
                pub mod clear_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ClearMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "clear_metadata";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceSetMetadata {
                    #[codec(compact)]
                    pub id: force_set_metadata::Id,
                    pub name: force_set_metadata::Name,
                    pub symbol: force_set_metadata::Symbol,
                    pub decimals: force_set_metadata::Decimals,
                    pub is_frozen: force_set_metadata::IsFrozen,
                }
                pub mod force_set_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Name =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Symbol =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Decimals = ::core::primitive::u8;
                    pub type IsFrozen = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSetMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_set_metadata";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceClearMetadata {
                    #[codec(compact)]
                    pub id: force_clear_metadata::Id,
                }
                pub mod force_clear_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceClearMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_clear_metadata";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceAssetStatus {
                    #[codec(compact)]
                    pub id: force_asset_status::Id,
                    pub owner: force_asset_status::Owner,
                    pub issuer: force_asset_status::Issuer,
                    pub admin: force_asset_status::Admin,
                    pub freezer: force_asset_status::Freezer,
                    #[codec(compact)]
                    pub min_balance: force_asset_status::MinBalance,
                    pub is_sufficient: force_asset_status::IsSufficient,
                    pub is_frozen: force_asset_status::IsFrozen,
                }
                pub mod force_asset_status {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Issuer = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Admin = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Freezer = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type MinBalance = ::core::primitive::u128;
                    pub type IsSufficient = ::core::primitive::bool;
                    pub type IsFrozen = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceAssetStatus {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_asset_status";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ApproveTransfer {
                    #[codec(compact)]
                    pub id: approve_transfer::Id,
                    pub delegate: approve_transfer::Delegate,
                    #[codec(compact)]
                    pub amount: approve_transfer::Amount,
                }
                pub mod approve_transfer {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Delegate = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApproveTransfer {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "approve_transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CancelApproval {
                    #[codec(compact)]
                    pub id: cancel_approval::Id,
                    pub delegate: cancel_approval::Delegate,
                }
                pub mod cancel_approval {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Delegate = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for CancelApproval {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "cancel_approval";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ForceCancelApproval {
                    #[codec(compact)]
                    pub id: force_cancel_approval::Id,
                    pub owner: force_cancel_approval::Owner,
                    pub delegate: force_cancel_approval::Delegate,
                }
                pub mod force_cancel_approval {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Delegate = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceCancelApproval {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_cancel_approval";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransferApproved {
                    #[codec(compact)]
                    pub id: transfer_approved::Id,
                    pub owner: transfer_approved::Owner,
                    pub destination: transfer_approved::Destination,
                    #[codec(compact)]
                    pub amount: transfer_approved::Amount,
                }
                pub mod transfer_approved {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Destination = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferApproved {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer_approved";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Touch {
                    #[codec(compact)]
                    pub id: touch::Id,
                }
                pub mod touch {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Touch {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "touch";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Refund {
                    #[codec(compact)]
                    pub id: refund::Id,
                    pub allow_burn: refund::AllowBurn,
                }
                pub mod refund {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type AllowBurn = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Refund {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "refund";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetMinBalance {
                    #[codec(compact)]
                    pub id: set_min_balance::Id,
                    pub min_balance: set_min_balance::MinBalance,
                }
                pub mod set_min_balance {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetMinBalance {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "set_min_balance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TouchOther {
                    #[codec(compact)]
                    pub id: touch_other::Id,
                    pub who: touch_other::Who,
                }
                pub mod touch_other {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TouchOther {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "touch_other";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RefundOther {
                    #[codec(compact)]
                    pub id: refund_other::Id,
                    pub who: refund_other::Who,
                }
                pub mod refund_other {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RefundOther {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "refund_other";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Block {
                    #[codec(compact)]
                    pub id: block::Id,
                    pub who: block::Who,
                }
                pub mod block {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Block {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "block";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create(
                    &self,
                    id: types::create::Id,
                    admin: types::create::Admin,
                    min_balance: types::create::MinBalance,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Create>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "create",
                        types::Create {
                            id,
                            admin,
                            min_balance,
                        },
                        [
                            120u8, 25u8, 99u8, 39u8, 102u8, 201u8, 14u8, 2u8, 32u8, 139u8, 206u8,
                            218u8, 223u8, 161u8, 25u8, 98u8, 159u8, 133u8, 65u8, 105u8, 45u8, 4u8,
                            28u8, 49u8, 248u8, 147u8, 2u8, 179u8, 11u8, 195u8, 177u8, 250u8,
                        ],
                    )
                }
                pub fn force_create(
                    &self,
                    id: types::force_create::Id,
                    owner: types::force_create::Owner,
                    is_sufficient: types::force_create::IsSufficient,
                    min_balance: types::force_create::MinBalance,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceCreate>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_create",
                        types::ForceCreate {
                            id,
                            owner,
                            is_sufficient,
                            min_balance,
                        },
                        [
                            149u8, 41u8, 54u8, 146u8, 18u8, 248u8, 84u8, 52u8, 202u8, 88u8, 192u8,
                            208u8, 247u8, 227u8, 254u8, 98u8, 92u8, 46u8, 164u8, 152u8, 143u8,
                            20u8, 179u8, 227u8, 197u8, 247u8, 242u8, 153u8, 142u8, 148u8, 40u8,
                            184u8,
                        ],
                    )
                }
                pub fn start_destroy(
                    &self,
                    id: types::start_destroy::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::StartDestroy>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "start_destroy",
                        types::StartDestroy { id },
                        [
                            125u8, 82u8, 151u8, 106u8, 25u8, 49u8, 68u8, 203u8, 247u8, 175u8,
                            117u8, 230u8, 84u8, 98u8, 172u8, 73u8, 233u8, 218u8, 212u8, 198u8,
                            69u8, 35u8, 15u8, 179u8, 161u8, 205u8, 190u8, 109u8, 198u8, 214u8,
                            65u8, 164u8,
                        ],
                    )
                }
                pub fn destroy_accounts(
                    &self,
                    id: types::destroy_accounts::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::DestroyAccounts>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "destroy_accounts",
                        types::DestroyAccounts { id },
                        [
                            236u8, 102u8, 233u8, 170u8, 179u8, 46u8, 42u8, 29u8, 200u8, 116u8,
                            62u8, 114u8, 233u8, 59u8, 217u8, 215u8, 109u8, 232u8, 147u8, 95u8,
                            255u8, 248u8, 119u8, 222u8, 216u8, 165u8, 138u8, 47u8, 28u8, 56u8,
                            204u8, 93u8,
                        ],
                    )
                }
                pub fn destroy_approvals(
                    &self,
                    id: types::destroy_approvals::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::DestroyApprovals>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "destroy_approvals",
                        types::DestroyApprovals { id },
                        [
                            34u8, 35u8, 15u8, 44u8, 239u8, 232u8, 88u8, 130u8, 130u8, 87u8, 171u8,
                            255u8, 247u8, 179u8, 14u8, 35u8, 47u8, 223u8, 32u8, 232u8, 41u8, 105u8,
                            207u8, 199u8, 90u8, 136u8, 144u8, 139u8, 252u8, 76u8, 177u8, 106u8,
                        ],
                    )
                }
                pub fn finish_destroy(
                    &self,
                    id: types::finish_destroy::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::FinishDestroy>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "finish_destroy",
                        types::FinishDestroy { id },
                        [
                            132u8, 67u8, 78u8, 84u8, 240u8, 51u8, 176u8, 119u8, 48u8, 34u8, 153u8,
                            37u8, 25u8, 171u8, 21u8, 164u8, 53u8, 214u8, 36u8, 149u8, 20u8, 240u8,
                            123u8, 195u8, 170u8, 162u8, 118u8, 81u8, 176u8, 218u8, 114u8, 113u8,
                        ],
                    )
                }
                pub fn mint(
                    &self,
                    id: types::mint::Id,
                    beneficiary: types::mint::Beneficiary,
                    amount: types::mint::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Mint>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "mint",
                        types::Mint {
                            id,
                            beneficiary,
                            amount,
                        },
                        [
                            172u8, 131u8, 103u8, 81u8, 206u8, 2u8, 143u8, 114u8, 137u8, 60u8,
                            147u8, 67u8, 226u8, 64u8, 71u8, 11u8, 36u8, 145u8, 51u8, 8u8, 0u8,
                            110u8, 8u8, 195u8, 103u8, 205u8, 156u8, 43u8, 215u8, 12u8, 150u8,
                            135u8,
                        ],
                    )
                }
                pub fn burn(
                    &self,
                    id: types::burn::Id,
                    who: types::burn::Who,
                    amount: types::burn::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Burn>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "burn",
                        types::Burn { id, who, amount },
                        [
                            105u8, 133u8, 82u8, 100u8, 124u8, 65u8, 174u8, 31u8, 152u8, 45u8, 23u8,
                            200u8, 23u8, 199u8, 239u8, 8u8, 187u8, 142u8, 21u8, 192u8, 35u8, 211u8,
                            172u8, 130u8, 169u8, 74u8, 167u8, 36u8, 149u8, 7u8, 19u8, 37u8,
                        ],
                    )
                }
                pub fn transfer(
                    &self,
                    id: types::transfer::Id,
                    target: types::transfer::Target,
                    amount: types::transfer::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Transfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer",
                        types::Transfer { id, target, amount },
                        [
                            126u8, 31u8, 70u8, 179u8, 222u8, 190u8, 12u8, 19u8, 94u8, 225u8, 217u8,
                            109u8, 54u8, 69u8, 124u8, 61u8, 97u8, 199u8, 193u8, 166u8, 39u8, 143u8,
                            125u8, 251u8, 87u8, 173u8, 149u8, 91u8, 182u8, 18u8, 184u8, 65u8,
                        ],
                    )
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: types::transfer_keep_alive::Id,
                    target: types::transfer_keep_alive::Target,
                    amount: types::transfer_keep_alive::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { id, target, amount },
                        [
                            99u8, 101u8, 219u8, 188u8, 238u8, 230u8, 141u8, 43u8, 38u8, 175u8,
                            46u8, 89u8, 33u8, 23u8, 223u8, 115u8, 108u8, 18u8, 190u8, 213u8, 157u8,
                            12u8, 139u8, 97u8, 7u8, 75u8, 196u8, 159u8, 122u8, 32u8, 164u8, 154u8,
                        ],
                    )
                }
                pub fn force_transfer(
                    &self,
                    id: types::force_transfer::Id,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    amount: types::force_transfer::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_transfer",
                        types::ForceTransfer {
                            id,
                            source,
                            dest,
                            amount,
                        },
                        [
                            10u8, 210u8, 8u8, 209u8, 8u8, 78u8, 40u8, 213u8, 235u8, 176u8, 144u8,
                            145u8, 70u8, 13u8, 75u8, 72u8, 166u8, 137u8, 22u8, 191u8, 226u8, 244u8,
                            92u8, 183u8, 129u8, 212u8, 158u8, 179u8, 169u8, 232u8, 177u8, 225u8,
                        ],
                    )
                }
                pub fn freeze(
                    &self,
                    id: types::freeze::Id,
                    who: types::freeze::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Freeze>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "freeze",
                        types::Freeze { id, who },
                        [
                            180u8, 124u8, 252u8, 66u8, 205u8, 23u8, 32u8, 217u8, 173u8, 10u8, 91u8,
                            57u8, 44u8, 215u8, 234u8, 152u8, 115u8, 38u8, 141u8, 212u8, 57u8,
                            217u8, 169u8, 61u8, 215u8, 130u8, 172u8, 58u8, 90u8, 193u8, 25u8,
                            249u8,
                        ],
                    )
                }
                pub fn thaw(
                    &self,
                    id: types::thaw::Id,
                    who: types::thaw::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Thaw>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "thaw",
                        types::Thaw { id, who },
                        [
                            187u8, 130u8, 9u8, 152u8, 231u8, 9u8, 245u8, 162u8, 115u8, 19u8, 73u8,
                            176u8, 16u8, 230u8, 30u8, 60u8, 180u8, 183u8, 154u8, 160u8, 72u8,
                            219u8, 116u8, 57u8, 140u8, 6u8, 105u8, 38u8, 98u8, 90u8, 250u8, 135u8,
                        ],
                    )
                }
                pub fn freeze_asset(
                    &self,
                    id: types::freeze_asset::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::FreezeAsset>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "freeze_asset",
                        types::FreezeAsset { id },
                        [
                            75u8, 237u8, 183u8, 112u8, 112u8, 123u8, 250u8, 203u8, 169u8, 51u8,
                            218u8, 35u8, 159u8, 23u8, 21u8, 10u8, 167u8, 84u8, 161u8, 212u8, 124u8,
                            236u8, 88u8, 175u8, 48u8, 195u8, 33u8, 145u8, 141u8, 156u8, 31u8,
                            250u8,
                        ],
                    )
                }
                pub fn thaw_asset(
                    &self,
                    id: types::thaw_asset::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ThawAsset>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "thaw_asset",
                        types::ThawAsset { id },
                        [
                            151u8, 6u8, 170u8, 114u8, 55u8, 8u8, 5u8, 194u8, 251u8, 78u8, 232u8,
                            181u8, 157u8, 62u8, 16u8, 39u8, 79u8, 119u8, 205u8, 198u8, 199u8, 26u8,
                            92u8, 162u8, 169u8, 173u8, 93u8, 51u8, 7u8, 79u8, 198u8, 77u8,
                        ],
                    )
                }
                pub fn transfer_ownership(
                    &self,
                    id: types::transfer_ownership::Id,
                    owner: types::transfer_ownership::Owner,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferOwnership>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer_ownership",
                        types::TransferOwnership { id, owner },
                        [
                            65u8, 85u8, 40u8, 202u8, 212u8, 170u8, 130u8, 132u8, 140u8, 90u8, 68u8,
                            28u8, 101u8, 154u8, 222u8, 150u8, 244u8, 165u8, 44u8, 22u8, 225u8,
                            152u8, 7u8, 162u8, 110u8, 54u8, 173u8, 181u8, 54u8, 215u8, 105u8,
                            239u8,
                        ],
                    )
                }
                pub fn set_team(
                    &self,
                    id: types::set_team::Id,
                    issuer: types::set_team::Issuer,
                    admin: types::set_team::Admin,
                    freezer: types::set_team::Freezer,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetTeam>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "set_team",
                        types::SetTeam {
                            id,
                            issuer,
                            admin,
                            freezer,
                        },
                        [
                            52u8, 75u8, 50u8, 30u8, 164u8, 161u8, 121u8, 25u8, 135u8, 83u8, 115u8,
                            25u8, 103u8, 1u8, 124u8, 206u8, 83u8, 182u8, 41u8, 116u8, 44u8, 37u8,
                            75u8, 70u8, 252u8, 225u8, 240u8, 144u8, 96u8, 160u8, 151u8, 4u8,
                        ],
                    )
                }
                pub fn set_metadata(
                    &self,
                    id: types::set_metadata::Id,
                    name: types::set_metadata::Name,
                    symbol: types::set_metadata::Symbol,
                    decimals: types::set_metadata::Decimals,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetMetadata>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "set_metadata",
                        types::SetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                        },
                        [
                            215u8, 66u8, 15u8, 17u8, 88u8, 174u8, 77u8, 75u8, 229u8, 155u8, 160u8,
                            34u8, 108u8, 194u8, 88u8, 238u8, 131u8, 97u8, 234u8, 102u8, 71u8, 56u8,
                            70u8, 248u8, 211u8, 85u8, 72u8, 92u8, 71u8, 222u8, 190u8, 91u8,
                        ],
                    )
                }
                pub fn clear_metadata(
                    &self,
                    id: types::clear_metadata::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ClearMetadata>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "clear_metadata",
                        types::ClearMetadata { id },
                        [
                            68u8, 172u8, 6u8, 158u8, 237u8, 254u8, 22u8, 4u8, 254u8, 157u8, 179u8,
                            168u8, 105u8, 114u8, 56u8, 166u8, 213u8, 38u8, 188u8, 195u8, 99u8,
                            43u8, 142u8, 220u8, 94u8, 248u8, 51u8, 226u8, 233u8, 114u8, 86u8, 93u8,
                        ],
                    )
                }
                pub fn force_set_metadata(
                    &self,
                    id: types::force_set_metadata::Id,
                    name: types::force_set_metadata::Name,
                    symbol: types::force_set_metadata::Symbol,
                    decimals: types::force_set_metadata::Decimals,
                    is_frozen: types::force_set_metadata::IsFrozen,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSetMetadata>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_set_metadata",
                        types::ForceSetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                            is_frozen,
                        },
                        [
                            76u8, 90u8, 182u8, 13u8, 133u8, 248u8, 94u8, 136u8, 169u8, 114u8,
                            151u8, 20u8, 106u8, 89u8, 78u8, 228u8, 22u8, 29u8, 68u8, 8u8, 54u8,
                            47u8, 1u8, 186u8, 45u8, 167u8, 14u8, 112u8, 34u8, 43u8, 91u8, 140u8,
                        ],
                    )
                }
                pub fn force_clear_metadata(
                    &self,
                    id: types::force_clear_metadata::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceClearMetadata>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_clear_metadata",
                        types::ForceClearMetadata { id },
                        [
                            2u8, 224u8, 84u8, 48u8, 130u8, 132u8, 79u8, 38u8, 217u8, 17u8, 165u8,
                            139u8, 89u8, 53u8, 116u8, 184u8, 32u8, 91u8, 122u8, 39u8, 85u8, 40u8,
                            213u8, 216u8, 135u8, 171u8, 50u8, 69u8, 202u8, 28u8, 166u8, 147u8,
                        ],
                    )
                }
                pub fn force_asset_status(
                    &self,
                    id: types::force_asset_status::Id,
                    owner: types::force_asset_status::Owner,
                    issuer: types::force_asset_status::Issuer,
                    admin: types::force_asset_status::Admin,
                    freezer: types::force_asset_status::Freezer,
                    min_balance: types::force_asset_status::MinBalance,
                    is_sufficient: types::force_asset_status::IsSufficient,
                    is_frozen: types::force_asset_status::IsFrozen,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceAssetStatus>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_asset_status",
                        types::ForceAssetStatus {
                            id,
                            owner,
                            issuer,
                            admin,
                            freezer,
                            min_balance,
                            is_sufficient,
                            is_frozen,
                        },
                        [
                            149u8, 136u8, 250u8, 33u8, 53u8, 220u8, 207u8, 187u8, 42u8, 118u8,
                            93u8, 173u8, 100u8, 243u8, 234u8, 207u8, 88u8, 45u8, 79u8, 221u8,
                            113u8, 166u8, 229u8, 171u8, 223u8, 126u8, 20u8, 67u8, 19u8, 77u8, 44u8,
                            19u8,
                        ],
                    )
                }
                pub fn approve_transfer(
                    &self,
                    id: types::approve_transfer::Id,
                    delegate: types::approve_transfer::Delegate,
                    amount: types::approve_transfer::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ApproveTransfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "approve_transfer",
                        types::ApproveTransfer {
                            id,
                            delegate,
                            amount,
                        },
                        [
                            39u8, 227u8, 23u8, 143u8, 10u8, 120u8, 227u8, 1u8, 223u8, 78u8, 40u8,
                            213u8, 249u8, 175u8, 170u8, 183u8, 10u8, 244u8, 117u8, 111u8, 140u8,
                            157u8, 153u8, 212u8, 94u8, 119u8, 213u8, 44u8, 41u8, 8u8, 114u8, 200u8,
                        ],
                    )
                }
                pub fn cancel_approval(
                    &self,
                    id: types::cancel_approval::Id,
                    delegate: types::cancel_approval::Delegate,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::CancelApproval>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "cancel_approval",
                        types::CancelApproval { id, delegate },
                        [
                            74u8, 117u8, 101u8, 78u8, 152u8, 208u8, 16u8, 102u8, 34u8, 195u8, 61u8,
                            36u8, 85u8, 91u8, 253u8, 182u8, 61u8, 199u8, 12u8, 102u8, 149u8, 20u8,
                            238u8, 207u8, 236u8, 50u8, 63u8, 249u8, 34u8, 85u8, 88u8, 229u8,
                        ],
                    )
                }
                pub fn force_cancel_approval(
                    &self,
                    id: types::force_cancel_approval::Id,
                    owner: types::force_cancel_approval::Owner,
                    delegate: types::force_cancel_approval::Delegate,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceCancelApproval>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_cancel_approval",
                        types::ForceCancelApproval {
                            id,
                            owner,
                            delegate,
                        },
                        [
                            27u8, 231u8, 85u8, 241u8, 18u8, 151u8, 64u8, 234u8, 11u8, 84u8, 252u8,
                            128u8, 44u8, 247u8, 132u8, 82u8, 34u8, 210u8, 202u8, 50u8, 158u8, 45u8,
                            239u8, 192u8, 7u8, 24u8, 39u8, 95u8, 57u8, 21u8, 178u8, 113u8,
                        ],
                    )
                }
                pub fn transfer_approved(
                    &self,
                    id: types::transfer_approved::Id,
                    owner: types::transfer_approved::Owner,
                    destination: types::transfer_approved::Destination,
                    amount: types::transfer_approved::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferApproved>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer_approved",
                        types::TransferApproved {
                            id,
                            owner,
                            destination,
                            amount,
                        },
                        [
                            214u8, 51u8, 243u8, 129u8, 116u8, 233u8, 199u8, 183u8, 25u8, 5u8,
                            109u8, 85u8, 255u8, 68u8, 36u8, 99u8, 99u8, 179u8, 34u8, 66u8, 65u8,
                            82u8, 189u8, 174u8, 22u8, 100u8, 211u8, 13u8, 178u8, 19u8, 128u8,
                            177u8,
                        ],
                    )
                }
                pub fn touch(
                    &self,
                    id: types::touch::Id,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Touch>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "touch",
                        types::Touch { id },
                        [
                            50u8, 185u8, 46u8, 134u8, 136u8, 31u8, 191u8, 34u8, 215u8, 150u8, 73u8,
                            103u8, 140u8, 36u8, 95u8, 156u8, 201u8, 152u8, 32u8, 165u8, 47u8, 86u8,
                            163u8, 255u8, 8u8, 251u8, 176u8, 138u8, 165u8, 48u8, 12u8, 27u8,
                        ],
                    )
                }
                pub fn refund(
                    &self,
                    id: types::refund::Id,
                    allow_burn: types::refund::AllowBurn,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Refund>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "refund",
                        types::Refund { id, allow_burn },
                        [
                            218u8, 207u8, 8u8, 41u8, 154u8, 250u8, 117u8, 174u8, 143u8, 133u8,
                            34u8, 113u8, 171u8, 18u8, 177u8, 227u8, 146u8, 92u8, 12u8, 226u8,
                            101u8, 230u8, 246u8, 162u8, 32u8, 73u8, 138u8, 158u8, 95u8, 226u8,
                            75u8, 95u8,
                        ],
                    )
                }
                pub fn set_min_balance(
                    &self,
                    id: types::set_min_balance::Id,
                    min_balance: types::set_min_balance::MinBalance,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetMinBalance>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "set_min_balance",
                        types::SetMinBalance { id, min_balance },
                        [
                            141u8, 241u8, 137u8, 50u8, 232u8, 122u8, 252u8, 104u8, 185u8, 170u8,
                            246u8, 0u8, 20u8, 128u8, 136u8, 155u8, 62u8, 243u8, 4u8, 221u8, 42u8,
                            225u8, 16u8, 245u8, 58u8, 127u8, 84u8, 193u8, 175u8, 165u8, 35u8, 49u8,
                        ],
                    )
                }
                pub fn touch_other(
                    &self,
                    id: types::touch_other::Id,
                    who: types::touch_other::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TouchOther>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "touch_other",
                        types::TouchOther { id, who },
                        [
                            104u8, 85u8, 80u8, 68u8, 135u8, 149u8, 102u8, 104u8, 188u8, 79u8, 42u8,
                            34u8, 241u8, 84u8, 183u8, 176u8, 215u8, 172u8, 78u8, 196u8, 206u8,
                            214u8, 138u8, 240u8, 92u8, 65u8, 117u8, 170u8, 140u8, 120u8, 50u8,
                            166u8,
                        ],
                    )
                }
                pub fn refund_other(
                    &self,
                    id: types::refund_other::Id,
                    who: types::refund_other::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RefundOther>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "refund_other",
                        types::RefundOther { id, who },
                        [
                            113u8, 58u8, 33u8, 109u8, 233u8, 229u8, 210u8, 40u8, 176u8, 252u8,
                            131u8, 80u8, 33u8, 132u8, 19u8, 170u8, 145u8, 146u8, 246u8, 31u8,
                            222u8, 120u8, 167u8, 187u8, 8u8, 144u8, 164u8, 251u8, 52u8, 249u8,
                            91u8, 136u8,
                        ],
                    )
                }
                pub fn block(
                    &self,
                    id: types::block::Id,
                    who: types::block::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Block>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "block",
                        types::Block { id, who },
                        [
                            224u8, 63u8, 26u8, 229u8, 23u8, 164u8, 212u8, 170u8, 156u8, 104u8,
                            63u8, 158u8, 53u8, 162u8, 157u8, 127u8, 183u8, 94u8, 211u8, 123u8,
                            228u8, 198u8, 47u8, 80u8, 53u8, 122u8, 46u8, 69u8, 67u8, 170u8, 193u8,
                            33u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Created {
                pub asset_id: created::AssetId,
                pub creator: created::Creator,
                pub owner: created::Owner,
            }
            pub mod created {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Creator = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Created {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Issued {
                pub asset_id: issued::AssetId,
                pub owner: issued::Owner,
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Transferred {
                pub asset_id: transferred::AssetId,
                pub from: transferred::From,
                pub to: transferred::To,
                pub amount: transferred::Amount,
            }
            pub mod transferred {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Transferred {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Burned {
                pub asset_id: burned::AssetId,
                pub owner: burned::Owner,
                pub balance: burned::Balance,
            }
            pub mod burned {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Balance = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct TeamChanged {
                pub asset_id: team_changed::AssetId,
                pub issuer: team_changed::Issuer,
                pub admin: team_changed::Admin,
                pub freezer: team_changed::Freezer,
            }
            pub mod team_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Issuer = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Admin = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Freezer = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TeamChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct OwnerChanged {
                pub asset_id: owner_changed::AssetId,
                pub owner: owner_changed::Owner,
            }
            pub mod owner_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for OwnerChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Frozen {
                pub asset_id: frozen::AssetId,
                pub who: frozen::Who,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Thawed {
                pub asset_id: thawed::AssetId,
                pub who: thawed::Who,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AssetFrozen {
                pub asset_id: asset_frozen::AssetId,
            }
            pub mod asset_frozen {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AssetFrozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AssetThawed {
                pub asset_id: asset_thawed::AssetId,
            }
            pub mod asset_thawed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AssetThawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AccountsDestroyed {
                pub asset_id: accounts_destroyed::AssetId,
                pub accounts_destroyed: accounts_destroyed::AccountsDestroyed,
                pub accounts_remaining: accounts_destroyed::AccountsRemaining,
            }
            pub mod accounts_destroyed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type AccountsDestroyed = ::core::primitive::u32;
                pub type AccountsRemaining = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AccountsDestroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AccountsDestroyed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ApprovalsDestroyed {
                pub asset_id: approvals_destroyed::AssetId,
                pub approvals_destroyed: approvals_destroyed::ApprovalsDestroyed,
                pub approvals_remaining: approvals_destroyed::ApprovalsRemaining,
            }
            pub mod approvals_destroyed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type ApprovalsDestroyed = ::core::primitive::u32;
                pub type ApprovalsRemaining = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApprovalsDestroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalsDestroyed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct DestructionStarted {
                pub asset_id: destruction_started::AssetId,
            }
            pub mod destruction_started {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DestructionStarted {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "DestructionStarted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Destroyed {
                pub asset_id: destroyed::AssetId,
            }
            pub mod destroyed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Destroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ForceCreated {
                pub asset_id: force_created::AssetId,
                pub owner: force_created::Owner,
            }
            pub mod force_created {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ForceCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct MetadataSet {
                pub asset_id: metadata_set::AssetId,
                pub name: metadata_set::Name,
                pub symbol: metadata_set::Symbol,
                pub decimals: metadata_set::Decimals,
                pub is_frozen: metadata_set::IsFrozen,
            }
            pub mod metadata_set {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Name = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Symbol = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Decimals = ::core::primitive::u8;
                pub type IsFrozen = ::core::primitive::bool;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for MetadataSet {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct MetadataCleared {
                pub asset_id: metadata_cleared::AssetId,
            }
            pub mod metadata_cleared {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for MetadataCleared {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ApprovedTransfer {
                pub asset_id: approved_transfer::AssetId,
                pub source: approved_transfer::Source,
                pub delegate: approved_transfer::Delegate,
                pub amount: approved_transfer::Amount,
            }
            pub mod approved_transfer {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Source = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Delegate = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApprovedTransfer {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ApprovalCancelled {
                pub asset_id: approval_cancelled::AssetId,
                pub owner: approval_cancelled::Owner,
                pub delegate: approval_cancelled::Delegate,
            }
            pub mod approval_cancelled {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Delegate = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApprovalCancelled {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct TransferredApproved {
                pub asset_id: transferred_approved::AssetId,
                pub owner: transferred_approved::Owner,
                pub delegate: transferred_approved::Delegate,
                pub destination: transferred_approved::Destination,
                pub amount: transferred_approved::Amount,
            }
            pub mod transferred_approved {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Delegate = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Destination = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TransferredApproved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AssetStatusChanged {
                pub asset_id: asset_status_changed::AssetId,
            }
            pub mod asset_status_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AssetStatusChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AssetMinBalanceChanged {
                pub asset_id: asset_min_balance_changed::AssetId,
                pub new_min_balance: asset_min_balance_changed::NewMinBalance,
            }
            pub mod asset_min_balance_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type NewMinBalance = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AssetMinBalanceChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetMinBalanceChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Touched {
                pub asset_id: touched::AssetId,
                pub who: touched::Who,
                pub depositor: touched::Depositor,
            }
            pub mod touched {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Depositor = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Touched {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Touched";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Blocked {
                pub asset_id: blocked::AssetId,
                pub who: blocked::Who,
            }
            pub mod blocked {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Blocked {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Blocked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod asset {
                    use super::runtime_types;
                    pub type Asset = runtime_types::pallet_assets::types::AssetDetails<
                        ::core::primitive::u128,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::pallet_assets::types::AssetAccount<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                        (),
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod approvals {
                    use super::runtime_types;
                    pub type Approvals = runtime_types::pallet_assets::types::Approval<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param2 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod metadata {
                    use super::runtime_types;
                    pub type Metadata = runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn asset_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::asset::Asset,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Asset",
                        (),
                        [
                            159u8, 234u8, 177u8, 31u8, 58u8, 51u8, 173u8, 184u8, 250u8, 169u8,
                            246u8, 122u8, 54u8, 19u8, 232u8, 60u8, 0u8, 165u8, 12u8, 101u8, 93u8,
                            169u8, 23u8, 34u8, 154u8, 44u8, 134u8, 128u8, 97u8, 71u8, 167u8, 224u8,
                        ],
                    )
                }
                pub fn asset(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::asset::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::asset::Param0,
                    >,
                    types::asset::Asset,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Asset",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            159u8, 234u8, 177u8, 31u8, 58u8, 51u8, 173u8, 184u8, 250u8, 169u8,
                            246u8, 122u8, 54u8, 19u8, 232u8, 60u8, 0u8, 165u8, 12u8, 101u8, 93u8,
                            169u8, 23u8, 34u8, 154u8, 44u8, 134u8, 128u8, 97u8, 71u8, 167u8, 224u8,
                        ],
                    )
                }
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Account",
                        (),
                        [
                            188u8, 242u8, 133u8, 64u8, 0u8, 11u8, 57u8, 146u8, 60u8, 137u8, 35u8,
                            23u8, 183u8, 200u8, 242u8, 8u8, 94u8, 158u8, 218u8, 13u8, 104u8, 215u8,
                            87u8, 86u8, 69u8, 200u8, 11u8, 51u8, 6u8, 65u8, 216u8, 102u8,
                        ],
                    )
                }
                pub fn account_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            188u8, 242u8, 133u8, 64u8, 0u8, 11u8, 57u8, 146u8, 60u8, 137u8, 35u8,
                            23u8, 183u8, 200u8, 242u8, 8u8, 94u8, 158u8, 218u8, 13u8, 104u8, 215u8,
                            87u8, 86u8, 69u8, 200u8, 11u8, 51u8, 6u8, 65u8, 216u8, 102u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                    _1: impl ::core::borrow::Borrow<types::account::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::account::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::account::Param1,
                        >,
                    ),
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Account",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            188u8, 242u8, 133u8, 64u8, 0u8, 11u8, 57u8, 146u8, 60u8, 137u8, 35u8,
                            23u8, 183u8, 200u8, 242u8, 8u8, 94u8, 158u8, 218u8, 13u8, 104u8, 215u8,
                            87u8, 86u8, 69u8, 200u8, 11u8, 51u8, 6u8, 65u8, 216u8, 102u8,
                        ],
                    )
                }
                pub fn approvals_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        (),
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }
                pub fn approvals_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::approvals::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::approvals::Param0,
                    >,
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }
                pub fn approvals_iter2(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::approvals::Param0>,
                    _1: impl ::core::borrow::Borrow<types::approvals::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::approvals::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::approvals::Param1,
                        >,
                    ),
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }
                pub fn approvals(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::approvals::Param0>,
                    _1: impl ::core::borrow::Borrow<types::approvals::Param1>,
                    _2: impl ::core::borrow::Borrow<types::approvals::Param2>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::approvals::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::approvals::Param1,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::approvals::Param2,
                        >,
                    ),
                    types::approvals::Approvals,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _2.borrow(),
                            ),
                        ),
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }
                pub fn metadata_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::metadata::Metadata,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Metadata",
                        (),
                        [
                            129u8, 202u8, 244u8, 77u8, 55u8, 81u8, 86u8, 106u8, 20u8, 153u8, 209u8,
                            69u8, 199u8, 107u8, 111u8, 49u8, 88u8, 157u8, 84u8, 41u8, 198u8, 190u8,
                            234u8, 218u8, 68u8, 207u8, 87u8, 217u8, 73u8, 66u8, 211u8, 163u8,
                        ],
                    )
                }
                pub fn metadata(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::metadata::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::metadata::Param0,
                    >,
                    types::metadata::Metadata,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Metadata",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            129u8, 202u8, 244u8, 77u8, 55u8, 81u8, 86u8, 106u8, 20u8, 153u8, 209u8,
                            69u8, 199u8, 107u8, 111u8, 49u8, 88u8, 157u8, 84u8, 41u8, 198u8, 190u8,
                            234u8, 218u8, 68u8, 207u8, 87u8, 217u8, 73u8, 66u8, 211u8, 163u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn remove_items_limit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "RemoveItemsLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn asset_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "AssetDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn asset_account_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "AssetAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "MetadataDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "MetadataDepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn approval_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "ApprovalDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn string_limit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "StringLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_btree_map {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedBTreeMap<_0, _1>(
                    pub ::subxt::ext::subxt_core::utils::KeyedVec<_0, _1>,
                );
            }
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeakBoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
        }
        pub mod contracts_node_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                ),
                #[codec(index = 1)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 3)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 8)]
                Contracts(runtime_types::pallet_contracts::pallet::Call),
                #[codec(index = 9)]
                Assets(runtime_types::pallet_assets::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 2)]
                Utility(runtime_types::pallet_utility::pallet::Error),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 8)]
                Contracts(runtime_types::pallet_contracts::pallet::Error),
                #[codec(index = 9)]
                Assets(runtime_types::pallet_assets::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 2)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 6)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 8)]
                Contracts(runtime_types::pallet_contracts::pallet::Event),
                #[codec(index = 9)]
                Assets(runtime_types::pallet_assets::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {
                #[codec(index = 8)]
                Contracts(runtime_types::pallet_contracts::pallet::HoldReason),
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    remark {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    set_code {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    set_code_without_checks {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_storage {
                        items: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    kill_storage {
                        keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    kill_prefix {
                        prefix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    remark_with_event {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    authorize_upgrade {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    authorize_upgrade_without_checks {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 11)]
                    apply_authorized_upgrade {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                    #[codec(index = 6)]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    NothingAuthorized,
                    #[codec(index = 8)]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    KilledAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    Remarked {
                        sender: ::subxt::ext::subxt_core::utils::AccountId32,
                        hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    UpgradeAuthorized {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt::ext::subxt_core::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    start_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    destroy_accounts {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    destroy_approvals {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    finish_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 12)]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 14)]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 15)]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 16)]
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 17)]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 18)]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 20)]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 21)]
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 22)]
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 24)]
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        delegate: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 25)]
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        destination: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    touch {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 27)]
                    refund {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        allow_burn: ::core::primitive::bool,
                    },
                    #[codec(index = 28)]
                    set_min_balance {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 29)]
                    touch_other {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 30)]
                    refund_other {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 31)]
                    block {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    NoAccount,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    UnavailableConsumer,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                    #[codec(index = 12)]
                    AlreadyExists,
                    #[codec(index = 13)]
                    NoDeposit,
                    #[codec(index = 14)]
                    WouldBurn,
                    #[codec(index = 15)]
                    LiveAsset,
                    #[codec(index = 16)]
                    AssetNotLive,
                    #[codec(index = 17)]
                    IncorrectStatus,
                    #[codec(index = 18)]
                    NotFrozen,
                    #[codec(index = 19)]
                    CallbackFailed,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::ext::subxt_core::utils::AccountId32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::ext::subxt_core::utils::AccountId32,
                        admin: ::subxt::ext::subxt_core::utils::AccountId32,
                        freezer: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    AssetFrozen { asset_id: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    AssetThawed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    AccountsDestroyed {
                        asset_id: ::core::primitive::u32,
                        accounts_destroyed: ::core::primitive::u32,
                        accounts_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    ApprovalsDestroyed {
                        asset_id: ::core::primitive::u32,
                        approvals_destroyed: ::core::primitive::u32,
                        approvals_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    DestructionStarted { asset_id: ::core::primitive::u32 },
                    #[codec(index = 13)]
                    Destroyed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    ForceCreated {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    MetadataSet {
                        asset_id: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    MetadataCleared { asset_id: ::core::primitive::u32 },
                    #[codec(index = 17)]
                    ApprovedTransfer {
                        asset_id: ::core::primitive::u32,
                        source: ::subxt::ext::subxt_core::utils::AccountId32,
                        delegate: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        delegate: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 19)]
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        delegate: ::subxt::ext::subxt_core::utils::AccountId32,
                        destination: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                    #[codec(index = 21)]
                    AssetMinBalanceChanged {
                        asset_id: ::core::primitive::u32,
                        new_min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 22)]
                    Touched {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        depositor: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 23)]
                    Blocked {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AccountStatus {
                    #[codec(index = 0)]
                    Liquid,
                    #[codec(index = 1)]
                    Frozen,
                    #[codec(index = 2)]
                    Blocked,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AssetAccount<_0, _1, _2, _3> {
                    pub balance: _0,
                    pub status: runtime_types::pallet_assets::types::AccountStatus,
                    pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0, _3>,
                    pub extra: _2,
                    #[codec(skip)]
                    pub __ignore: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _2,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub status: runtime_types::pallet_assets::types::AssetStatus,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AssetStatus {
                    #[codec(index = 0)]
                    Live,
                    #[codec(index = 1)]
                    Frozen,
                    #[codec(index = 2)]
                    Destroying,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExistenceReason<_0, _1> {
                    #[codec(index = 0)]
                    Consumer,
                    #[codec(index = 1)]
                    Sufficient,
                    #[codec(index = 2)]
                    DepositHeld(_0),
                    #[codec(index = 3)]
                    DepositRefunded,
                    #[codec(index = 4)]
                    DepositFrom(_1, _0),
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer_allow_death {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    upgrade_accounts {
                        who: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 8)]
                    force_set_balance {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    Expendability,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                    #[codec(index = 8)]
                    TooManyHolds,
                    #[codec(index = 9)]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    DeltaZero,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    Minted {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    Burned {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    Suspended {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    Restored {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    Upgraded {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    Locked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    Unlocked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    Frozen {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    Thawed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 21)]
                    TotalIssuanceForced {
                        old: ::core::primitive::u128,
                        new: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    call_old_weight {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    instantiate_with_code_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    instantiate_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    upload_code {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        determinism: runtime_types::pallet_contracts::wasm::Determinism,
                    },
                    #[codec(index = 4)]
                    remove_code {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 5)]
                    set_code {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    call {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    instantiate_with_code {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    instantiate {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::subxt_core::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    migrate {
                        weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSchedule,
                    #[codec(index = 1)]
                    InvalidCallFlags,
                    #[codec(index = 2)]
                    OutOfGas,
                    #[codec(index = 3)]
                    OutputBufferTooSmall,
                    #[codec(index = 4)]
                    TransferFailed,
                    #[codec(index = 5)]
                    MaxCallDepthReached,
                    #[codec(index = 6)]
                    ContractNotFound,
                    #[codec(index = 7)]
                    CodeTooLarge,
                    #[codec(index = 8)]
                    CodeNotFound,
                    #[codec(index = 9)]
                    CodeInfoNotFound,
                    #[codec(index = 10)]
                    OutOfBounds,
                    #[codec(index = 11)]
                    DecodingFailed,
                    #[codec(index = 12)]
                    ContractTrapped,
                    #[codec(index = 13)]
                    ValueTooLarge,
                    #[codec(index = 14)]
                    TerminatedWhileReentrant,
                    #[codec(index = 15)]
                    InputForwarded,
                    #[codec(index = 16)]
                    RandomSubjectTooLong,
                    #[codec(index = 17)]
                    TooManyTopics,
                    #[codec(index = 18)]
                    NoChainExtension,
                    #[codec(index = 19)]
                    XCMDecodeFailed,
                    #[codec(index = 20)]
                    DuplicateContract,
                    #[codec(index = 21)]
                    TerminatedInConstructor,
                    #[codec(index = 22)]
                    ReentranceDenied,
                    #[codec(index = 23)]
                    StorageDepositNotEnoughFunds,
                    #[codec(index = 24)]
                    StorageDepositLimitExhausted,
                    #[codec(index = 25)]
                    CodeInUse,
                    #[codec(index = 26)]
                    ContractReverted,
                    #[codec(index = 27)]
                    CodeRejected,
                    #[codec(index = 28)]
                    Indeterministic,
                    #[codec(index = 29)]
                    MigrationInProgress,
                    #[codec(index = 30)]
                    NoMigrationPerformed,
                    #[codec(index = 31)]
                    MaxDelegateDependenciesReached,
                    #[codec(index = 32)]
                    DelegateDependencyNotFound,
                    #[codec(index = 33)]
                    DelegateDependencyAlreadyExists,
                    #[codec(index = 34)]
                    CannotAddSelfAsDelegateDependency,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Instantiated {
                        deployer: ::subxt::ext::subxt_core::utils::AccountId32,
                        contract: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    Terminated {
                        contract: ::subxt::ext::subxt_core::utils::AccountId32,
                        beneficiary: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    CodeStored {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        deposit_held: ::core::primitive::u128,
                        uploader: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    ContractEmitted {
                        contract: ::subxt::ext::subxt_core::utils::AccountId32,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    CodeRemoved {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        deposit_released: ::core::primitive::u128,
                        remover: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    ContractCodeUpdated {
                        contract: ::subxt::ext::subxt_core::utils::AccountId32,
                        new_code_hash: ::subxt::ext::subxt_core::utils::H256,
                        old_code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    Called {
                        caller: runtime_types::pallet_contracts::Origin<
                            runtime_types::contracts_node_runtime::Runtime,
                        >,
                        contract: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    DelegateCalled {
                        contract: ::subxt::ext::subxt_core::utils::AccountId32,
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 8)]
                    StorageDepositTransferredAndHeld {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    StorageDepositTransferredAndReleased {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    CodeUploadDepositReserve,
                    #[codec(index = 1)]
                    StorageDepositReserve,
                }
            }
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Code<_0> {
                    #[codec(index = 0)]
                    Upload(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 1)]
                    Existing(_0),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CodeUploadReturnValue<_0, _1> {
                    pub code_hash: _0,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ContractAccessError {
                    #[codec(index = 0)]
                    DoesntExist,
                    #[codec(index = 1)]
                    KeyDecodingFailed,
                    #[codec(index = 2)]
                    MigrationInProgress,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ContractResult<_0, _1, _2> {
                    pub gas_consumed: runtime_types::sp_weights::weight_v2::Weight,
                    pub gas_required: runtime_types::sp_weights::weight_v2::Weight,
                    pub storage_deposit:
                        runtime_types::pallet_contracts::primitives::StorageDeposit<_1>,
                    pub debug_message:
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub result: _0,
                    pub events:
                        ::core::option::Option<::subxt::ext::subxt_core::alloc::vec::Vec<_2>>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExecReturnValue {
                    pub flags: runtime_types::pallet_contracts_uapi::flags::ReturnFlags,
                    pub data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InstantiateReturnValue<_0> {
                    pub result: runtime_types::pallet_contracts::primitives::ExecReturnValue,
                    pub account_id: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum StorageDeposit<_0> {
                    #[codec(index = 0)]
                    Refund(_0),
                    #[codec(index = 1)]
                    Charge(_0),
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct HostFnWeights {
                    pub caller: runtime_types::sp_weights::weight_v2::Weight,
                    pub is_contract: runtime_types::sp_weights::weight_v2::Weight,
                    pub code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub own_code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub caller_is_origin: runtime_types::sp_weights::weight_v2::Weight,
                    pub caller_is_root: runtime_types::sp_weights::weight_v2::Weight,
                    pub address: runtime_types::sp_weights::weight_v2::Weight,
                    pub gas_left: runtime_types::sp_weights::weight_v2::Weight,
                    pub balance: runtime_types::sp_weights::weight_v2::Weight,
                    pub value_transferred: runtime_types::sp_weights::weight_v2::Weight,
                    pub minimum_balance: runtime_types::sp_weights::weight_v2::Weight,
                    pub block_number: runtime_types::sp_weights::weight_v2::Weight,
                    pub now: runtime_types::sp_weights::weight_v2::Weight,
                    pub weight_to_fee: runtime_types::sp_weights::weight_v2::Weight,
                    pub input: runtime_types::sp_weights::weight_v2::Weight,
                    pub input_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub r#return: runtime_types::sp_weights::weight_v2::Weight,
                    pub return_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub terminate: runtime_types::sp_weights::weight_v2::Weight,
                    pub random: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event_per_topic: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub debug_message: runtime_types::sp_weights::weight_v2::Weight,
                    pub debug_message_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage_per_new_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage_per_old_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub clear_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub clear_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub contains_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub contains_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub get_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub get_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub take_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub take_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub transfer: runtime_types::sp_weights::weight_v2::Weight,
                    pub call: runtime_types::sp_weights::weight_v2::Weight,
                    pub delegate_call: runtime_types::sp_weights::weight_v2::Weight,
                    pub call_transfer_surcharge: runtime_types::sp_weights::weight_v2::Weight,
                    pub call_per_cloned_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_transfer_surcharge:
                        runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_per_input_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_per_salt_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_sha2_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_sha2_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_keccak_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_keccak_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_128: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_128_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub ecdsa_recover: runtime_types::sp_weights::weight_v2::Weight,
                    pub ecdsa_to_eth_address: runtime_types::sp_weights::weight_v2::Weight,
                    pub sr25519_verify: runtime_types::sp_weights::weight_v2::Weight,
                    pub sr25519_verify_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub reentrance_count: runtime_types::sp_weights::weight_v2::Weight,
                    pub account_reentrance_count: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiation_nonce: runtime_types::sp_weights::weight_v2::Weight,
                    pub lock_delegate_dependency: runtime_types::sp_weights::weight_v2::Weight,
                    pub unlock_delegate_dependency: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InstructionWeights {
                    pub base: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                    pub runtime_memory: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ContractInfo {
                    pub trie_id: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub code_hash: ::subxt::ext::subxt_core::utils::H256,
                    pub storage_bytes: ::core::primitive::u32,
                    pub storage_items: ::core::primitive::u32,
                    pub storage_byte_deposit: ::core::primitive::u128,
                    pub storage_item_deposit: ::core::primitive::u128,
                    pub storage_base_deposit: ::core::primitive::u128,
                    pub delegate_dependencies:
                        runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                            ::subxt::ext::subxt_core::utils::H256,
                            ::core::primitive::u128,
                        >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DeletionQueueManager {
                    pub insert_counter: ::core::primitive::u32,
                    pub delete_counter: ::core::primitive::u32,
                }
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CodeInfo {
                    pub owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    #[codec(compact)]
                    pub deposit: ::core::primitive::u128,
                    #[codec(compact)]
                    pub refcount: ::core::primitive::u64,
                    pub determinism: runtime_types::pallet_contracts::wasm::Determinism,
                    pub code_len: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Determinism {
                    #[codec(index = 0)]
                    Enforced,
                    #[codec(index = 1)]
                    Relaxed,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ApiVersion(pub ::core::primitive::u16);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Environment {
                pub account_id: runtime_types::pallet_contracts::EnvironmentType<
                    ::subxt::ext::subxt_core::utils::AccountId32,
                >,
                pub balance:
                    runtime_types::pallet_contracts::EnvironmentType<::core::primitive::u128>,
                pub hash: runtime_types::pallet_contracts::EnvironmentType<
                    ::subxt::ext::subxt_core::utils::H256,
                >,
                pub hasher: runtime_types::pallet_contracts::EnvironmentType<
                    runtime_types::sp_runtime::traits::BlakeTwo256,
                >,
                pub timestamp:
                    runtime_types::pallet_contracts::EnvironmentType<::core::primitive::u64>,
                pub block_number:
                    runtime_types::pallet_contracts::EnvironmentType<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EnvironmentType<_0>(pub ::core::marker::PhantomData<_0>);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Origin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(::subxt::ext::subxt_core::utils::AccountId32),
                __Ignore(::core::marker::PhantomData<_0>),
            }
        }
        pub mod pallet_contracts_uapi {
            use super::runtime_types;
            pub mod flags {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ReturnFlags {
                    pub bits: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    remove_key,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        old: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                        new: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    KeyRemoved,
                    #[codec(index = 3)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    TransactionFeePaid {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FeeDetails<_0> {
                    pub inclusion_fee: ::core::option::Option<
                        runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                    >,
                    pub tip: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InclusionFee<_0> {
                    pub base_fee: _0,
                    pub len_fee: _0,
                    pub adjusted_weight_fee: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RuntimeDispatchInfo<_0, _1> {
                    pub weight: _1,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub partial_fee: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    batch {
                        calls: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 2)]
                    batch_all {
                        calls: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 3)]
                    dispatch_as {
                        as_origin: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::OriginCaller,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    force_batch {
                        calls: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 5)]
                    with_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::contracts_node_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    BatchCompletedWithErrors,
                    #[codec(index = 3)]
                    ItemCompleted,
                    #[codec(index = 4)]
                    ItemFailed {
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 5)]
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct OpaqueMetadata(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::ext::subxt_core::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Digest {
                        pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::ext::subxt_core::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::ext::subxt_core::utils::H256,
                        pub extrinsics_root: ::subxt::ext::subxt_core::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlakeTwo256;
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                    pub provides: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ExtrinsicInclusionMode {
                #[codec(index = 0)]
                AllExtrinsics,
                #[codec(index = 1)]
                OnlyInherents,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 64usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 64usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 65usize]),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub impl_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
