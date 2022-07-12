#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod types {
    pub mod pallet_offences {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                Offence {
                    kind: [u8; 16usize],
                    timeslot: Vec<u8>,
                },
            }
        }
    }
    pub mod pallet_utility {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Event {
            #[codec(index = 0u8)]
            BatchInterrupted(Vec<u32>, (u32, sp_runtime::DispatchError)),
            #[codec(index = 1u8)]
            BatchOptimisticFailed(Vec<u32>, Vec<(u32, sp_runtime::DispatchError)>),
            #[codec(index = 2u8)]
            BatchCompleted(Vec<u32>),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            InvalidSignature,
            #[codec(index = 1u8)]
            TargetCddMissing,
            #[codec(index = 2u8)]
            InvalidNonce,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct UniqueCall<C> {
            pub nonce: u64,
            pub call: ::std::boxed::Box<C>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            batch {
                calls: Vec<polymesh_runtime_develop::runtime::Call>,
            },
            #[codec(index = 1u8)]
            batch_atomic {
                calls: Vec<polymesh_runtime_develop::runtime::Call>,
            },
            #[codec(index = 2u8)]
            batch_optimistic {
                calls: Vec<polymesh_runtime_develop::runtime::Call>,
            },
            #[codec(index = 3u8)]
            relay_tx {
                target: sp_core::crypto::AccountId32,
                signature: sp_runtime::MultiSignature,
                call: pallet_utility::UniqueCall<polymesh_runtime_develop::runtime::Call>,
            },
        }
    }
    pub mod frame_system {
        use super::*;
        pub mod limits {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct WeightsPerClass {
                pub base_extrinsic: u64,
                pub max_extrinsic: Option<u64>,
                pub max_total: Option<u64>,
                pub reserved: Option<u64>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct BlockWeights {
                pub base_block: u64,
                pub max_block: u64,
                pub per_class:
                    frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct BlockLength {
                pub max: frame_support::weights::PerDispatchClass<u32>,
            }
        }
        pub mod extensions {
            use super::*;
            pub mod check_genesis {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct CheckGenesis();
            }
            pub mod check_mortality {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct CheckMortality(pub sp_runtime::generic::Era);
            }
            pub mod check_spec_version {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct CheckSpecVersion();
            }
            pub mod check_tx_version {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct CheckTxVersion();
            }
            pub mod check_nonce {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct CheckNonce(pub ::codec::Compact<u32>);
            }
            pub mod check_weight {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct CheckWeight();
            }
        }
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                ExtrinsicSuccess {
                    dispatch_info: frame_support::weights::DispatchInfo,
                },
                #[codec(index = 1u8)]
                ExtrinsicFailed {
                    dispatch_error: sp_runtime::DispatchError,
                    dispatch_info: frame_support::weights::DispatchInfo,
                },
                #[codec(index = 2u8)]
                CodeUpdated,
                #[codec(index = 3u8)]
                NewAccount {
                    account: sp_core::crypto::AccountId32,
                },
                #[codec(index = 4u8)]
                KilledAccount {
                    account: sp_core::crypto::AccountId32,
                },
                #[codec(index = 5u8)]
                Remarked {
                    sender: sp_core::crypto::AccountId32,
                    hash: primitive_types::H256,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                fill_block {
                    ratio: sp_arithmetic::per_things::Perbill,
                },
                #[codec(index = 1u8)]
                remark { remark: Vec<u8> },
                #[codec(index = 2u8)]
                set_heap_pages { pages: u64 },
                #[codec(index = 3u8)]
                set_code { code: Vec<u8> },
                #[codec(index = 4u8)]
                set_code_without_checks { code: Vec<u8> },
                #[codec(index = 5u8)]
                set_storage { items: Vec<(Vec<u8>, Vec<u8>)> },
                #[codec(index = 6u8)]
                kill_storage { keys: Vec<Vec<u8>> },
                #[codec(index = 7u8)]
                kill_prefix { prefix: Vec<u8>, subkeys: u32 },
                #[codec(index = 8u8)]
                remark_with_event { remark: Vec<u8> },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                InvalidSpecName,
                #[codec(index = 1u8)]
                SpecVersionNeedsToIncrease,
                #[codec(index = 2u8)]
                FailedToExtractRuntimeVersion,
                #[codec(index = 3u8)]
                NonDefaultComposite,
                #[codec(index = 4u8)]
                NonZeroRefCount,
                #[codec(index = 5u8)]
                CallFiltered,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Phase {
            #[codec(index = 0u8)]
            ApplyExtrinsic(u32),
            #[codec(index = 1u8)]
            Finalization,
            #[codec(index = 2u8)]
            Initialization,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct EventRecord<E, T> {
            pub phase: frame_system::Phase,
            pub event: E,
            pub topics: Vec<T>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct LastRuntimeUpgradeInfo {
            pub spec_version: ::codec::Compact<u32>,
            pub spec_name: String,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct AccountInfo<Index, AccountData> {
            pub nonce: Index,
            pub consumers: Index,
            pub providers: Index,
            pub sufficients: Index,
            pub data: AccountData,
        }
    }
    pub mod pallet_scheduler {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                schedule {
                    when: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: ::std::boxed::Box<
                        frame_support::traits::schedule::MaybeHashed<
                            polymesh_runtime_develop::runtime::Call,
                            primitive_types::H256,
                        >,
                    >,
                },
                #[codec(index = 1u8)]
                cancel { when: u32, index: u32 },
                #[codec(index = 2u8)]
                schedule_named {
                    id: Vec<u8>,
                    when: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: ::std::boxed::Box<
                        frame_support::traits::schedule::MaybeHashed<
                            polymesh_runtime_develop::runtime::Call,
                            primitive_types::H256,
                        >,
                    >,
                },
                #[codec(index = 3u8)]
                cancel_named { id: Vec<u8> },
                #[codec(index = 4u8)]
                schedule_after {
                    after: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: ::std::boxed::Box<
                        frame_support::traits::schedule::MaybeHashed<
                            polymesh_runtime_develop::runtime::Call,
                            primitive_types::H256,
                        >,
                    >,
                },
                #[codec(index = 5u8)]
                schedule_named_after {
                    id: Vec<u8>,
                    after: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: ::std::boxed::Box<
                        frame_support::traits::schedule::MaybeHashed<
                            polymesh_runtime_develop::runtime::Call,
                            primitive_types::H256,
                        >,
                    >,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                FailedToSchedule,
                #[codec(index = 1u8)]
                NotFound,
                #[codec(index = 2u8)]
                TargetBlockNumberInPast,
                #[codec(index = 3u8)]
                RescheduleNoChange,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                Scheduled { when: u32, index: u32 },
                #[codec(index = 1u8)]
                Canceled { when: u32, index: u32 },
                #[codec(index = 2u8)]
                Dispatched {
                    task: (u32, u32),
                    id: Option<Vec<u8>>,
                    result: Result<(), sp_runtime::DispatchError>,
                },
                #[codec(index = 3u8)]
                CallLookupFailed {
                    task: (u32, u32),
                    id: Option<Vec<u8>>,
                    error: frame_support::traits::schedule::LookupError,
                },
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ScheduledV3<Call, BlockNumber, PalletsOrigin, AccountId> {
            pub maybe_id: Option<Vec<u8>>,
            pub priority: u8,
            pub call: Call,
            pub maybe_periodic: Option<(BlockNumber, BlockNumber)>,
            pub origin: PalletsOrigin,
            _phantom_data: core::marker::PhantomData<AccountId>,
        }
    }
    pub mod pallet_timestamp {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                set { now: ::codec::Compact<u64> },
            }
        }
    }
    pub mod pallet_authorship {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                set_uncles {
                    new_uncles:
                        Vec<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                InvalidUncleParent,
                #[codec(index = 1u8)]
                UnclesAlreadySet,
                #[codec(index = 2u8)]
                TooManyUncles,
                #[codec(index = 3u8)]
                GenesisUncle,
                #[codec(index = 4u8)]
                TooHighUncle,
                #[codec(index = 5u8)]
                UncleAlreadyIncluded,
                #[codec(index = 6u8)]
                OldUncle,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum UncleEntryItem<BlockNumber, Hash, Author> {
            #[codec(index = 0u8)]
            InclusionHeight(BlockNumber),
            #[codec(index = 1u8)]
            Uncle(Hash, Option<Author>),
        }
    }
    pub mod polymesh_runtime_develop {
        use super::*;
        pub mod runtime {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct SessionKeys {
                pub grandpa: sp_finality_grandpa::app::Public,
                pub babe: sp_consensus_babe::app::Public,
                pub im_online: pallet_im_online::sr25519::app_sr25519::Public,
                pub authority_discovery: sp_authority_discovery::app::Public,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                System(frame_system::pallet::Call),
                #[codec(index = 1u8)]
                Babe(pallet_babe::pallet::Call),
                #[codec(index = 2u8)]
                Timestamp(pallet_timestamp::pallet::Call),
                #[codec(index = 3u8)]
                Indices(pallet_indices::pallet::Call),
                #[codec(index = 4u8)]
                Authorship(pallet_authorship::pallet::Call),
                #[codec(index = 5u8)]
                Balances(pallet_balances::Call),
                #[codec(index = 7u8)]
                Identity(pallet_identity::Call),
                #[codec(index = 8u8)]
                CddServiceProviders(pallet_group::Call),
                #[codec(index = 9u8)]
                PolymeshCommittee(pallet_committee::Call),
                #[codec(index = 10u8)]
                CommitteeMembership(pallet_group::Call),
                #[codec(index = 11u8)]
                TechnicalCommittee(pallet_committee::Call),
                #[codec(index = 12u8)]
                TechnicalCommitteeMembership(pallet_group::Call),
                #[codec(index = 13u8)]
                UpgradeCommittee(pallet_committee::Call),
                #[codec(index = 14u8)]
                UpgradeCommitteeMembership(pallet_group::Call),
                #[codec(index = 15u8)]
                MultiSig(pallet_multisig::Call),
                #[codec(index = 16u8)]
                Bridge(pallet_bridge::Call),
                #[codec(index = 17u8)]
                Staking(pallet_staking::Call),
                #[codec(index = 19u8)]
                Session(pallet_session::pallet::Call),
                #[codec(index = 21u8)]
                Grandpa(pallet_grandpa::pallet::Call),
                #[codec(index = 23u8)]
                ImOnline(pallet_im_online::pallet::Call),
                #[codec(index = 25u8)]
                Sudo(pallet_sudo::Call),
                #[codec(index = 26u8)]
                Asset(pallet_asset::Call),
                #[codec(index = 27u8)]
                CapitalDistribution(pallet_corporate_actions::distribution::Call),
                #[codec(index = 28u8)]
                Checkpoint(pallet_asset::checkpoint::Call),
                #[codec(index = 29u8)]
                ComplianceManager(pallet_compliance_manager::Call),
                #[codec(index = 30u8)]
                CorporateAction(pallet_corporate_actions::Call),
                #[codec(index = 31u8)]
                CorporateBallot(pallet_corporate_actions::ballot::Call),
                #[codec(index = 33u8)]
                Pips(pallet_pips::Call),
                #[codec(index = 34u8)]
                Portfolio(pallet_portfolio::Call),
                #[codec(index = 35u8)]
                ProtocolFee(pallet_protocol_fee::Call),
                #[codec(index = 36u8)]
                Scheduler(pallet_scheduler::pallet::Call),
                #[codec(index = 37u8)]
                Settlement(pallet_settlement::Call),
                #[codec(index = 38u8)]
                Statistics(pallet_statistics::Call),
                #[codec(index = 39u8)]
                Sto(pallet_sto::Call),
                #[codec(index = 40u8)]
                Treasury(pallet_treasury::Call),
                #[codec(index = 41u8)]
                Utility(pallet_utility::Call),
                #[codec(index = 42u8)]
                Base(pallet_base::Call),
                #[codec(index = 43u8)]
                ExternalAgents(pallet_external_agents::Call),
                #[codec(index = 44u8)]
                Relayer(pallet_relayer::Call),
                #[codec(index = 45u8)]
                Rewards(pallet_rewards::Call),
                #[codec(index = 46u8)]
                Contracts(pallet_contracts::pallet::Call),
                #[codec(index = 47u8)]
                PolymeshContracts(polymesh_contracts::Call),
                #[codec(index = 48u8)]
                Preimage(pallet_preimage::pallet::Call),
                #[codec(index = 50u8)]
                TestUtils(pallet_test_utils::Call),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Runtime();
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum OriginCaller {
                #[codec(index = 0u8)]
                system(frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>),
                #[codec(index = 9u8)]
                PolymeshCommittee(
                    pallet_committee::RawOrigin<
                        sp_core::crypto::AccountId32,
                        pallet_committee::Instance1,
                    >,
                ),
                #[codec(index = 11u8)]
                TechnicalCommittee(
                    pallet_committee::RawOrigin<
                        sp_core::crypto::AccountId32,
                        pallet_committee::Instance3,
                    >,
                ),
                #[codec(index = 13u8)]
                UpgradeCommittee(
                    pallet_committee::RawOrigin<
                        sp_core::crypto::AccountId32,
                        pallet_committee::Instance4,
                    >,
                ),
                #[codec(index = 4u8)]
                Void(sp_core::Void),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                System(frame_system::pallet::Event),
                #[codec(index = 3u8)]
                Indices(pallet_indices::pallet::Event),
                #[codec(index = 5u8)]
                Balances(
                    polymesh_common_utilities::traits::balances::RawEvent<
                        sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 7u8)]
                Identity(
                    polymesh_common_utilities::traits::identity::RawEvent<
                        sp_core::crypto::AccountId32,
                        u64,
                    >,
                ),
                #[codec(index = 8u8)]
                CddServiceProviders(
                    polymesh_common_utilities::traits::group::RawEvent<
                        sp_core::crypto::AccountId32,
                        polymesh_runtime_develop::runtime::Event,
                        pallet_group::Instance2,
                    >,
                ),
                #[codec(index = 9u8)]
                PolymeshCommittee(
                    pallet_committee::RawEvent<
                        primitive_types::H256,
                        u32,
                        pallet_committee::Instance1,
                    >,
                ),
                #[codec(index = 10u8)]
                CommitteeMembership(
                    polymesh_common_utilities::traits::group::RawEvent<
                        sp_core::crypto::AccountId32,
                        polymesh_runtime_develop::runtime::Event,
                        pallet_group::Instance1,
                    >,
                ),
                #[codec(index = 11u8)]
                TechnicalCommittee(
                    pallet_committee::RawEvent<
                        primitive_types::H256,
                        u32,
                        pallet_committee::Instance3,
                    >,
                ),
                #[codec(index = 12u8)]
                TechnicalCommitteeMembership(
                    polymesh_common_utilities::traits::group::RawEvent<
                        sp_core::crypto::AccountId32,
                        polymesh_runtime_develop::runtime::Event,
                        pallet_group::Instance3,
                    >,
                ),
                #[codec(index = 13u8)]
                UpgradeCommittee(
                    pallet_committee::RawEvent<
                        primitive_types::H256,
                        u32,
                        pallet_committee::Instance4,
                    >,
                ),
                #[codec(index = 14u8)]
                UpgradeCommitteeMembership(
                    polymesh_common_utilities::traits::group::RawEvent<
                        sp_core::crypto::AccountId32,
                        polymesh_runtime_develop::runtime::Event,
                        pallet_group::Instance4,
                    >,
                ),
                #[codec(index = 15u8)]
                MultiSig(pallet_multisig::RawEvent<sp_core::crypto::AccountId32>),
                #[codec(index = 16u8)]
                Bridge(pallet_bridge::RawEvent<sp_core::crypto::AccountId32, u32>),
                #[codec(index = 17u8)]
                Staking(pallet_staking::RawEvent<u128, sp_core::crypto::AccountId32>),
                #[codec(index = 18u8)]
                Offences(pallet_offences::pallet::Event),
                #[codec(index = 19u8)]
                Session(pallet_session::pallet::Event),
                #[codec(index = 21u8)]
                Grandpa(pallet_grandpa::pallet::Event),
                #[codec(index = 23u8)]
                ImOnline(pallet_im_online::pallet::Event),
                #[codec(index = 25u8)]
                Sudo(pallet_sudo::RawEvent<sp_core::crypto::AccountId32>),
                #[codec(index = 26u8)]
                Asset(
                    polymesh_common_utilities::traits::asset::RawEvent<
                        u64,
                        sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 27u8)]
                CapitalDistribution(pallet_corporate_actions::distribution::Event),
                #[codec(index = 28u8)]
                Checkpoint(polymesh_common_utilities::traits::checkpoint::Event),
                #[codec(index = 29u8)]
                ComplianceManager(pallet_compliance_manager::Event),
                #[codec(index = 30u8)]
                CorporateAction(pallet_corporate_actions::Event),
                #[codec(index = 31u8)]
                CorporateBallot(pallet_corporate_actions::ballot::Event),
                #[codec(index = 33u8)]
                Pips(pallet_pips::RawEvent<sp_core::crypto::AccountId32, u32>),
                #[codec(index = 34u8)]
                Portfolio(polymesh_common_utilities::traits::portfolio::Event),
                #[codec(index = 35u8)]
                ProtocolFee(pallet_protocol_fee::RawEvent<sp_core::crypto::AccountId32>),
                #[codec(index = 36u8)]
                Scheduler(pallet_scheduler::pallet::Event),
                #[codec(index = 37u8)]
                Settlement(pallet_settlement::RawEvent<u64, u32, sp_core::crypto::AccountId32>),
                #[codec(index = 38u8)]
                Statistics(polymesh_common_utilities::traits::statistics::Event),
                #[codec(index = 39u8)]
                Sto(pallet_sto::RawEvent<u64>),
                #[codec(index = 40u8)]
                Treasury(pallet_treasury::RawEvent<u128, sp_core::crypto::AccountId32>),
                #[codec(index = 41u8)]
                Utility(pallet_utility::Event),
                #[codec(index = 42u8)]
                Base(polymesh_common_utilities::traits::base::Event),
                #[codec(index = 43u8)]
                ExternalAgents(polymesh_common_utilities::traits::external_agents::Event),
                #[codec(index = 44u8)]
                Relayer(
                    polymesh_common_utilities::traits::relayer::RawEvent<
                        sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 45u8)]
                Rewards(pallet_rewards::RawEvent<sp_core::crypto::AccountId32>),
                #[codec(index = 46u8)]
                Contracts(pallet_contracts::pallet::Event),
                #[codec(index = 47u8)]
                PolymeshContracts(polymesh_contracts::Event),
                #[codec(index = 48u8)]
                Preimage(pallet_preimage::pallet::Event),
                #[codec(index = 50u8)]
                TestUtils(pallet_test_utils::RawEvent<sp_core::crypto::AccountId32>),
            }
        }
    }
    pub mod pallet_sudo {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            sudo {
                call: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
            },
            #[codec(index = 1u8)]
            sudo_unchecked_weight {
                call: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
                _weight: u64,
            },
            #[codec(index = 2u8)]
            set_key {
                new: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
            },
            #[codec(index = 3u8)]
            sudo_as {
                who: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                call: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId> {
            #[codec(index = 0u8)]
            Sudid(Result<(), sp_runtime::DispatchError>),
            #[codec(index = 1u8)]
            KeyChanged(AccountId),
            #[codec(index = 2u8)]
            SudoAsDone(Result<(), sp_runtime::DispatchError>),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            RequireSudo,
        }
    }
    pub mod pallet_identity {
        use super::*;
        pub mod types {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Claim1stKey {
                pub target: polymesh_primitives::identity_id::IdentityId,
                pub claim_type: polymesh_primitives::identity_claim::ClaimType,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Claim2ndKey {
                pub issuer: polymesh_primitives::identity_id::IdentityId,
                pub scope: Option<polymesh_primitives::identity_claim::Scope>,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            cdd_register_did {
                target_account: sp_core::crypto::AccountId32,
                secondary_keys: Vec<
                    polymesh_primitives::secondary_key::SecondaryKey<sp_core::crypto::AccountId32>,
                >,
            },
            #[codec(index = 1u8)]
            invalidate_cdd_claims {
                cdd: polymesh_primitives::identity_id::IdentityId,
                disable_from: u64,
                expiry: Option<u64>,
            },
            #[codec(index = 2u8)]
            remove_secondary_keys_old {
                keys_to_remove: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
            },
            #[codec(index = 3u8)]
            accept_primary_key {
                rotation_auth_id: u64,
                optional_cdd_auth_id: Option<u64>,
            },
            #[codec(index = 4u8)]
            change_cdd_requirement_for_mk_rotation { auth_required: bool },
            #[codec(index = 5u8)]
            join_identity_as_key { auth_id: u64 },
            #[codec(index = 6u8)]
            leave_identity_as_key,
            #[codec(index = 7u8)]
            add_claim {
                target: polymesh_primitives::identity_id::IdentityId,
                claim: polymesh_primitives::identity_claim::Claim,
                expiry: Option<u64>,
            },
            #[codec(index = 8u8)]
            revoke_claim {
                target: polymesh_primitives::identity_id::IdentityId,
                claim: polymesh_primitives::identity_claim::Claim,
            },
            #[codec(index = 9u8)]
            set_permission_to_signer {
                key: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                perms: polymesh_primitives::secondary_key::Permissions,
            },
            #[codec(index = 10u8)]
            placeholder_legacy_set_permission_to_signer,
            #[codec(index = 11u8)]
            freeze_secondary_keys,
            #[codec(index = 12u8)]
            unfreeze_secondary_keys,
            #[codec(index = 13u8)]
            add_authorization {
                target: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                data: polymesh_primitives::authorization::AuthorizationData<
                    sp_core::crypto::AccountId32,
                >,
                expiry: Option<u64>,
            },
            #[codec(index = 14u8)]
            remove_authorization {
                target: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                auth_id: u64,
                _auth_issuer_pays: bool,
            },
            #[codec(index = 15u8)]
            add_secondary_keys_with_authorization_old {
                additional_keys: Vec<
                    polymesh_common_utilities::traits::identity::SecondaryKeyWithAuthV1<
                        sp_core::crypto::AccountId32,
                    >,
                >,
                expires_at: u64,
            },
            #[codec(index = 16u8)]
            add_investor_uniqueness_claim {
                target: polymesh_primitives::identity_id::IdentityId,
                claim: polymesh_primitives::identity_claim::Claim,
                proof: [u8; 64usize],
                expiry: Option<u64>,
            },
            #[codec(index = 17u8)]
            gc_add_cdd_claim {
                target: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 18u8)]
            gc_revoke_cdd_claim {
                target: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 19u8)]
            add_investor_uniqueness_claim_v2 {
                target: polymesh_primitives::identity_id::IdentityId,
                scope: polymesh_primitives::identity_claim::Scope,
                claim: polymesh_primitives::identity_claim::Claim,
                proof: confidential_identity::claim_proofs::ScopeClaimProof,
                expiry: Option<u64>,
            },
            #[codec(index = 20u8)]
            revoke_claim_by_index {
                target: polymesh_primitives::identity_id::IdentityId,
                claim_type: polymesh_primitives::identity_claim::ClaimType,
                scope: Option<polymesh_primitives::identity_claim::Scope>,
            },
            #[codec(index = 21u8)]
            rotate_primary_key_to_secondary {
                auth_id: u64,
                optional_cdd_auth_id: Option<u64>,
            },
            #[codec(index = 22u8)]
            add_secondary_keys_with_authorization {
                additional_keys: Vec<
                    polymesh_common_utilities::traits::identity::SecondaryKeyWithAuth<
                        sp_core::crypto::AccountId32,
                    >,
                >,
                expires_at: u64,
            },
            #[codec(index = 23u8)]
            set_secondary_key_permissions {
                key: sp_core::crypto::AccountId32,
                perms: polymesh_primitives::secondary_key::Permissions,
            },
            #[codec(index = 24u8)]
            remove_secondary_keys {
                keys_to_remove: Vec<sp_core::crypto::AccountId32>,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            AlreadyLinked,
            #[codec(index = 1u8)]
            MissingCurrentIdentity,
            #[codec(index = 2u8)]
            Unauthorized,
            #[codec(index = 3u8)]
            InvalidAccountKey,
            #[codec(index = 4u8)]
            UnAuthorizedCddProvider,
            #[codec(index = 5u8)]
            InvalidAuthorizationFromOwner,
            #[codec(index = 6u8)]
            InvalidAuthorizationFromCddProvider,
            #[codec(index = 7u8)]
            NotCddProviderAttestation,
            #[codec(index = 8u8)]
            AuthorizationsNotForSameDids,
            #[codec(index = 9u8)]
            DidMustAlreadyExist,
            #[codec(index = 10u8)]
            CurrentIdentityCannotBeForwarded,
            #[codec(index = 11u8)]
            AuthorizationExpired,
            #[codec(index = 12u8)]
            TargetHasNoCdd,
            #[codec(index = 13u8)]
            AuthorizationHasBeenRevoked,
            #[codec(index = 14u8)]
            InvalidAuthorizationSignature,
            #[codec(index = 15u8)]
            KeyNotAllowed,
            #[codec(index = 16u8)]
            NotPrimaryKey,
            #[codec(index = 17u8)]
            DidDoesNotExist,
            #[codec(index = 18u8)]
            DidAlreadyExists,
            #[codec(index = 19u8)]
            SecondaryKeysContainPrimaryKey,
            #[codec(index = 20u8)]
            FailedToChargeFee,
            #[codec(index = 21u8)]
            NotASigner,
            #[codec(index = 22u8)]
            CannotDecodeSignerAccountId,
            #[codec(index = 23u8)]
            MultiSigHasBalance,
            #[codec(index = 24u8)]
            ConfidentialScopeClaimNotAllowed,
            #[codec(index = 25u8)]
            InvalidScopeClaim,
            #[codec(index = 26u8)]
            ClaimVariantNotAllowed,
            #[codec(index = 27u8)]
            TargetHasNonZeroBalanceAtScopeId,
            #[codec(index = 28u8)]
            CDDIdNotUniqueForIdentity,
            #[codec(index = 29u8)]
            InvalidCDDId,
            #[codec(index = 30u8)]
            ClaimAndProofVersionsDoNotMatch,
            #[codec(index = 31u8)]
            AccountKeyIsBeingUsed,
            #[codec(index = 32u8)]
            CustomScopeTooLong,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
    }
    pub mod pallet_test_utils {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            register_did {
                uid: polymesh_primitives::cdd_id::InvestorUid,
                secondary_keys: Vec<
                    polymesh_primitives::secondary_key::SecondaryKey<sp_core::crypto::AccountId32>,
                >,
            },
            #[codec(index = 1u8)]
            mock_cdd_register_did {
                target_account: sp_core::crypto::AccountId32,
            },
            #[codec(index = 2u8)]
            get_my_did,
            #[codec(index = 3u8)]
            get_cdd_of { of: sp_core::crypto::AccountId32 },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {}
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId> {
            #[codec(index = 0u8)]
            MockInvestorUIDCreated(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::cdd_id::InvestorUid,
            ),
            #[codec(index = 1u8)]
            DidStatus(polymesh_primitives::identity_id::IdentityId, AccountId),
            #[codec(index = 2u8)]
            CddStatus(
                Option<polymesh_primitives::identity_id::IdentityId>,
                AccountId,
                bool,
            ),
        }
    }
    pub mod pallet_protocol_fee {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            change_coefficient {
                coefficient: polymesh_primitives::PosRatio,
            },
            #[codec(index = 1u8)]
            change_base_fee {
                op: polymesh_common_utilities::protocol_fee::ProtocolOp,
                base_fee: u128,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId> {
            #[codec(index = 0u8)]
            FeeSet(polymesh_primitives::identity_id::IdentityId, u128),
            #[codec(index = 1u8)]
            CoefficientSet(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::PosRatio,
            ),
            #[codec(index = 2u8)]
            FeeCharged(AccountId, u128),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            InsufficientAccountBalance,
            #[codec(index = 1u8)]
            UnHandledImbalances,
            #[codec(index = 2u8)]
            InsufficientSubsidyBalance,
        }
    }
    pub mod sp_consensus_babe {
        use super::*;
        pub mod app {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Public(pub sp_core::sr25519::Public);
        }
        pub mod digests {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum NextConfigDescriptor {
                #[codec(index = 1u8)]
                V1 {
                    c: (u64, u64),
                    allowed_slots: sp_consensus_babe::AllowedSlots,
                },
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct BabeEpochConfiguration {
            pub c: (u64, u64),
            pub allowed_slots: sp_consensus_babe::AllowedSlots,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum AllowedSlots {
            #[codec(index = 0u8)]
            PrimarySlots,
            #[codec(index = 1u8)]
            PrimaryAndSecondaryPlainSlots,
            #[codec(index = 2u8)]
            PrimaryAndSecondaryVRFSlots,
        }
    }
    pub mod sp_consensus_slots {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Slot(pub u64);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct EquivocationProof<Header, Id> {
            pub offender: Id,
            pub slot: sp_consensus_slots::Slot,
            pub first_header: Header,
            pub second_header: Header,
        }
    }
    pub mod primitive_types {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct H256(pub [u8; 32usize]);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct H512(pub [u8; 64usize]);
    }
    pub mod sp_finality_grandpa {
        use super::*;
        pub mod app {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Public(pub sp_core::ed25519::Public);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Signature(pub sp_core::ed25519::Signature);
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Equivocation<H, N> {
            #[codec(index = 0u8)]
            Prevote(
                finality_grandpa::Equivocation<
                    sp_finality_grandpa::app::Public,
                    finality_grandpa::Prevote<H, N>,
                    sp_finality_grandpa::app::Signature,
                >,
            ),
            #[codec(index = 1u8)]
            Precommit(
                finality_grandpa::Equivocation<
                    sp_finality_grandpa::app::Public,
                    finality_grandpa::Precommit<H, N>,
                    sp_finality_grandpa::app::Signature,
                >,
            ),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct EquivocationProof<H, N> {
            pub set_id: u64,
            pub equivocation: sp_finality_grandpa::Equivocation<H, N>,
        }
    }
    pub mod pallet_corporate_actions {
        use super::*;
        pub mod ballot {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Motion {
                pub title: pallet_corporate_actions::ballot::MotionTitle,
                pub info_link: pallet_corporate_actions::ballot::MotionInfoLink,
                pub choices: Vec<pallet_corporate_actions::ballot::ChoiceTitle>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct BallotTimeRange {
                pub start: u64,
                pub end: u64,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct BallotTitle(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct ChoiceTitle(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct MotionInfoLink(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct BallotMeta {
                pub title: pallet_corporate_actions::ballot::BallotTitle,
                pub motions: Vec<pallet_corporate_actions::ballot::Motion>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                CANotNotice,
                #[codec(index = 1u8)]
                AlreadyExists,
                #[codec(index = 2u8)]
                NoSuchBallot,
                #[codec(index = 3u8)]
                StartAfterEnd,
                #[codec(index = 4u8)]
                NowAfterEnd,
                #[codec(index = 5u8)]
                NumberOfChoicesOverflow,
                #[codec(index = 6u8)]
                VotingAlreadyStarted,
                #[codec(index = 7u8)]
                VotingNotStarted,
                #[codec(index = 8u8)]
                VotingAlreadyEnded,
                #[codec(index = 9u8)]
                WrongVoteCount,
                #[codec(index = 10u8)]
                InsufficientVotes,
                #[codec(index = 11u8)]
                NoSuchRCVFallback,
                #[codec(index = 12u8)]
                RCVSelfCycle,
                #[codec(index = 13u8)]
                RCVNotAllowed,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                attach_ballot {
                    ca_id: pallet_corporate_actions::CAId,
                    range: pallet_corporate_actions::ballot::BallotTimeRange,
                    meta: pallet_corporate_actions::ballot::BallotMeta,
                    rcv: bool,
                },
                #[codec(index = 1u8)]
                vote {
                    ca_id: pallet_corporate_actions::CAId,
                    votes: Vec<pallet_corporate_actions::ballot::BallotVote>,
                },
                #[codec(index = 2u8)]
                change_end {
                    ca_id: pallet_corporate_actions::CAId,
                    end: u64,
                },
                #[codec(index = 3u8)]
                change_meta {
                    ca_id: pallet_corporate_actions::CAId,
                    meta: pallet_corporate_actions::ballot::BallotMeta,
                },
                #[codec(index = 4u8)]
                change_rcv {
                    ca_id: pallet_corporate_actions::CAId,
                    rcv: bool,
                },
                #[codec(index = 5u8)]
                remove_ballot {
                    ca_id: pallet_corporate_actions::CAId,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct MotionTitle(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                Created(
                    polymesh_primitives::identity_id::IdentityId,
                    pallet_corporate_actions::CAId,
                    pallet_corporate_actions::ballot::BallotTimeRange,
                    pallet_corporate_actions::ballot::BallotMeta,
                    bool,
                ),
                #[codec(index = 1u8)]
                VoteCast(
                    polymesh_primitives::identity_id::IdentityId,
                    pallet_corporate_actions::CAId,
                    Vec<pallet_corporate_actions::ballot::BallotVote>,
                ),
                #[codec(index = 2u8)]
                RangeChanged(
                    polymesh_primitives::identity_id::IdentityId,
                    pallet_corporate_actions::CAId,
                    pallet_corporate_actions::ballot::BallotTimeRange,
                ),
                #[codec(index = 3u8)]
                MetaChanged(
                    polymesh_primitives::identity_id::IdentityId,
                    pallet_corporate_actions::CAId,
                    pallet_corporate_actions::ballot::BallotMeta,
                ),
                #[codec(index = 4u8)]
                RCVChanged(
                    polymesh_primitives::identity_id::IdentityId,
                    pallet_corporate_actions::CAId,
                    bool,
                ),
                #[codec(index = 5u8)]
                Removed(
                    polymesh_primitives::event_only::EventOnly<
                        polymesh_primitives::identity_id::IdentityId,
                    >,
                    pallet_corporate_actions::CAId,
                ),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct BallotVote {
                pub power: u128,
                pub fallback: Option<u16>,
            }
        }
        pub mod distribution {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                distribute {
                    ca_id: pallet_corporate_actions::CAId,
                    portfolio: Option<polymesh_primitives::identity_id::PortfolioNumber>,
                    currency: polymesh_primitives::ticker::Ticker,
                    per_share: u128,
                    amount: u128,
                    payment_at: u64,
                    expires_at: Option<u64>,
                },
                #[codec(index = 1u8)]
                claim {
                    ca_id: pallet_corporate_actions::CAId,
                },
                #[codec(index = 2u8)]
                push_benefit {
                    ca_id: pallet_corporate_actions::CAId,
                    holder: polymesh_primitives::identity_id::IdentityId,
                },
                #[codec(index = 3u8)]
                reclaim {
                    ca_id: pallet_corporate_actions::CAId,
                },
                #[codec(index = 4u8)]
                remove_distribution {
                    ca_id: pallet_corporate_actions::CAId,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                Created(
                    polymesh_primitives::event_only::EventOnly<
                        polymesh_primitives::identity_id::IdentityId,
                    >,
                    pallet_corporate_actions::CAId,
                    pallet_corporate_actions::distribution::Distribution,
                ),
                #[codec(index = 1u8)]
                BenefitClaimed(
                    polymesh_primitives::event_only::EventOnly<
                        polymesh_primitives::identity_id::IdentityId,
                    >,
                    polymesh_primitives::event_only::EventOnly<
                        polymesh_primitives::identity_id::IdentityId,
                    >,
                    pallet_corporate_actions::CAId,
                    pallet_corporate_actions::distribution::Distribution,
                    u128,
                    sp_arithmetic::per_things::Permill,
                ),
                #[codec(index = 2u8)]
                Reclaimed(
                    polymesh_primitives::event_only::EventOnly<
                        polymesh_primitives::identity_id::IdentityId,
                    >,
                    pallet_corporate_actions::CAId,
                    u128,
                ),
                #[codec(index = 3u8)]
                Removed(
                    polymesh_primitives::event_only::EventOnly<
                        polymesh_primitives::identity_id::IdentityId,
                    >,
                    pallet_corporate_actions::CAId,
                ),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Distribution {
                pub from: polymesh_primitives::identity_id::PortfolioId,
                pub currency: polymesh_primitives::ticker::Ticker,
                pub per_share: u128,
                pub amount: u128,
                pub remaining: u128,
                pub reclaimed: bool,
                pub payment_at: u64,
                pub expires_at: Option<u64>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Version(pub u8);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                CANotBenefit,
                #[codec(index = 1u8)]
                AlreadyExists,
                #[codec(index = 2u8)]
                ExpiryBeforePayment,
                #[codec(index = 3u8)]
                HolderAlreadyPaid,
                #[codec(index = 4u8)]
                NoSuchDistribution,
                #[codec(index = 5u8)]
                CannotClaimBeforeStart,
                #[codec(index = 6u8)]
                CannotClaimAfterExpiry,
                #[codec(index = 7u8)]
                BalancePerShareProductOverflowed,
                #[codec(index = 8u8)]
                NotDistributionCreator,
                #[codec(index = 9u8)]
                AlreadyReclaimed,
                #[codec(index = 10u8)]
                NotExpired,
                #[codec(index = 11u8)]
                DistributionStarted,
                #[codec(index = 12u8)]
                InsufficientRemainingAmount,
                #[codec(index = 13u8)]
                DistributionAmountIsZero,
                #[codec(index = 14u8)]
                DistributionPerShareIsZero,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct CADetails(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct CorporateAction {
            pub kind: pallet_corporate_actions::CAKind,
            pub decl_date: u64,
            pub record_date: Option<pallet_corporate_actions::RecordDate>,
            pub targets: pallet_corporate_actions::TargetIdentities,
            pub default_withholding_tax: sp_arithmetic::per_things::Permill,
            pub withholding_tax: Vec<(
                polymesh_primitives::identity_id::IdentityId,
                sp_arithmetic::per_things::Permill,
            )>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum CACheckpoint {
            #[codec(index = 0u8)]
            Scheduled(
                polymesh_common_utilities::traits::checkpoint::ScheduleId,
                u64,
            ),
            #[codec(index = 1u8)]
            Existing(polymesh_primitives::calendar::CheckpointId),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct InitiateCorporateActionArgs {
            pub ticker: polymesh_primitives::ticker::Ticker,
            pub kind: pallet_corporate_actions::CAKind,
            pub decl_date: u64,
            pub record_date: Option<pallet_corporate_actions::RecordDateSpec>,
            pub details: pallet_corporate_actions::CADetails,
            pub targets: Option<pallet_corporate_actions::TargetIdentities>,
            pub default_withholding_tax: Option<sp_arithmetic::per_things::Permill>,
            pub withholding_tax: Option<
                Vec<(
                    polymesh_primitives::identity_id::IdentityId,
                    sp_arithmetic::per_things::Permill,
                )>,
            >,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct CAId {
            pub ticker: polymesh_primitives::ticker::Ticker,
            pub local_id: pallet_corporate_actions::LocalCAId,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct LocalCAId(pub u32);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Event {
            #[codec(index = 0u8)]
            MaxDetailsLengthChanged(polymesh_primitives::identity_id::IdentityId, u32),
            #[codec(index = 1u8)]
            DefaultTargetIdentitiesChanged(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                pallet_corporate_actions::TargetIdentities,
            ),
            #[codec(index = 2u8)]
            DefaultWithholdingTaxChanged(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                sp_arithmetic::per_things::Permill,
            ),
            #[codec(index = 3u8)]
            DidWithholdingTaxChanged(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::identity_id::IdentityId,
                Option<sp_arithmetic::per_things::Permill>,
            ),
            #[codec(index = 4u8)]
            CAATransferred(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::identity_id::IdentityId,
            ),
            #[codec(index = 5u8)]
            CAInitiated(
                polymesh_primitives::event_only::EventOnly<
                    polymesh_primitives::identity_id::IdentityId,
                >,
                pallet_corporate_actions::CAId,
                pallet_corporate_actions::CorporateAction,
                pallet_corporate_actions::CADetails,
            ),
            #[codec(index = 6u8)]
            CALinkedToDoc(
                polymesh_primitives::identity_id::IdentityId,
                pallet_corporate_actions::CAId,
                Vec<polymesh_primitives::document::DocumentId>,
            ),
            #[codec(index = 7u8)]
            CARemoved(
                polymesh_primitives::event_only::EventOnly<
                    polymesh_primitives::identity_id::IdentityId,
                >,
                pallet_corporate_actions::CAId,
            ),
            #[codec(index = 8u8)]
            RecordDateChanged(
                polymesh_primitives::event_only::EventOnly<
                    polymesh_primitives::identity_id::IdentityId,
                >,
                pallet_corporate_actions::CAId,
                pallet_corporate_actions::CorporateAction,
            ),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct TargetIdentities {
            pub identities: Vec<polymesh_primitives::identity_id::IdentityId>,
            pub treatment: pallet_corporate_actions::TargetTreatment,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum CAKind {
            #[codec(index = 0u8)]
            PredictableBenefit,
            #[codec(index = 1u8)]
            UnpredictableBenefit,
            #[codec(index = 2u8)]
            IssuerNotice,
            #[codec(index = 3u8)]
            Reorganization,
            #[codec(index = 4u8)]
            Other,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RecordDateSpec {
            #[codec(index = 0u8)]
            Scheduled(u64),
            #[codec(index = 1u8)]
            ExistingSchedule(polymesh_common_utilities::traits::checkpoint::ScheduleId),
            #[codec(index = 2u8)]
            Existing(polymesh_primitives::calendar::CheckpointId),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct RecordDate {
            pub date: u64,
            pub checkpoint: pallet_corporate_actions::CACheckpoint,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            AuthNotCAATransfer,
            #[codec(index = 1u8)]
            DetailsTooLong,
            #[codec(index = 2u8)]
            DuplicateDidTax,
            #[codec(index = 3u8)]
            TooManyDidTaxes,
            #[codec(index = 4u8)]
            TooManyTargetIds,
            #[codec(index = 5u8)]
            NoSuchCheckpointId,
            #[codec(index = 6u8)]
            NoSuchCA,
            #[codec(index = 7u8)]
            NoRecordDate,
            #[codec(index = 8u8)]
            RecordDateAfterStart,
            #[codec(index = 9u8)]
            DeclDateAfterRecordDate,
            #[codec(index = 10u8)]
            DeclDateInFuture,
            #[codec(index = 11u8)]
            NotTargetedByCA,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            set_max_details_length { length: u32 },
            #[codec(index = 1u8)]
            set_default_targets {
                ticker: polymesh_primitives::ticker::Ticker,
                targets: pallet_corporate_actions::TargetIdentities,
            },
            #[codec(index = 2u8)]
            set_default_withholding_tax {
                ticker: polymesh_primitives::ticker::Ticker,
                tax: sp_arithmetic::per_things::Permill,
            },
            #[codec(index = 3u8)]
            set_did_withholding_tax {
                ticker: polymesh_primitives::ticker::Ticker,
                taxed_did: polymesh_primitives::identity_id::IdentityId,
                tax: Option<sp_arithmetic::per_things::Permill>,
            },
            #[codec(index = 4u8)]
            initiate_corporate_action {
                ticker: polymesh_primitives::ticker::Ticker,
                kind: pallet_corporate_actions::CAKind,
                decl_date: u64,
                record_date: Option<pallet_corporate_actions::RecordDateSpec>,
                details: pallet_corporate_actions::CADetails,
                targets: Option<pallet_corporate_actions::TargetIdentities>,
                default_withholding_tax: Option<sp_arithmetic::per_things::Permill>,
                withholding_tax: Option<
                    Vec<(
                        polymesh_primitives::identity_id::IdentityId,
                        sp_arithmetic::per_things::Permill,
                    )>,
                >,
            },
            #[codec(index = 5u8)]
            link_ca_doc {
                id: pallet_corporate_actions::CAId,
                docs: Vec<polymesh_primitives::document::DocumentId>,
            },
            #[codec(index = 6u8)]
            remove_ca {
                ca_id: pallet_corporate_actions::CAId,
            },
            #[codec(index = 7u8)]
            change_record_date {
                ca_id: pallet_corporate_actions::CAId,
                record_date: Option<pallet_corporate_actions::RecordDateSpec>,
            },
            #[codec(index = 8u8)]
            initiate_corporate_action_and_distribute {
                ca_args: pallet_corporate_actions::InitiateCorporateActionArgs,
                portfolio: Option<polymesh_primitives::identity_id::PortfolioNumber>,
                currency: polymesh_primitives::ticker::Ticker,
                per_share: u128,
                amount: u128,
                payment_at: u64,
                expires_at: Option<u64>,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum TargetTreatment {
            #[codec(index = 0u8)]
            Include,
            #[codec(index = 1u8)]
            Exclude,
        }
    }
    pub mod pallet_contracts {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                Instantiated {
                    deployer: sp_core::crypto::AccountId32,
                    contract: sp_core::crypto::AccountId32,
                },
                #[codec(index = 1u8)]
                Terminated {
                    contract: sp_core::crypto::AccountId32,
                    beneficiary: sp_core::crypto::AccountId32,
                },
                #[codec(index = 2u8)]
                CodeStored { code_hash: primitive_types::H256 },
                #[codec(index = 3u8)]
                ContractEmitted {
                    contract: sp_core::crypto::AccountId32,
                    data: Vec<u8>,
                },
                #[codec(index = 4u8)]
                CodeRemoved { code_hash: primitive_types::H256 },
                #[codec(index = 5u8)]
                ContractCodeUpdated {
                    contract: sp_core::crypto::AccountId32,
                    new_code_hash: primitive_types::H256,
                    old_code_hash: primitive_types::H256,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                call {
                    dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                    value: ::codec::Compact<u128>,
                    gas_limit: ::codec::Compact<u64>,
                    storage_deposit_limit: Option<::codec::Compact<u128>>,
                    data: Vec<u8>,
                },
                #[codec(index = 1u8)]
                instantiate_with_code {
                    value: ::codec::Compact<u128>,
                    gas_limit: ::codec::Compact<u64>,
                    storage_deposit_limit: Option<::codec::Compact<u128>>,
                    code: Vec<u8>,
                    data: Vec<u8>,
                    salt: Vec<u8>,
                },
                #[codec(index = 2u8)]
                instantiate {
                    value: ::codec::Compact<u128>,
                    gas_limit: ::codec::Compact<u64>,
                    storage_deposit_limit: Option<::codec::Compact<u128>>,
                    code_hash: primitive_types::H256,
                    data: Vec<u8>,
                    salt: Vec<u8>,
                },
                #[codec(index = 3u8)]
                upload_code {
                    code: Vec<u8>,
                    storage_deposit_limit: Option<::codec::Compact<u128>>,
                },
                #[codec(index = 4u8)]
                remove_code { code_hash: primitive_types::H256 },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                InvalidScheduleVersion,
                #[codec(index = 1u8)]
                InvalidCallFlags,
                #[codec(index = 2u8)]
                OutOfGas,
                #[codec(index = 3u8)]
                OutputBufferTooSmall,
                #[codec(index = 4u8)]
                TransferFailed,
                #[codec(index = 5u8)]
                MaxCallDepthReached,
                #[codec(index = 6u8)]
                ContractNotFound,
                #[codec(index = 7u8)]
                CodeTooLarge,
                #[codec(index = 8u8)]
                CodeNotFound,
                #[codec(index = 9u8)]
                OutOfBounds,
                #[codec(index = 10u8)]
                DecodingFailed,
                #[codec(index = 11u8)]
                ContractTrapped,
                #[codec(index = 12u8)]
                ValueTooLarge,
                #[codec(index = 13u8)]
                TerminatedWhileReentrant,
                #[codec(index = 14u8)]
                InputForwarded,
                #[codec(index = 15u8)]
                RandomSubjectTooLong,
                #[codec(index = 16u8)]
                TooManyTopics,
                #[codec(index = 17u8)]
                DuplicateTopics,
                #[codec(index = 18u8)]
                NoChainExtension,
                #[codec(index = 19u8)]
                DeletionQueueFull,
                #[codec(index = 20u8)]
                DuplicateContract,
                #[codec(index = 21u8)]
                TerminatedInConstructor,
                #[codec(index = 22u8)]
                DebugMessageInvalidUTF8,
                #[codec(index = 23u8)]
                ReentranceDenied,
                #[codec(index = 24u8)]
                StorageDepositNotEnoughFunds,
                #[codec(index = 25u8)]
                StorageDepositLimitExhausted,
                #[codec(index = 26u8)]
                CodeInUse,
                #[codec(index = 27u8)]
                ContractReverted,
                #[codec(index = 28u8)]
                CodeRejected,
            }
        }
        pub mod wasm {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct PrefabWasmModule {
                pub instruction_weights_version: ::codec::Compact<u32>,
                pub initial: ::codec::Compact<u32>,
                pub maximum: ::codec::Compact<u32>,
                pub code: Vec<u8>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct OwnerInfo {
                pub owner: sp_core::crypto::AccountId32,
                pub deposit: ::codec::Compact<u128>,
                pub refcount: ::codec::Compact<u64>,
            }
        }
        pub mod storage {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct RawContractInfo<CodeHash, Balance> {
                pub trie_id: Vec<u8>,
                pub code_hash: CodeHash,
                pub storage_deposit: Balance,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct DeletedContract {
                pub trie_id: Vec<u8>,
            }
        }
        pub mod schedule {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Schedule {
                pub limits: pallet_contracts::schedule::Limits,
                pub instruction_weights: pallet_contracts::schedule::InstructionWeights,
                pub host_fn_weights: pallet_contracts::schedule::HostFnWeights,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Limits {
                pub event_topics: u32,
                pub stack_height: Option<u32>,
                pub globals: u32,
                pub parameters: u32,
                pub memory_pages: u32,
                pub table_size: u32,
                pub br_table_size: u32,
                pub subject_len: u32,
                pub call_depth: u32,
                pub payload_len: u32,
                pub code_len: u32,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct InstructionWeights {
                pub version: u32,
                pub i64const: u32,
                pub i64load: u32,
                pub i64store: u32,
                pub select: u32,
                pub r#if: u32,
                pub br: u32,
                pub br_if: u32,
                pub br_table: u32,
                pub br_table_per_entry: u32,
                pub call: u32,
                pub call_indirect: u32,
                pub call_indirect_per_param: u32,
                pub local_get: u32,
                pub local_set: u32,
                pub local_tee: u32,
                pub global_get: u32,
                pub global_set: u32,
                pub memory_current: u32,
                pub memory_grow: u32,
                pub i64clz: u32,
                pub i64ctz: u32,
                pub i64popcnt: u32,
                pub i64eqz: u32,
                pub i64extendsi32: u32,
                pub i64extendui32: u32,
                pub i32wrapi64: u32,
                pub i64eq: u32,
                pub i64ne: u32,
                pub i64lts: u32,
                pub i64ltu: u32,
                pub i64gts: u32,
                pub i64gtu: u32,
                pub i64les: u32,
                pub i64leu: u32,
                pub i64ges: u32,
                pub i64geu: u32,
                pub i64add: u32,
                pub i64sub: u32,
                pub i64mul: u32,
                pub i64divs: u32,
                pub i64divu: u32,
                pub i64rems: u32,
                pub i64remu: u32,
                pub i64and: u32,
                pub i64or: u32,
                pub i64xor: u32,
                pub i64shl: u32,
                pub i64shrs: u32,
                pub i64shru: u32,
                pub i64rotl: u32,
                pub i64rotr: u32,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct HostFnWeights {
                pub caller: u64,
                pub is_contract: u64,
                pub code_hash: u64,
                pub own_code_hash: u64,
                pub caller_is_origin: u64,
                pub address: u64,
                pub gas_left: u64,
                pub balance: u64,
                pub value_transferred: u64,
                pub minimum_balance: u64,
                pub block_number: u64,
                pub now: u64,
                pub weight_to_fee: u64,
                pub gas: u64,
                pub input: u64,
                pub input_per_byte: u64,
                pub r#return: u64,
                pub return_per_byte: u64,
                pub terminate: u64,
                pub random: u64,
                pub deposit_event: u64,
                pub deposit_event_per_topic: u64,
                pub deposit_event_per_byte: u64,
                pub debug_message: u64,
                pub set_storage: u64,
                pub set_storage_per_new_byte: u64,
                pub set_storage_per_old_byte: u64,
                pub set_code_hash: u64,
                pub clear_storage: u64,
                pub clear_storage_per_byte: u64,
                pub contains_storage: u64,
                pub contains_storage_per_byte: u64,
                pub get_storage: u64,
                pub get_storage_per_byte: u64,
                pub take_storage: u64,
                pub take_storage_per_byte: u64,
                pub transfer: u64,
                pub call: u64,
                pub delegate_call: u64,
                pub call_transfer_surcharge: u64,
                pub call_per_cloned_byte: u64,
                pub instantiate: u64,
                pub instantiate_transfer_surcharge: u64,
                pub instantiate_per_salt_byte: u64,
                pub hash_sha2_256: u64,
                pub hash_sha2_256_per_byte: u64,
                pub hash_keccak_256: u64,
                pub hash_keccak_256_per_byte: u64,
                pub hash_blake2_256: u64,
                pub hash_blake2_256_per_byte: u64,
                pub hash_blake2_128: u64,
                pub hash_blake2_128_per_byte: u64,
                pub ecdsa_recover: u64,
                pub ecdsa_to_eth_address: u64,
            }
        }
    }
    pub mod pallet_babe {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                report_equivocation {
                    equivocation_proof: ::std::boxed::Box<
                        sp_consensus_slots::EquivocationProof<
                            sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>,
                            sp_consensus_babe::app::Public,
                        >,
                    >,
                    key_owner_proof: sp_session::MembershipProof,
                },
                #[codec(index = 1u8)]
                report_equivocation_unsigned {
                    equivocation_proof: ::std::boxed::Box<
                        sp_consensus_slots::EquivocationProof<
                            sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>,
                            sp_consensus_babe::app::Public,
                        >,
                    >,
                    key_owner_proof: sp_session::MembershipProof,
                },
                #[codec(index = 2u8)]
                plan_config_change {
                    config: sp_consensus_babe::digests::NextConfigDescriptor,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                InvalidEquivocationProof,
                #[codec(index = 1u8)]
                InvalidKeyOwnershipProof,
                #[codec(index = 2u8)]
                DuplicateOffenceReport,
            }
        }
    }
    pub mod pallet_transaction_payment {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Releases {
            #[codec(index = 0u8)]
            V1Ancient,
            #[codec(index = 1u8)]
            V2,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ChargeTransactionPayment(pub ::codec::Compact<u128>);
    }
    pub mod confidential_identity {
        use super::*;
        pub mod sign {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Signature {
                pub r: [u8; 32usize],
                pub s: [u8; 32usize],
            }
        }
        pub mod claim_proofs {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct ZkProofData {
                pub challenge_responses: [[u8; 32usize]; 2usize],
                pub subtract_expressions_res: [u8; 32usize],
                pub blinded_scope_did_hash: [u8; 32usize],
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct ScopeClaimProof {
                pub proof_scope_id_wellformed: confidential_identity::sign::Signature,
                pub proof_scope_id_cdd_id_match: confidential_identity::claim_proofs::ZkProofData,
                pub scope_id: [u8; 32usize],
            }
        }
    }
    pub mod pallet_group {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            set_active_members_limit { limit: u32 },
            #[codec(index = 1u8)]
            disable_member {
                who: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
                at: Option<u64>,
            },
            #[codec(index = 2u8)]
            add_member {
                who: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 3u8)]
            remove_member {
                who: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 4u8)]
            swap_member {
                remove: polymesh_primitives::identity_id::IdentityId,
                add: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 5u8)]
            reset_members {
                members: Vec<polymesh_primitives::identity_id::IdentityId>,
            },
            #[codec(index = 6u8)]
            abdicate_membership,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            OnlyPrimaryKeyAllowed,
            #[codec(index = 1u8)]
            DuplicateMember,
            #[codec(index = 2u8)]
            NoSuchMember,
            #[codec(index = 3u8)]
            LastMemberCannotQuit,
            #[codec(index = 4u8)]
            MissingCurrentIdentity,
            #[codec(index = 5u8)]
            ActiveMembersLimitExceeded,
            #[codec(index = 6u8)]
            ActiveMembersLimitOverflow,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance1();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance3();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance2();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance4();
    }
    pub mod pallet_committee {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance1();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance4();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PolymeshVotes<BlockNumber> {
            pub index: BlockNumber,
            pub ayes: Vec<polymesh_primitives::identity_id::IdentityId>,
            pub nays: Vec<polymesh_primitives::identity_id::IdentityId>,
            pub expiry: polymesh_common_utilities::MaybeBlock<BlockNumber>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<Hash, BlockNumber, I> {
            #[codec(index = 0u8)]
            Proposed(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                Hash,
            ),
            #[codec(index = 1u8)]
            Voted(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                Hash,
                bool,
                BlockNumber,
                BlockNumber,
                BlockNumber,
            ),
            #[codec(index = 2u8)]
            VoteRetracted(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                Hash,
                bool,
            ),
            #[codec(index = 3u8)]
            FinalVotes(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                Hash,
                Vec<polymesh_primitives::identity_id::IdentityId>,
                Vec<polymesh_primitives::identity_id::IdentityId>,
            ),
            #[codec(index = 4u8)]
            Approved(
                polymesh_primitives::identity_id::IdentityId,
                Hash,
                BlockNumber,
                BlockNumber,
                BlockNumber,
            ),
            #[codec(index = 5u8)]
            Rejected(
                polymesh_primitives::identity_id::IdentityId,
                Hash,
                BlockNumber,
                BlockNumber,
                BlockNumber,
            ),
            #[codec(index = 6u8)]
            Executed(
                polymesh_primitives::identity_id::IdentityId,
                Hash,
                Result<(), sp_runtime::DispatchError>,
            ),
            #[codec(index = 7u8)]
            ReleaseCoordinatorUpdated(
                polymesh_primitives::identity_id::IdentityId,
                Option<polymesh_primitives::identity_id::IdentityId>,
            ),
            #[codec(index = 8u8)]
            ExpiresAfterUpdated(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_common_utilities::MaybeBlock<BlockNumber>,
            ),
            #[codec(index = 9u8)]
            VoteThresholdUpdated(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                BlockNumber,
            ),
            PhantomDataVariant(core::marker::PhantomData<I>),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instance3();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            DuplicateVote,
            #[codec(index = 1u8)]
            NotAMember,
            #[codec(index = 2u8)]
            NoSuchProposal,
            #[codec(index = 3u8)]
            ProposalExpired,
            #[codec(index = 4u8)]
            DuplicateProposal,
            #[codec(index = 5u8)]
            MismatchedVotingIndex,
            #[codec(index = 6u8)]
            InvalidProportion,
            #[codec(index = 7u8)]
            FirstVoteReject,
            #[codec(index = 8u8)]
            ProposalsLimitReached,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            set_vote_threshold { n: u32, d: u32 },
            #[codec(index = 1u8)]
            set_release_coordinator {
                id: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 2u8)]
            set_expires_after {
                expiry: polymesh_common_utilities::MaybeBlock<u32>,
            },
            #[codec(index = 3u8)]
            vote_or_propose {
                approve: bool,
                call: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
            },
            #[codec(index = 4u8)]
            vote {
                proposal: primitive_types::H256,
                index: u32,
                approve: bool,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawOrigin<AccountId, I> {
            #[codec(index = 0u8)]
            Endorsed,
            PhantomDataVariant(core::marker::PhantomData<(AccountId, I)>),
        }
    }
    pub mod polymesh_common_utilities {
        use super::*;
        pub mod traits {
            use super::*;
            pub mod statistics {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum Event {
                    #[codec(index = 0u8)]
                    StatTypesAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::statistics::AssetScope,
                        Vec<polymesh_primitives::statistics::StatType>,
                    ),
                    #[codec(index = 1u8)]
                    StatTypesRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::statistics::AssetScope,
                        Vec<polymesh_primitives::statistics::StatType>,
                    ),
                    #[codec(index = 2u8)]
                    AssetStatsUpdated(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::statistics::AssetScope,
                        polymesh_primitives::statistics::StatType,
                        Vec<polymesh_primitives::statistics::StatUpdate>,
                    ),
                    #[codec(index = 3u8)]
                    SetAssetTransferCompliance(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::statistics::AssetScope,
                        Vec<polymesh_primitives::transfer_compliance::TransferCondition>,
                    ),
                    #[codec(index = 4u8)]
                    TransferConditionExemptionsAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::transfer_compliance::TransferConditionExemptKey,
                        Vec<polymesh_primitives::identity_id::IdentityId>,
                    ),
                    #[codec(index = 5u8)]
                    TransferConditionExemptionsRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::transfer_compliance::TransferConditionExemptKey,
                        Vec<polymesh_primitives::identity_id::IdentityId>,
                    ),
                }
            }
            pub mod external_agents {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum Event {
                    #[codec(index = 0u8)]
                    GroupCreated(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::agent::AGId,
                        polymesh_primitives::subset::SubsetRestriction<
                            polymesh_primitives::secondary_key::PalletPermissions,
                        >,
                    ),
                    #[codec(index = 1u8)]
                    GroupPermissionsUpdated(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::agent::AGId,
                        polymesh_primitives::subset::SubsetRestriction<
                            polymesh_primitives::secondary_key::PalletPermissions,
                        >,
                    ),
                    #[codec(index = 2u8)]
                    AgentAdded(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::agent::AgentGroup,
                    ),
                    #[codec(index = 3u8)]
                    AgentRemoved(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 4u8)]
                    GroupChanged(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::agent::AgentGroup,
                    ),
                }
            }
            pub mod relayer {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum RawEvent<AccountId> {
                    #[codec(index = 0u8)]
                    AuthorizedPayingKey(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        AccountId,
                        AccountId,
                        u128,
                        u64,
                    ),
                    #[codec(index = 1u8)]
                    AcceptedPayingKey(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        AccountId,
                        AccountId,
                    ),
                    #[codec(index = 2u8)]
                    RemovedPayingKey(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        AccountId,
                        AccountId,
                    ),
                    #[codec(index = 3u8)]
                    UpdatedPolyxLimit(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        AccountId,
                        AccountId,
                        u128,
                        u128,
                    ),
                }
            }
            pub mod portfolio {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum Event {
                    #[codec(index = 0u8)]
                    PortfolioCreated(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::PortfolioNumber,
                        polymesh_primitives::identity_id::PortfolioName,
                    ),
                    #[codec(index = 1u8)]
                    PortfolioDeleted(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::PortfolioNumber,
                    ),
                    #[codec(index = 2u8)]
                    MovedBetweenPortfolios(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::PortfolioId,
                        polymesh_primitives::identity_id::PortfolioId,
                        polymesh_primitives::ticker::Ticker,
                        u128,
                        Option<polymesh_common_utilities::traits::balances::Memo>,
                    ),
                    #[codec(index = 3u8)]
                    PortfolioRenamed(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::PortfolioNumber,
                        polymesh_primitives::identity_id::PortfolioName,
                    ),
                    #[codec(index = 4u8)]
                    UserPortfolios(
                        polymesh_primitives::identity_id::IdentityId,
                        Vec<(
                            polymesh_primitives::identity_id::PortfolioNumber,
                            polymesh_primitives::identity_id::PortfolioName,
                        )>,
                    ),
                    #[codec(index = 5u8)]
                    PortfolioCustodianChanged(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::PortfolioId,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                }
            }
            pub mod base {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum Event {
                    #[codec(index = 0u8)]
                    UnexpectedError(Option<sp_runtime::DispatchError>),
                }
            }
            pub mod group {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum RawEvent<AccountId, Event, I> {
                    #[codec(index = 0u8)]
                    MemberAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 1u8)]
                    MemberRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 2u8)]
                    MemberRevoked(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 3u8)]
                    MembersSwapped(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 4u8)]
                    MembersReset(
                        polymesh_primitives::identity_id::IdentityId,
                        Vec<polymesh_primitives::identity_id::IdentityId>,
                    ),
                    #[codec(index = 5u8)]
                    ActiveLimitChanged(polymesh_primitives::identity_id::IdentityId, u32, u32),
                    #[codec(index = 6u8)]
                    Dummy,
                    PhantomDataVariant(core::marker::PhantomData<(AccountId, Event, I)>),
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct InactiveMember<Moment> {
                    pub id: polymesh_primitives::identity_id::IdentityId,
                    pub deactivated_at: Moment,
                    pub expiry: Option<Moment>,
                }
            }
            pub mod asset {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum RawEvent<Moment, AccountId> {
                    #[codec(index = 0u8)]
                    Transfer(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::PortfolioId,
                        polymesh_primitives::identity_id::PortfolioId,
                        u128,
                    ),
                    #[codec(index = 1u8)]
                    Issued(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                        u128,
                        polymesh_primitives::asset::FundingRoundName,
                        u128,
                    ),
                    #[codec(index = 2u8)]
                    Redeemed(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                        u128,
                    ),
                    #[codec(index = 3u8)]
                    AssetCreated(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        bool,
                        polymesh_primitives::asset::AssetType,
                        polymesh_primitives::identity_id::IdentityId,
                        bool,
                    ),
                    #[codec(index = 4u8)]
                    IdentifiersUpdated(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
                    ),
                    #[codec(index = 5u8)]
                    DivisibilityChanged(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        bool,
                    ),
                    #[codec(index = 6u8)]
                    TransferWithData(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_id::IdentityId,
                        u128,
                        Vec<u8>,
                    ),
                    #[codec(index = 7u8)]
                    IsIssuable(polymesh_primitives::ticker::Ticker, bool),
                    #[codec(index = 8u8)]
                    TickerRegistered(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        Option<Moment>,
                    ),
                    #[codec(index = 9u8)]
                    TickerTransferred(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 10u8)]
                    AssetOwnershipTransferred(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::IdentityId,
                    ),
                    #[codec(index = 11u8)]
                    AssetFrozen(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                    ),
                    #[codec(index = 12u8)]
                    AssetUnfrozen(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                    ),
                    #[codec(index = 13u8)]
                    AssetRenamed(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::asset::AssetName,
                    ),
                    #[codec(index = 14u8)]
                    FundingRoundSet(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::asset::FundingRoundName,
                    ),
                    #[codec(index = 15u8)]
                    DocumentAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::document::DocumentId,
                        polymesh_primitives::document::Document,
                    ),
                    #[codec(index = 16u8)]
                    DocumentRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::document::DocumentId,
                    ),
                    #[codec(index = 17u8)]
                    ExtensionRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        AccountId,
                    ),
                    #[codec(index = 18u8)]
                    ClassicTickerClaimed(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::ethereum::EthereumAddress,
                    ),
                    #[codec(index = 19u8)]
                    ControllerTransfer(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::identity_id::PortfolioId,
                        u128,
                    ),
                    #[codec(index = 20u8)]
                    CustomAssetTypeExists(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::asset::CustomAssetTypeId,
                        Vec<u8>,
                    ),
                    #[codec(index = 21u8)]
                    CustomAssetTypeRegistered(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::asset::CustomAssetTypeId,
                        Vec<u8>,
                    ),
                    #[codec(index = 22u8)]
                    SetAssetMetadataValue(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::asset_metadata::AssetMetadataValue,
                        Option<
                            polymesh_primitives::asset_metadata::AssetMetadataValueDetail<Moment>,
                        >,
                    ),
                    #[codec(index = 23u8)]
                    SetAssetMetadataValueDetails(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::asset_metadata::AssetMetadataValueDetail<Moment>,
                    ),
                    #[codec(index = 24u8)]
                    RegisterAssetMetadataLocalType(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::asset_metadata::AssetMetadataName,
                        polymesh_primitives::asset_metadata::AssetMetadataLocalKey,
                        polymesh_primitives::asset_metadata::AssetMetadataSpec,
                    ),
                    #[codec(index = 25u8)]
                    RegisterAssetMetadataGlobalType(
                        polymesh_primitives::asset_metadata::AssetMetadataName,
                        polymesh_primitives::asset_metadata::AssetMetadataGlobalKey,
                        polymesh_primitives::asset_metadata::AssetMetadataSpec,
                    ),
                }
            }
            pub mod checkpoint {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct StoredSchedule {
                    pub schedule: polymesh_primitives::calendar::CheckpointSchedule,
                    pub id: polymesh_common_utilities::traits::checkpoint::ScheduleId,
                    pub at: u64,
                    pub remaining: u32,
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum Event {
                    #[codec(index = 0u8)]
                    CheckpointCreated(
                        Option<
                            polymesh_primitives::event_only::EventOnly<
                                polymesh_primitives::identity_id::IdentityId,
                            >,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_primitives::calendar::CheckpointId,
                        u128,
                        u64,
                    ),
                    #[codec(index = 1u8)]
                    MaximumSchedulesComplexityChanged(
                        polymesh_primitives::identity_id::IdentityId,
                        u64,
                    ),
                    #[codec(index = 2u8)]
                    ScheduleCreated(
                        polymesh_primitives::event_only::EventOnly<
                            polymesh_primitives::identity_id::IdentityId,
                        >,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_common_utilities::traits::checkpoint::StoredSchedule,
                    ),
                    #[codec(index = 3u8)]
                    ScheduleRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                        polymesh_common_utilities::traits::checkpoint::StoredSchedule,
                    ),
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct ScheduleId(pub u64);
            }
            pub mod balances {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum RawEvent<AccountId> {
                    #[codec(index = 0u8)]
                    Endowed(
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        AccountId,
                        u128,
                    ),
                    #[codec(index = 1u8)]
                    Transfer(
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        AccountId,
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        AccountId,
                        u128,
                        Option<polymesh_common_utilities::traits::balances::Memo>,
                    ),
                    #[codec(index = 2u8)]
                    BalanceSet(
                        polymesh_primitives::identity_id::IdentityId,
                        AccountId,
                        u128,
                        u128,
                    ),
                    #[codec(index = 3u8)]
                    AccountBalanceBurned(
                        polymesh_primitives::identity_id::IdentityId,
                        AccountId,
                        u128,
                    ),
                    #[codec(index = 4u8)]
                    Reserved(AccountId, u128),
                    #[codec(index = 5u8)]
                    Unreserved(AccountId, u128),
                    #[codec(index = 6u8)]
                    ReserveRepatriated(
                        AccountId,
                        AccountId,
                        u128,
                        frame_support::traits::BalanceStatus,
                    ),
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct Memo(pub [u8; 32usize]);
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum Reasons {
                    #[codec(index = 0u8)]
                    Fee,
                    #[codec(index = 1u8)]
                    Misc,
                    #[codec(index = 2u8)]
                    All,
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct AccountData {
                    pub free: u128,
                    pub reserved: u128,
                    pub misc_frozen: u128,
                    pub fee_frozen: u128,
                }
            }
            pub mod identity {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub enum RawEvent<AccountId, Moment> {
                    #[codec(index = 0u8)]
                    DidCreated(
                        polymesh_primitives::identity_id::IdentityId,
                        AccountId,
                        Vec<polymesh_primitives::secondary_key::SecondaryKey<AccountId>>,
                    ),
                    #[codec(index = 1u8)]
                    SecondaryKeysAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        Vec<polymesh_primitives::secondary_key::SecondaryKey<AccountId>>,
                    ),
                    #[codec(index = 2u8)]
                    SecondaryKeysRemoved(
                        polymesh_primitives::identity_id::IdentityId,
                        Vec<AccountId>,
                    ),
                    #[codec(index = 3u8)]
                    SecondaryKeyLeftIdentity(
                        polymesh_primitives::identity_id::IdentityId,
                        AccountId,
                    ),
                    #[codec(index = 4u8)]
                    SecondaryKeyPermissionsUpdated(
                        polymesh_primitives::identity_id::IdentityId,
                        AccountId,
                        polymesh_primitives::secondary_key::Permissions,
                        polymesh_primitives::secondary_key::Permissions,
                    ),
                    #[codec(index = 5u8)]
                    PrimaryKeyUpdated(
                        polymesh_primitives::identity_id::IdentityId,
                        AccountId,
                        AccountId,
                    ),
                    #[codec(index = 6u8)]
                    ClaimAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_claim::IdentityClaim,
                    ),
                    #[codec(index = 7u8)]
                    ClaimRevoked(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::identity_claim::IdentityClaim,
                    ),
                    #[codec(index = 8u8)]
                    AssetDidRegistered(
                        polymesh_primitives::identity_id::IdentityId,
                        polymesh_primitives::ticker::Ticker,
                    ),
                    #[codec(index = 9u8)]
                    AuthorizationAdded(
                        polymesh_primitives::identity_id::IdentityId,
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        Option<AccountId>,
                        Moment,
                        polymesh_primitives::authorization::AuthorizationData<AccountId>,
                        Option<Moment>,
                    ),
                    #[codec(index = 10u8)]
                    AuthorizationRevoked(
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        Option<AccountId>,
                        Moment,
                    ),
                    #[codec(index = 11u8)]
                    AuthorizationRejected(
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        Option<AccountId>,
                        Moment,
                    ),
                    #[codec(index = 12u8)]
                    AuthorizationConsumed(
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        Option<AccountId>,
                        Moment,
                    ),
                    #[codec(index = 13u8)]
                    AuthorizationRetryLimitReached(
                        Option<polymesh_primitives::identity_id::IdentityId>,
                        Option<AccountId>,
                        Moment,
                    ),
                    #[codec(index = 14u8)]
                    CddRequirementForPrimaryKeyUpdated(bool),
                    #[codec(index = 15u8)]
                    CddClaimsInvalidated(polymesh_primitives::identity_id::IdentityId, Moment),
                    #[codec(index = 16u8)]
                    SecondaryKeysFrozen(polymesh_primitives::identity_id::IdentityId),
                    #[codec(index = 17u8)]
                    SecondaryKeysUnfrozen(polymesh_primitives::identity_id::IdentityId),
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct SecondaryKeyWithAuthV1<AccountId> {
                    pub secondary_key:
                        polymesh_primitives::secondary_key::v1::SecondaryKey<AccountId>,
                    pub auth_signature: primitive_types::H512,
                }
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct SecondaryKeyWithAuth<AccountId> {
                    pub secondary_key: polymesh_primitives::secondary_key::SecondaryKey<AccountId>,
                    pub auth_signature: primitive_types::H512,
                }
            }
        }
        pub mod protocol_fee {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum ProtocolOp {
                #[codec(index = 0u8)]
                AssetRegisterTicker,
                #[codec(index = 1u8)]
                AssetIssue,
                #[codec(index = 2u8)]
                AssetAddDocuments,
                #[codec(index = 3u8)]
                AssetCreateAsset,
                #[codec(index = 4u8)]
                CheckpointCreateSchedule,
                #[codec(index = 5u8)]
                ComplianceManagerAddComplianceRequirement,
                #[codec(index = 6u8)]
                IdentityCddRegisterDid,
                #[codec(index = 7u8)]
                IdentityAddClaim,
                #[codec(index = 8u8)]
                IdentityAddSecondaryKeysWithAuthorization,
                #[codec(index = 9u8)]
                PipsPropose,
                #[codec(index = 10u8)]
                ContractsPutCode,
                #[codec(index = 11u8)]
                CorporateBallotAttachBallot,
                #[codec(index = 12u8)]
                CapitalDistributionDistribute,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum MaybeBlock<BlockNumber> {
            #[codec(index = 0u8)]
            Some(BlockNumber),
            #[codec(index = 1u8)]
            None,
        }
    }
    pub mod pallet_treasury {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            disbursement {
                beneficiaries: Vec<polymesh_primitives::Beneficiary<u128>>,
            },
            #[codec(index = 1u8)]
            reimbursement { amount: u128 },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            InsufficientBalance,
            #[codec(index = 1u8)]
            InvalidIdentity,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<Balance, AccountId> {
            #[codec(index = 0u8)]
            TreasuryDisbursement(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Balance,
            ),
            #[codec(index = 1u8)]
            TreasuryDisbursementFailed(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Balance,
            ),
            #[codec(index = 2u8)]
            TreasuryReimbursement(polymesh_primitives::identity_id::IdentityId, Balance),
        }
    }
    pub mod sp_authority_discovery {
        use super::*;
        pub mod app {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Public(pub sp_core::sr25519::Public);
        }
    }
    pub mod pallet_asset {
        use super::*;
        pub mod checkpoint {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                NoSuchSchedule,
                #[codec(index = 1u8)]
                ScheduleNotRemovable,
                #[codec(index = 2u8)]
                FailedToComputeNextCheckpoint,
                #[codec(index = 3u8)]
                ScheduleDurationTooShort,
                #[codec(index = 4u8)]
                SchedulesTooComplex,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct ScheduleSpec {
                pub start: Option<u64>,
                pub period: polymesh_primitives::calendar::CalendarPeriod,
                pub remaining: u32,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                create_checkpoint {
                    ticker: polymesh_primitives::ticker::Ticker,
                },
                #[codec(index = 1u8)]
                set_schedules_max_complexity { max_complexity: u64 },
                #[codec(index = 2u8)]
                create_schedule {
                    ticker: polymesh_primitives::ticker::Ticker,
                    schedule: pallet_asset::checkpoint::ScheduleSpec,
                },
                #[codec(index = 3u8)]
                remove_schedule {
                    ticker: polymesh_primitives::ticker::Ticker,
                    id: polymesh_common_utilities::traits::checkpoint::ScheduleId,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Version(pub u8);
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct TickerRegistrationConfig<U> {
            pub max_ticker_length: u8,
            pub registration_length: Option<U>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct TickerRegistration<U> {
            pub owner: polymesh_primitives::identity_id::IdentityId,
            pub expiry: Option<U>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            Unauthorized,
            #[codec(index = 1u8)]
            AssetAlreadyCreated,
            #[codec(index = 2u8)]
            TickerTooLong,
            #[codec(index = 3u8)]
            TickerNotAscii,
            #[codec(index = 4u8)]
            TickerAlreadyRegistered,
            #[codec(index = 5u8)]
            TotalSupplyAboveLimit,
            #[codec(index = 6u8)]
            NoSuchAsset,
            #[codec(index = 7u8)]
            AlreadyFrozen,
            #[codec(index = 8u8)]
            NotAnOwner,
            #[codec(index = 9u8)]
            BalanceOverflow,
            #[codec(index = 10u8)]
            TotalSupplyOverflow,
            #[codec(index = 11u8)]
            InvalidGranularity,
            #[codec(index = 12u8)]
            NotFrozen,
            #[codec(index = 13u8)]
            InvalidTransfer,
            #[codec(index = 14u8)]
            InsufficientBalance,
            #[codec(index = 15u8)]
            AssetAlreadyDivisible,
            #[codec(index = 16u8)]
            InvalidEthereumSignature,
            #[codec(index = 17u8)]
            NoSuchClassicTicker,
            #[codec(index = 18u8)]
            TickerRegistrationExpired,
            #[codec(index = 19u8)]
            SenderSameAsReceiver,
            #[codec(index = 20u8)]
            NoSuchDoc,
            #[codec(index = 21u8)]
            MaxLengthOfAssetNameExceeded,
            #[codec(index = 22u8)]
            FundingRoundNameMaxLengthExceeded,
            #[codec(index = 23u8)]
            InvalidAssetIdentifier,
            #[codec(index = 24u8)]
            InvestorUniquenessClaimNotAllowed,
            #[codec(index = 25u8)]
            InvalidCustomAssetTypeId,
            #[codec(index = 26u8)]
            AssetMetadataNameMaxLengthExceeded,
            #[codec(index = 27u8)]
            AssetMetadataValueMaxLengthExceeded,
            #[codec(index = 28u8)]
            AssetMetadataTypeDefMaxLengthExceeded,
            #[codec(index = 29u8)]
            AssetMetadataKeyIsMissing,
            #[codec(index = 30u8)]
            AssetMetadataValueIsLocked,
            #[codec(index = 31u8)]
            AssetMetadataLocalKeyAlreadyExists,
            #[codec(index = 32u8)]
            AssetMetadataGlobalKeyAlreadyExists,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum AssetOwnershipRelation {
            #[codec(index = 0u8)]
            NotOwned,
            #[codec(index = 1u8)]
            TickerOwned,
            #[codec(index = 2u8)]
            AssetOwned,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ClassicTickerRegistration {
            pub eth_owner: polymesh_primitives::ethereum::EthereumAddress,
            pub is_created: bool,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ClassicTickerImport {
            pub eth_owner: polymesh_primitives::ethereum::EthereumAddress,
            pub ticker: polymesh_primitives::ticker::Ticker,
            pub is_contract: bool,
            pub is_created: bool,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct SecurityToken {
            pub total_supply: u128,
            pub owner_did: polymesh_primitives::identity_id::IdentityId,
            pub divisible: bool,
            pub asset_type: polymesh_primitives::asset::AssetType,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            register_ticker {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 1u8)]
            accept_ticker_transfer { auth_id: u64 },
            #[codec(index = 2u8)]
            accept_asset_ownership_transfer { auth_id: u64 },
            #[codec(index = 3u8)]
            create_asset {
                name: polymesh_primitives::asset::AssetName,
                ticker: polymesh_primitives::ticker::Ticker,
                divisible: bool,
                asset_type: polymesh_primitives::asset::AssetType,
                identifiers: Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
                funding_round: Option<polymesh_primitives::asset::FundingRoundName>,
                disable_iu: bool,
            },
            #[codec(index = 4u8)]
            freeze {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 5u8)]
            unfreeze {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 6u8)]
            rename_asset {
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset::AssetName,
            },
            #[codec(index = 7u8)]
            issue {
                ticker: polymesh_primitives::ticker::Ticker,
                amount: u128,
            },
            #[codec(index = 8u8)]
            redeem {
                ticker: polymesh_primitives::ticker::Ticker,
                value: u128,
            },
            #[codec(index = 9u8)]
            make_divisible {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 10u8)]
            add_documents {
                docs: Vec<polymesh_primitives::document::Document>,
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 11u8)]
            remove_documents {
                ids: Vec<polymesh_primitives::document::DocumentId>,
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 12u8)]
            set_funding_round {
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset::FundingRoundName,
            },
            #[codec(index = 13u8)]
            update_identifiers {
                ticker: polymesh_primitives::ticker::Ticker,
                identifiers: Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
            },
            #[codec(index = 14u8)]
            claim_classic_ticker {
                ticker: polymesh_primitives::ticker::Ticker,
                ethereum_signature: polymesh_primitives::ethereum::EcdsaSignature,
            },
            #[codec(index = 15u8)]
            reserve_classic_ticker {
                classic_ticker_import: pallet_asset::ClassicTickerImport,
                contract_did: polymesh_primitives::identity_id::IdentityId,
                config: pallet_asset::TickerRegistrationConfig<u64>,
            },
            #[codec(index = 16u8)]
            controller_transfer {
                ticker: polymesh_primitives::ticker::Ticker,
                value: u128,
                from_portfolio: polymesh_primitives::identity_id::PortfolioId,
            },
            #[codec(index = 17u8)]
            register_custom_asset_type { ty: Vec<u8> },
            #[codec(index = 18u8)]
            create_asset_with_custom_type {
                name: polymesh_primitives::asset::AssetName,
                ticker: polymesh_primitives::ticker::Ticker,
                divisible: bool,
                custom_asset_type: Vec<u8>,
                identifiers: Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
                funding_round: Option<polymesh_primitives::asset::FundingRoundName>,
                disable_iu: bool,
            },
            #[codec(index = 19u8)]
            set_asset_metadata {
                ticker: polymesh_primitives::ticker::Ticker,
                key: polymesh_primitives::asset_metadata::AssetMetadataKey,
                value: polymesh_primitives::asset_metadata::AssetMetadataValue,
                detail: Option<polymesh_primitives::asset_metadata::AssetMetadataValueDetail<u64>>,
            },
            #[codec(index = 20u8)]
            set_asset_metadata_details {
                ticker: polymesh_primitives::ticker::Ticker,
                key: polymesh_primitives::asset_metadata::AssetMetadataKey,
                detail: polymesh_primitives::asset_metadata::AssetMetadataValueDetail<u64>,
            },
            #[codec(index = 21u8)]
            register_and_set_local_asset_metadata {
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset_metadata::AssetMetadataName,
                spec: polymesh_primitives::asset_metadata::AssetMetadataSpec,
                value: polymesh_primitives::asset_metadata::AssetMetadataValue,
                detail: Option<polymesh_primitives::asset_metadata::AssetMetadataValueDetail<u64>>,
            },
            #[codec(index = 22u8)]
            register_asset_metadata_local_type {
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset_metadata::AssetMetadataName,
                spec: polymesh_primitives::asset_metadata::AssetMetadataSpec,
            },
            #[codec(index = 23u8)]
            register_asset_metadata_global_type {
                name: polymesh_primitives::asset_metadata::AssetMetadataName,
                spec: polymesh_primitives::asset_metadata::AssetMetadataSpec,
            },
        }
    }
    pub mod pallet_portfolio {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            PortfolioDoesNotExist,
            #[codec(index = 1u8)]
            InsufficientPortfolioBalance,
            #[codec(index = 2u8)]
            DestinationIsSamePortfolio,
            #[codec(index = 3u8)]
            PortfolioNameAlreadyInUse,
            #[codec(index = 4u8)]
            SecondaryKeyNotAuthorizedForPortfolio,
            #[codec(index = 5u8)]
            UnauthorizedCustodian,
            #[codec(index = 6u8)]
            InsufficientTokensLocked,
            #[codec(index = 7u8)]
            PortfolioNotEmpty,
            #[codec(index = 8u8)]
            DifferentIdentityPortfolios,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            create_portfolio {
                name: polymesh_primitives::identity_id::PortfolioName,
            },
            #[codec(index = 1u8)]
            delete_portfolio {
                num: polymesh_primitives::identity_id::PortfolioNumber,
            },
            #[codec(index = 2u8)]
            move_portfolio_funds {
                from: polymesh_primitives::identity_id::PortfolioId,
                to: polymesh_primitives::identity_id::PortfolioId,
                items: Vec<pallet_portfolio::MovePortfolioItem>,
            },
            #[codec(index = 3u8)]
            rename_portfolio {
                num: polymesh_primitives::identity_id::PortfolioNumber,
                to_name: polymesh_primitives::identity_id::PortfolioName,
            },
            #[codec(index = 4u8)]
            quit_portfolio_custody {
                pid: polymesh_primitives::identity_id::PortfolioId,
            },
            #[codec(index = 5u8)]
            accept_portfolio_custody { auth_id: u64 },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct MovePortfolioItem {
            pub ticker: polymesh_primitives::ticker::Ticker,
            pub amount: u128,
            pub memo: Option<polymesh_common_utilities::traits::balances::Memo>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
    }
    pub mod sp_npos_elections {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ElectionScore {
            pub minimal_stake: u128,
            pub sum_stake: u128,
            pub sum_stake_squared: u128,
        }
    }
    pub mod finality_grandpa {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Precommit<H, N> {
            pub target_hash: H,
            pub target_number: N,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Equivocation<Id, V, S> {
            pub round_number: u64,
            pub identity: Id,
            pub first: (V, S),
            pub second: (V, S),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Prevote<H, N> {
            pub target_hash: H,
            pub target_number: N,
        }
    }
    pub mod pallet_base {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            TooLong,
            #[codec(index = 1u8)]
            CounterOverflow,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {}
    }
    pub mod pallet_multisig {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            CddMissing,
            #[codec(index = 1u8)]
            ProposalMissing,
            #[codec(index = 2u8)]
            DecodingError,
            #[codec(index = 3u8)]
            NoSigners,
            #[codec(index = 4u8)]
            RequiredSignaturesOutOfBounds,
            #[codec(index = 5u8)]
            NotASigner,
            #[codec(index = 6u8)]
            NoSuchMultisig,
            #[codec(index = 7u8)]
            NotEnoughSigners,
            #[codec(index = 8u8)]
            NonceOverflow,
            #[codec(index = 9u8)]
            AlreadyVoted,
            #[codec(index = 10u8)]
            AlreadyASigner,
            #[codec(index = 11u8)]
            FailedToChargeFee,
            #[codec(index = 12u8)]
            IdentityNotCreator,
            #[codec(index = 13u8)]
            ChangeNotAllowed,
            #[codec(index = 14u8)]
            SignerAlreadyLinkedToMultisig,
            #[codec(index = 15u8)]
            SignerAlreadyLinkedToIdentity,
            #[codec(index = 16u8)]
            MultisigNotAllowedToLinkToItself,
            #[codec(index = 17u8)]
            MissingCurrentIdentity,
            #[codec(index = 18u8)]
            NotPrimaryKey,
            #[codec(index = 19u8)]
            ProposalAlreadyRejected,
            #[codec(index = 20u8)]
            ProposalExpired,
            #[codec(index = 21u8)]
            ProposalAlreadyExecuted,
            #[codec(index = 22u8)]
            MultisigMissingIdentity,
            #[codec(index = 23u8)]
            FailedToSchedule,
            #[codec(index = 24u8)]
            TooManySigners,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId> {
            #[codec(index = 0u8)]
            MultiSigCreated(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                AccountId,
                Vec<polymesh_primitives::secondary_key::Signatory<AccountId>>,
                u64,
            ),
            #[codec(index = 1u8)]
            ProposalAdded(polymesh_primitives::identity_id::IdentityId, AccountId, u64),
            #[codec(index = 2u8)]
            ProposalExecuted(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                u64,
                bool,
            ),
            #[codec(index = 3u8)]
            MultiSigSignerAdded(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                polymesh_primitives::secondary_key::Signatory<AccountId>,
            ),
            #[codec(index = 4u8)]
            MultiSigSignerAuthorized(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                polymesh_primitives::secondary_key::Signatory<AccountId>,
            ),
            #[codec(index = 5u8)]
            MultiSigSignerRemoved(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                polymesh_primitives::secondary_key::Signatory<AccountId>,
            ),
            #[codec(index = 6u8)]
            MultiSigSignaturesRequiredChanged(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                u64,
            ),
            #[codec(index = 7u8)]
            ProposalApproved(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                polymesh_primitives::secondary_key::Signatory<AccountId>,
                u64,
            ),
            #[codec(index = 8u8)]
            ProposalRejectionVote(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                polymesh_primitives::secondary_key::Signatory<AccountId>,
                u64,
            ),
            #[codec(index = 9u8)]
            ProposalRejected(polymesh_primitives::identity_id::IdentityId, AccountId, u64),
            #[codec(index = 10u8)]
            ProposalExecutionFailed(sp_runtime::DispatchError),
            #[codec(index = 11u8)]
            SchedulingFailed(sp_runtime::DispatchError),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            create_multisig {
                signers: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
                sigs_required: u64,
            },
            #[codec(index = 1u8)]
            create_or_approve_proposal_as_identity {
                multisig: sp_core::crypto::AccountId32,
                proposal: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
                expiry: Option<u64>,
                auto_close: bool,
            },
            #[codec(index = 2u8)]
            create_or_approve_proposal_as_key {
                multisig: sp_core::crypto::AccountId32,
                proposal: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
                expiry: Option<u64>,
                auto_close: bool,
            },
            #[codec(index = 3u8)]
            create_proposal_as_identity {
                multisig: sp_core::crypto::AccountId32,
                proposal: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
                expiry: Option<u64>,
                auto_close: bool,
            },
            #[codec(index = 4u8)]
            create_proposal_as_key {
                multisig: sp_core::crypto::AccountId32,
                proposal: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
                expiry: Option<u64>,
                auto_close: bool,
            },
            #[codec(index = 5u8)]
            approve_as_identity {
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            },
            #[codec(index = 6u8)]
            approve_as_key {
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            },
            #[codec(index = 7u8)]
            reject_as_identity {
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            },
            #[codec(index = 8u8)]
            reject_as_key {
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            },
            #[codec(index = 9u8)]
            accept_multisig_signer_as_identity { auth_id: u64 },
            #[codec(index = 10u8)]
            accept_multisig_signer_as_key { auth_id: u64 },
            #[codec(index = 11u8)]
            add_multisig_signer {
                signer: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 12u8)]
            remove_multisig_signer {
                signer: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 13u8)]
            add_multisig_signers_via_creator {
                multisig: sp_core::crypto::AccountId32,
                signers: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
            },
            #[codec(index = 14u8)]
            remove_multisig_signers_via_creator {
                multisig: sp_core::crypto::AccountId32,
                signers: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
            },
            #[codec(index = 15u8)]
            change_sigs_required { sigs_required: u64 },
            #[codec(index = 16u8)]
            make_multisig_secondary {
                multisig: sp_core::crypto::AccountId32,
            },
            #[codec(index = 17u8)]
            make_multisig_primary {
                multisig: sp_core::crypto::AccountId32,
                optional_cdd_auth_id: Option<u64>,
            },
            #[codec(index = 18u8)]
            execute_scheduled_proposal {
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
                multisig_did: polymesh_primitives::identity_id::IdentityId,
                _proposal_weight: u64,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ProposalDetails<T> {
            pub approvals: T,
            pub rejections: T,
            pub status: pallet_multisig::ProposalStatus,
            pub expiry: Option<T>,
            pub auto_close: bool,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum ProposalStatus {
            #[codec(index = 0u8)]
            Invalid,
            #[codec(index = 1u8)]
            ActiveOrExpired,
            #[codec(index = 2u8)]
            ExecutionSuccessful,
            #[codec(index = 3u8)]
            ExecutionFailed,
            #[codec(index = 4u8)]
            Rejected,
        }
    }
    pub mod sp_staking {
        use super::*;
        pub mod offence {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct OffenceDetails<Reporter, Offender> {
                pub offender: Offender,
                pub reporters: Vec<Reporter>,
            }
        }
    }
    pub mod pallet_grandpa {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                report_equivocation {
                    equivocation_proof: ::std::boxed::Box<
                        sp_finality_grandpa::EquivocationProof<primitive_types::H256, u32>,
                    >,
                    key_owner_proof: sp_session::MembershipProof,
                },
                #[codec(index = 1u8)]
                report_equivocation_unsigned {
                    equivocation_proof: ::std::boxed::Box<
                        sp_finality_grandpa::EquivocationProof<primitive_types::H256, u32>,
                    >,
                    key_owner_proof: sp_session::MembershipProof,
                },
                #[codec(index = 2u8)]
                note_stalled {
                    delay: u32,
                    best_finalized_block_number: u32,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                NewAuthorities {
                    authority_set: Vec<(sp_finality_grandpa::app::Public, u64)>,
                },
                #[codec(index = 1u8)]
                Paused,
                #[codec(index = 2u8)]
                Resumed,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                PauseFailed,
                #[codec(index = 1u8)]
                ResumeFailed,
                #[codec(index = 2u8)]
                ChangePending,
                #[codec(index = 3u8)]
                TooSoon,
                #[codec(index = 4u8)]
                InvalidKeyOwnershipProof,
                #[codec(index = 5u8)]
                InvalidEquivocationProof,
                #[codec(index = 6u8)]
                DuplicateOffenceReport,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum StoredState<N> {
            #[codec(index = 0u8)]
            Live,
            #[codec(index = 1u8)]
            PendingPause { scheduled_at: N, delay: N },
            #[codec(index = 2u8)]
            Paused,
            #[codec(index = 3u8)]
            PendingResume { scheduled_at: N, delay: N },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct StoredPendingChange<N> {
            pub scheduled_at: N,
            pub delay: N,
            pub next_authorities: Vec<(sp_finality_grandpa::app::Public, u64)>,
            pub forced: Option<N>,
        }
    }
    pub mod pallet_settlement {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Instruction<Moment, BlockNumber> {
            pub instruction_id: pallet_settlement::InstructionId,
            pub venue_id: pallet_settlement::VenueId,
            pub status: pallet_settlement::InstructionStatus,
            pub settlement_type: pallet_settlement::SettlementType<BlockNumber>,
            pub created_at: Option<Moment>,
            pub trade_date: Option<Moment>,
            pub value_date: Option<Moment>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct LegId(pub u64);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            create_venue {
                details: pallet_settlement::VenueDetails,
                signers: Vec<sp_core::crypto::AccountId32>,
                typ: pallet_settlement::VenueType,
            },
            #[codec(index = 1u8)]
            update_venue_details {
                id: pallet_settlement::VenueId,
                details: pallet_settlement::VenueDetails,
            },
            #[codec(index = 2u8)]
            update_venue_type {
                id: pallet_settlement::VenueId,
                typ: pallet_settlement::VenueType,
            },
            #[codec(index = 3u8)]
            add_instruction {
                venue_id: pallet_settlement::VenueId,
                settlement_type: pallet_settlement::SettlementType<u32>,
                trade_date: Option<u64>,
                value_date: Option<u64>,
                legs: Vec<pallet_settlement::Leg>,
            },
            #[codec(index = 4u8)]
            add_and_affirm_instruction {
                venue_id: pallet_settlement::VenueId,
                settlement_type: pallet_settlement::SettlementType<u32>,
                trade_date: Option<u64>,
                value_date: Option<u64>,
                legs: Vec<pallet_settlement::Leg>,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
            },
            #[codec(index = 5u8)]
            affirm_instruction {
                id: pallet_settlement::InstructionId,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
                max_legs_count: u32,
            },
            #[codec(index = 6u8)]
            withdraw_affirmation {
                id: pallet_settlement::InstructionId,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
                max_legs_count: u32,
            },
            #[codec(index = 7u8)]
            reject_instruction {
                id: pallet_settlement::InstructionId,
                portfolio: polymesh_primitives::identity_id::PortfolioId,
                num_of_legs: u32,
            },
            #[codec(index = 8u8)]
            affirm_with_receipts {
                id: pallet_settlement::InstructionId,
                receipt_details: Vec<
                    pallet_settlement::ReceiptDetails<
                        sp_core::crypto::AccountId32,
                        sp_runtime::MultiSignature,
                    >,
                >,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
                max_legs_count: u32,
            },
            #[codec(index = 9u8)]
            claim_receipt {
                id: pallet_settlement::InstructionId,
                receipt_details: pallet_settlement::ReceiptDetails<
                    sp_core::crypto::AccountId32,
                    sp_runtime::MultiSignature,
                >,
            },
            #[codec(index = 10u8)]
            unclaim_receipt {
                instruction_id: pallet_settlement::InstructionId,
                leg_id: pallet_settlement::LegId,
            },
            #[codec(index = 11u8)]
            set_venue_filtering {
                ticker: polymesh_primitives::ticker::Ticker,
                enabled: bool,
            },
            #[codec(index = 12u8)]
            allow_venues {
                ticker: polymesh_primitives::ticker::Ticker,
                venues: Vec<pallet_settlement::VenueId>,
            },
            #[codec(index = 13u8)]
            disallow_venues {
                ticker: polymesh_primitives::ticker::Ticker,
                venues: Vec<pallet_settlement::VenueId>,
            },
            #[codec(index = 14u8)]
            change_receipt_validity { receipt_uid: u64, validity: bool },
            #[codec(index = 15u8)]
            execute_scheduled_instruction {
                id: pallet_settlement::InstructionId,
                _legs_count: u32,
            },
            #[codec(index = 16u8)]
            reschedule_instruction {
                id: pallet_settlement::InstructionId,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ReceiptDetails<AccountId, OffChainSignature> {
            pub receipt_uid: u64,
            pub leg_id: pallet_settlement::LegId,
            pub signer: AccountId,
            pub signature: OffChainSignature,
            pub metadata: pallet_settlement::ReceiptMetadata,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            InvalidVenue,
            #[codec(index = 1u8)]
            Unauthorized,
            #[codec(index = 2u8)]
            NoPendingAffirm,
            #[codec(index = 3u8)]
            InstructionNotAffirmed,
            #[codec(index = 4u8)]
            InstructionNotPending,
            #[codec(index = 5u8)]
            InstructionNotFailed,
            #[codec(index = 6u8)]
            LegNotPending,
            #[codec(index = 7u8)]
            UnauthorizedSigner,
            #[codec(index = 8u8)]
            ReceiptAlreadyClaimed,
            #[codec(index = 9u8)]
            ReceiptNotClaimed,
            #[codec(index = 10u8)]
            UnauthorizedVenue,
            #[codec(index = 11u8)]
            FailedToLockTokens,
            #[codec(index = 12u8)]
            InstructionFailed,
            #[codec(index = 13u8)]
            InstructionDatesInvalid,
            #[codec(index = 14u8)]
            InstructionSettleBlockPassed,
            #[codec(index = 15u8)]
            InvalidSignature,
            #[codec(index = 16u8)]
            SameSenderReceiver,
            #[codec(index = 17u8)]
            PortfolioMismatch,
            #[codec(index = 18u8)]
            SettleOnPastBlock,
            #[codec(index = 19u8)]
            NoPortfolioProvided,
            #[codec(index = 20u8)]
            UnexpectedAffirmationStatus,
            #[codec(index = 21u8)]
            FailedToSchedule,
            #[codec(index = 22u8)]
            LegCountTooSmall,
            #[codec(index = 23u8)]
            UnknownInstruction,
            #[codec(index = 24u8)]
            InstructionHasTooManyLegs,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum InstructionStatus {
            #[codec(index = 0u8)]
            Unknown,
            #[codec(index = 1u8)]
            Pending,
            #[codec(index = 2u8)]
            Failed,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum AffirmationStatus {
            #[codec(index = 0u8)]
            Unknown,
            #[codec(index = 1u8)]
            Pending,
            #[codec(index = 2u8)]
            Affirmed,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Leg {
            pub from: polymesh_primitives::identity_id::PortfolioId,
            pub to: polymesh_primitives::identity_id::PortfolioId,
            pub asset: polymesh_primitives::ticker::Ticker,
            pub amount: u128,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum VenueType {
            #[codec(index = 0u8)]
            Other,
            #[codec(index = 1u8)]
            Distribution,
            #[codec(index = 2u8)]
            Sto,
            #[codec(index = 3u8)]
            Exchange,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<Moment, BlockNumber, AccountId> {
            #[codec(index = 0u8)]
            VenueCreated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::VenueId,
                pallet_settlement::VenueDetails,
                pallet_settlement::VenueType,
            ),
            #[codec(index = 1u8)]
            VenueDetailsUpdated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::VenueId,
                pallet_settlement::VenueDetails,
            ),
            #[codec(index = 2u8)]
            VenueTypeUpdated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::VenueId,
                pallet_settlement::VenueType,
            ),
            #[codec(index = 3u8)]
            InstructionCreated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::VenueId,
                pallet_settlement::InstructionId,
                pallet_settlement::SettlementType<BlockNumber>,
                Option<Moment>,
                Option<Moment>,
                Vec<pallet_settlement::Leg>,
            ),
            #[codec(index = 4u8)]
            InstructionAffirmed(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::PortfolioId,
                pallet_settlement::InstructionId,
            ),
            #[codec(index = 5u8)]
            AffirmationWithdrawn(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::PortfolioId,
                pallet_settlement::InstructionId,
            ),
            #[codec(index = 6u8)]
            InstructionRejected(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
            ),
            #[codec(index = 7u8)]
            ReceiptClaimed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
                pallet_settlement::LegId,
                Moment,
                AccountId,
                pallet_settlement::ReceiptMetadata,
            ),
            #[codec(index = 8u8)]
            ReceiptValidityChanged(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Moment,
                bool,
            ),
            #[codec(index = 9u8)]
            ReceiptUnclaimed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
                pallet_settlement::LegId,
                Moment,
                AccountId,
            ),
            #[codec(index = 10u8)]
            VenueFiltering(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                bool,
            ),
            #[codec(index = 11u8)]
            VenuesAllowed(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                Vec<pallet_settlement::VenueId>,
            ),
            #[codec(index = 12u8)]
            VenuesBlocked(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                Vec<pallet_settlement::VenueId>,
            ),
            #[codec(index = 13u8)]
            LegFailedExecution(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
                pallet_settlement::LegId,
            ),
            #[codec(index = 14u8)]
            InstructionFailed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
            ),
            #[codec(index = 15u8)]
            InstructionExecuted(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
            ),
            #[codec(index = 16u8)]
            VenueUnauthorized(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                pallet_settlement::VenueId,
            ),
            #[codec(index = 17u8)]
            SchedulingFailed(sp_runtime::DispatchError),
            #[codec(index = 18u8)]
            InstructionRescheduled(
                polymesh_primitives::identity_id::IdentityId,
                pallet_settlement::InstructionId,
            ),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct InstructionId(pub u64);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ReceiptMetadata(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct VenueId(pub u64);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct VenueDetails(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum LegStatus<AccountId> {
            #[codec(index = 0u8)]
            PendingTokenLock,
            #[codec(index = 1u8)]
            ExecutionPending,
            #[codec(index = 2u8)]
            ExecutionToBeSkipped(AccountId, u64),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Venue {
            pub creator: polymesh_primitives::identity_id::IdentityId,
            pub venue_type: pallet_settlement::VenueType,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum SettlementType<BlockNumber> {
            #[codec(index = 0u8)]
            SettleOnAffirmation,
            #[codec(index = 1u8)]
            SettleOnBlock(BlockNumber),
        }
    }
    pub mod pallet_rewards {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            UnknownItnAddress,
            #[codec(index = 1u8)]
            ItnRewardAlreadyClaimed,
            #[codec(index = 2u8)]
            InvalidSignature,
            #[codec(index = 3u8)]
            UnableToCovertBalance,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum ItnRewardStatus {
            #[codec(index = 0u8)]
            Unclaimed(u128),
            #[codec(index = 1u8)]
            Claimed,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId> {
            #[codec(index = 0u8)]
            ItnRewardClaimed(AccountId, u128),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            claim_itn_reward {
                reward_address: sp_core::crypto::AccountId32,
                itn_address: sp_core::crypto::AccountId32,
                signature: sp_runtime::MultiSignature,
            },
            #[codec(index = 1u8)]
            set_itn_reward_status {
                itn_address: sp_core::crypto::AccountId32,
                status: pallet_rewards::ItnRewardStatus,
            },
        }
    }
    pub mod polymesh_contracts {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Event {}
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            instantiate_with_code_perms {
                endowment: u128,
                gas_limit: u64,
                storage_deposit_limit: Option<u128>,
                code: Vec<u8>,
                data: Vec<u8>,
                salt: Vec<u8>,
                perms: polymesh_primitives::secondary_key::Permissions,
            },
            #[codec(index = 1u8)]
            instantiate_with_hash_perms {
                endowment: u128,
                gas_limit: u64,
                storage_deposit_limit: Option<u128>,
                code_hash: primitive_types::H256,
                data: Vec<u8>,
                salt: Vec<u8>,
                perms: polymesh_primitives::secondary_key::Permissions,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            RuntimeCallNotFound,
            #[codec(index = 1u8)]
            DataLeftAfterDecoding,
            #[codec(index = 2u8)]
            InLenTooLarge,
            #[codec(index = 3u8)]
            InstantiatorWithNoIdentity,
        }
    }
    pub mod pallet_external_agents {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            create_group {
                ticker: polymesh_primitives::ticker::Ticker,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
            },
            #[codec(index = 1u8)]
            set_group_permissions {
                ticker: polymesh_primitives::ticker::Ticker,
                id: polymesh_primitives::agent::AGId,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
            },
            #[codec(index = 2u8)]
            remove_agent {
                ticker: polymesh_primitives::ticker::Ticker,
                agent: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 3u8)]
            abdicate {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 4u8)]
            change_group {
                ticker: polymesh_primitives::ticker::Ticker,
                agent: polymesh_primitives::identity_id::IdentityId,
                group: polymesh_primitives::agent::AgentGroup,
            },
            #[codec(index = 5u8)]
            accept_become_agent { auth_id: u64 },
            #[codec(index = 6u8)]
            create_group_and_add_auth {
                ticker: polymesh_primitives::ticker::Ticker,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
                target: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
            },
            #[codec(index = 7u8)]
            create_and_change_custom_group {
                ticker: polymesh_primitives::ticker::Ticker,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
                agent: polymesh_primitives::identity_id::IdentityId,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            NoSuchAG,
            #[codec(index = 1u8)]
            UnauthorizedAgent,
            #[codec(index = 2u8)]
            AlreadyAnAgent,
            #[codec(index = 3u8)]
            NotAnAgent,
            #[codec(index = 4u8)]
            RemovingLastFullAgent,
            #[codec(index = 5u8)]
            SecondaryKeyNotAuthorizedForAsset,
        }
    }
    pub mod pallet_compliance_manager {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Event {
            #[codec(index = 0u8)]
            ComplianceRequirementCreated(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::compliance_manager::ComplianceRequirement,
            ),
            #[codec(index = 1u8)]
            ComplianceRequirementRemoved(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                u32,
            ),
            #[codec(index = 2u8)]
            AssetComplianceReplaced(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                Vec<polymesh_primitives::compliance_manager::ComplianceRequirement>,
            ),
            #[codec(index = 3u8)]
            AssetComplianceReset(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
            ),
            #[codec(index = 4u8)]
            AssetComplianceResumed(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
            ),
            #[codec(index = 5u8)]
            AssetCompliancePaused(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
            ),
            #[codec(index = 6u8)]
            ComplianceRequirementChanged(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::compliance_manager::ComplianceRequirement,
            ),
            #[codec(index = 7u8)]
            TrustedDefaultClaimIssuerAdded(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::condition::TrustedIssuer,
            ),
            #[codec(index = 8u8)]
            TrustedDefaultClaimIssuerRemoved(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::identity_id::IdentityId,
            ),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            add_compliance_requirement {
                ticker: polymesh_primitives::ticker::Ticker,
                sender_conditions: Vec<polymesh_primitives::condition::Condition>,
                receiver_conditions: Vec<polymesh_primitives::condition::Condition>,
            },
            #[codec(index = 1u8)]
            remove_compliance_requirement {
                ticker: polymesh_primitives::ticker::Ticker,
                id: u32,
            },
            #[codec(index = 2u8)]
            replace_asset_compliance {
                ticker: polymesh_primitives::ticker::Ticker,
                asset_compliance:
                    Vec<polymesh_primitives::compliance_manager::ComplianceRequirement>,
            },
            #[codec(index = 3u8)]
            reset_asset_compliance {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 4u8)]
            pause_asset_compliance {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 5u8)]
            resume_asset_compliance {
                ticker: polymesh_primitives::ticker::Ticker,
            },
            #[codec(index = 6u8)]
            add_default_trusted_claim_issuer {
                ticker: polymesh_primitives::ticker::Ticker,
                issuer: polymesh_primitives::condition::TrustedIssuer,
            },
            #[codec(index = 7u8)]
            remove_default_trusted_claim_issuer {
                ticker: polymesh_primitives::ticker::Ticker,
                issuer: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 8u8)]
            change_compliance_requirement {
                ticker: polymesh_primitives::ticker::Ticker,
                new_req: polymesh_primitives::compliance_manager::ComplianceRequirement,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            Unauthorized,
            #[codec(index = 1u8)]
            DidNotExist,
            #[codec(index = 2u8)]
            InvalidComplianceRequirementId,
            #[codec(index = 3u8)]
            IncorrectOperationOnTrustedIssuer,
            #[codec(index = 4u8)]
            DuplicateComplianceRequirements,
            #[codec(index = 5u8)]
            ComplianceRequirementTooComplex,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
    }
    pub mod pallet_permissions {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct StoreCallMetadata();
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            UnauthorizedCaller,
        }
    }
    pub mod pallet_balances {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            transfer {
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
            },
            #[codec(index = 1u8)]
            transfer_with_memo {
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
                memo: Option<polymesh_common_utilities::traits::balances::Memo>,
            },
            #[codec(index = 2u8)]
            deposit_block_reward_reserve_balance { value: ::codec::Compact<u128> },
            #[codec(index = 3u8)]
            set_balance {
                who: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                new_free: ::codec::Compact<u128>,
                new_reserved: ::codec::Compact<u128>,
            },
            #[codec(index = 4u8)]
            force_transfer {
                source: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
            },
            #[codec(index = 5u8)]
            burn_account_balance { amount: u128 },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct BalanceLock<Balance> {
            pub id: [u8; 8usize],
            pub amount: Balance,
            pub reasons: polymesh_common_utilities::traits::balances::Reasons,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            LiquidityRestrictions,
            #[codec(index = 1u8)]
            Overflow,
            #[codec(index = 2u8)]
            InsufficientBalance,
            #[codec(index = 3u8)]
            ExistentialDeposit,
            #[codec(index = 4u8)]
            ReceiverCddMissing,
        }
    }
    pub mod pallet_preimage {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                Noted { hash: primitive_types::H256 },
                #[codec(index = 1u8)]
                Requested { hash: primitive_types::H256 },
                #[codec(index = 2u8)]
                Cleared { hash: primitive_types::H256 },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                TooLarge,
                #[codec(index = 1u8)]
                AlreadyNoted,
                #[codec(index = 2u8)]
                NotAuthorized,
                #[codec(index = 3u8)]
                NotNoted,
                #[codec(index = 4u8)]
                Requested,
                #[codec(index = 5u8)]
                NotRequested,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                note_preimage { bytes: Vec<u8> },
                #[codec(index = 1u8)]
                unnote_preimage { hash: primitive_types::H256 },
                #[codec(index = 2u8)]
                request_preimage { hash: primitive_types::H256 },
                #[codec(index = 3u8)]
                unrequest_preimage { hash: primitive_types::H256 },
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RequestStatus<AccountId, Balance> {
            #[codec(index = 0u8)]
            Unrequested(Option<(AccountId, Balance)>),
            #[codec(index = 1u8)]
            Requested(u32),
        }
    }
    pub mod pallet_relayer {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            set_paying_key {
                user_key: sp_core::crypto::AccountId32,
                polyx_limit: u128,
            },
            #[codec(index = 1u8)]
            accept_paying_key { auth_id: u64 },
            #[codec(index = 2u8)]
            remove_paying_key {
                user_key: sp_core::crypto::AccountId32,
                paying_key: sp_core::crypto::AccountId32,
            },
            #[codec(index = 3u8)]
            update_polyx_limit {
                user_key: sp_core::crypto::AccountId32,
                polyx_limit: u128,
            },
            #[codec(index = 4u8)]
            increase_polyx_limit {
                user_key: sp_core::crypto::AccountId32,
                amount: u128,
            },
            #[codec(index = 5u8)]
            decrease_polyx_limit {
                user_key: sp_core::crypto::AccountId32,
                amount: u128,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Subsidy<Acc> {
            pub paying_key: Acc,
            pub remaining: u128,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            UserKeyCddMissing,
            #[codec(index = 1u8)]
            PayingKeyCddMissing,
            #[codec(index = 2u8)]
            NoPayingKey,
            #[codec(index = 3u8)]
            NotPayingKey,
            #[codec(index = 4u8)]
            NotAuthorizedForPayingKey,
            #[codec(index = 5u8)]
            NotAuthorizedForUserKey,
            #[codec(index = 6u8)]
            Overflow,
        }
    }
    pub mod pallet_sto {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum FundraiserStatus {
            #[codec(index = 0u8)]
            Live,
            #[codec(index = 1u8)]
            Frozen,
            #[codec(index = 2u8)]
            Closed,
            #[codec(index = 3u8)]
            ClosedEarly,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            create_fundraiser {
                offering_portfolio: polymesh_primitives::identity_id::PortfolioId,
                offering_asset: polymesh_primitives::ticker::Ticker,
                raising_portfolio: polymesh_primitives::identity_id::PortfolioId,
                raising_asset: polymesh_primitives::ticker::Ticker,
                tiers: Vec<pallet_sto::PriceTier>,
                venue_id: pallet_settlement::VenueId,
                start: Option<u64>,
                end: Option<u64>,
                minimum_investment: u128,
                fundraiser_name: pallet_sto::FundraiserName,
            },
            #[codec(index = 1u8)]
            invest {
                investment_portfolio: polymesh_primitives::identity_id::PortfolioId,
                funding_portfolio: polymesh_primitives::identity_id::PortfolioId,
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
                purchase_amount: u128,
                max_price: Option<u128>,
                receipt: Option<
                    pallet_settlement::ReceiptDetails<
                        sp_core::crypto::AccountId32,
                        sp_runtime::MultiSignature,
                    >,
                >,
            },
            #[codec(index = 2u8)]
            freeze_fundraiser {
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
            },
            #[codec(index = 3u8)]
            unfreeze_fundraiser {
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
            },
            #[codec(index = 4u8)]
            modify_fundraiser_window {
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
                start: u64,
                end: Option<u64>,
            },
            #[codec(index = 5u8)]
            stop {
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            Unauthorized,
            #[codec(index = 1u8)]
            Overflow,
            #[codec(index = 2u8)]
            InsufficientTokensRemaining,
            #[codec(index = 3u8)]
            FundraiserNotFound,
            #[codec(index = 4u8)]
            FundraiserNotLive,
            #[codec(index = 5u8)]
            FundraiserClosed,
            #[codec(index = 6u8)]
            FundraiserExpired,
            #[codec(index = 7u8)]
            InvalidVenue,
            #[codec(index = 8u8)]
            InvalidPriceTiers,
            #[codec(index = 9u8)]
            InvalidOfferingWindow,
            #[codec(index = 10u8)]
            MaxPriceExceeded,
            #[codec(index = 11u8)]
            InvestmentAmountTooLow,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct FundraiserId(pub u64);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<Moment> {
            #[codec(index = 0u8)]
            FundraiserCreated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_sto::FundraiserId,
                pallet_sto::FundraiserName,
                pallet_sto::Fundraiser<Moment>,
            ),
            #[codec(index = 1u8)]
            Invested(
                polymesh_primitives::identity_id::IdentityId,
                pallet_sto::FundraiserId,
                polymesh_primitives::ticker::Ticker,
                polymesh_primitives::ticker::Ticker,
                u128,
                u128,
            ),
            #[codec(index = 2u8)]
            FundraiserFrozen(
                polymesh_primitives::identity_id::IdentityId,
                pallet_sto::FundraiserId,
            ),
            #[codec(index = 3u8)]
            FundraiserUnfrozen(
                polymesh_primitives::identity_id::IdentityId,
                pallet_sto::FundraiserId,
            ),
            #[codec(index = 4u8)]
            FundraiserWindowModified(
                polymesh_primitives::event_only::EventOnly<
                    polymesh_primitives::identity_id::IdentityId,
                >,
                pallet_sto::FundraiserId,
                Moment,
                Option<Moment>,
                Moment,
                Option<Moment>,
            ),
            #[codec(index = 5u8)]
            FundraiserClosed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_sto::FundraiserId,
            ),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Fundraiser<Moment> {
            pub creator: polymesh_primitives::identity_id::IdentityId,
            pub offering_portfolio: polymesh_primitives::identity_id::PortfolioId,
            pub offering_asset: polymesh_primitives::ticker::Ticker,
            pub raising_portfolio: polymesh_primitives::identity_id::PortfolioId,
            pub raising_asset: polymesh_primitives::ticker::Ticker,
            pub tiers: Vec<pallet_sto::FundraiserTier>,
            pub venue_id: pallet_settlement::VenueId,
            pub start: Moment,
            pub end: Option<Moment>,
            pub status: pallet_sto::FundraiserStatus,
            pub minimum_investment: u128,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct FundraiserName(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct FundraiserTier {
            pub total: u128,
            pub price: u128,
            pub remaining: u128,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PriceTier {
            pub total: u128,
            pub price: u128,
        }
    }
    pub mod polymesh_primitives {
        use super::*;
        pub mod calendar {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct CheckpointId(pub u64);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct CalendarPeriod {
                pub unit: polymesh_primitives::calendar::CalendarUnit,
                pub amount: u64,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum CalendarUnit {
                #[codec(index = 0u8)]
                Second,
                #[codec(index = 1u8)]
                Minute,
                #[codec(index = 2u8)]
                Hour,
                #[codec(index = 3u8)]
                Day,
                #[codec(index = 4u8)]
                Week,
                #[codec(index = 5u8)]
                Month,
                #[codec(index = 6u8)]
                Year,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct CheckpointSchedule {
                pub start: u64,
                pub period: polymesh_primitives::calendar::CalendarPeriod,
            }
        }
        pub mod document {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct DocumentType(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct DocumentName(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct DocumentId(pub u32);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Document {
                pub uri: polymesh_primitives::document::DocumentUri,
                pub content_hash: polymesh_primitives::document_hash::DocumentHash,
                pub name: polymesh_primitives::document::DocumentName,
                pub doc_type: Option<polymesh_primitives::document::DocumentType>,
                pub filing_date: Option<u64>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct DocumentUri(pub Vec<u8>);
        }
        pub mod agent {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AGId(pub u32);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AgentGroup {
                #[codec(index = 0u8)]
                Full,
                #[codec(index = 1u8)]
                Custom(polymesh_primitives::agent::AGId),
                #[codec(index = 2u8)]
                ExceptMeta,
                #[codec(index = 3u8)]
                PolymeshV1CAA,
                #[codec(index = 4u8)]
                PolymeshV1PIA,
            }
        }
        pub mod ethereum {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct EthereumAddress(pub [u8; 20usize]);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct EcdsaSignature(pub [u8; 65usize]);
        }
        pub mod ticker {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Ticker(pub [u8; 12usize]);
        }
        pub mod cdd_id {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct InvestorUid(pub [u8; 16usize]);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct CddId(pub [u8; 32usize]);
        }
        pub mod subset {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum SubsetRestriction<A> {
                #[codec(index = 0u8)]
                Whole,
                #[codec(index = 1u8)]
                These(Vec<A>),
                #[codec(index = 2u8)]
                Except(Vec<A>),
            }
        }
        pub mod statistics {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum StatOpType {
                #[codec(index = 0u8)]
                Count,
                #[codec(index = 1u8)]
                Balance,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct StatUpdate {
                pub key2: polymesh_primitives::statistics::Stat2ndKey,
                pub value: Option<u128>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum StatClaim {
                #[codec(index = 0u8)]
                Accredited(bool),
                #[codec(index = 1u8)]
                Affiliate(bool),
                #[codec(index = 2u8)]
                Jurisdiction(Option<polymesh_primitives::jurisdiction::CountryCode>),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct HashablePermill(pub sp_arithmetic::per_things::Permill);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Stat1stKey {
                pub asset: polymesh_primitives::statistics::AssetScope,
                pub stat_type: polymesh_primitives::statistics::StatType,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AssetScope {
                #[codec(index = 0u8)]
                Ticker(polymesh_primitives::ticker::Ticker),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct StatType {
                pub op: polymesh_primitives::statistics::StatOpType,
                pub claim_issuer: Option<(
                    polymesh_primitives::identity_claim::ClaimType,
                    polymesh_primitives::identity_id::IdentityId,
                )>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Stat2ndKey {
                #[codec(index = 0u8)]
                NoClaimStat,
                #[codec(index = 1u8)]
                Claim(polymesh_primitives::statistics::StatClaim),
            }
        }
        pub mod condition {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum TargetIdentity {
                #[codec(index = 0u8)]
                ExternalAgent,
                #[codec(index = 1u8)]
                Specific(polymesh_primitives::identity_id::IdentityId),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Condition {
                pub condition_type: polymesh_primitives::condition::ConditionType,
                pub issuers: Vec<polymesh_primitives::condition::TrustedIssuer>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum ConditionType {
                #[codec(index = 0u8)]
                IsPresent(polymesh_primitives::identity_claim::Claim),
                #[codec(index = 1u8)]
                IsAbsent(polymesh_primitives::identity_claim::Claim),
                #[codec(index = 2u8)]
                IsAnyOf(Vec<polymesh_primitives::identity_claim::Claim>),
                #[codec(index = 3u8)]
                IsNoneOf(Vec<polymesh_primitives::identity_claim::Claim>),
                #[codec(index = 4u8)]
                IsIdentity(polymesh_primitives::condition::TargetIdentity),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct TrustedIssuer {
                pub issuer: polymesh_primitives::identity_id::IdentityId,
                pub trusted_for: polymesh_primitives::condition::TrustedFor,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum TrustedFor {
                #[codec(index = 0u8)]
                Any,
                #[codec(index = 1u8)]
                Specific(Vec<polymesh_primitives::identity_claim::ClaimType>),
            }
        }
        pub mod authorization {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Authorization<AccountId, Moment> {
                pub authorization_data:
                    polymesh_primitives::authorization::AuthorizationData<AccountId>,
                pub authorized_by: polymesh_primitives::identity_id::IdentityId,
                pub expiry: Option<Moment>,
                pub auth_id: Moment,
                pub count: u32,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AuthorizationData<AccountId> {
                #[codec(index = 0u8)]
                AttestPrimaryKeyRotation(polymesh_primitives::identity_id::IdentityId),
                #[codec(index = 1u8)]
                RotatePrimaryKey,
                #[codec(index = 2u8)]
                TransferTicker(polymesh_primitives::ticker::Ticker),
                #[codec(index = 3u8)]
                AddMultiSigSigner(AccountId),
                #[codec(index = 4u8)]
                TransferAssetOwnership(polymesh_primitives::ticker::Ticker),
                #[codec(index = 5u8)]
                JoinIdentity(polymesh_primitives::secondary_key::Permissions),
                #[codec(index = 6u8)]
                PortfolioCustody(polymesh_primitives::identity_id::PortfolioId),
                #[codec(index = 7u8)]
                BecomeAgent(
                    polymesh_primitives::ticker::Ticker,
                    polymesh_primitives::agent::AgentGroup,
                ),
                #[codec(index = 8u8)]
                AddRelayerPayingKey(AccountId, AccountId, u128),
                #[codec(index = 9u8)]
                RotatePrimaryKeyToSecondary(polymesh_primitives::secondary_key::Permissions),
            }
        }
        pub mod asset {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetName(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AssetType {
                #[codec(index = 0u8)]
                EquityCommon,
                #[codec(index = 1u8)]
                EquityPreferred,
                #[codec(index = 2u8)]
                Commodity,
                #[codec(index = 3u8)]
                FixedIncome,
                #[codec(index = 4u8)]
                REIT,
                #[codec(index = 5u8)]
                Fund,
                #[codec(index = 6u8)]
                RevenueShareAgreement,
                #[codec(index = 7u8)]
                StructuredProduct,
                #[codec(index = 8u8)]
                Derivative,
                #[codec(index = 9u8)]
                Custom(polymesh_primitives::asset::CustomAssetTypeId),
                #[codec(index = 10u8)]
                StableCoin,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct CustomAssetTypeId(pub u32);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct FundingRoundName(pub Vec<u8>);
        }
        pub mod identity_id {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct IdentityId(pub [u8; 32usize]);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct PortfolioNumber(pub u64);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct PortfolioName(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct PortfolioId {
                pub did: polymesh_primitives::identity_id::IdentityId,
                pub kind: polymesh_primitives::identity_id::PortfolioKind,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum PortfolioKind {
                #[codec(index = 0u8)]
                Default,
                #[codec(index = 1u8)]
                User(polymesh_primitives::identity_id::PortfolioNumber),
            }
        }
        pub mod secondary_key {
            use super::*;
            pub mod v1 {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct SecondaryKey<AccountId> {
                    pub signer: polymesh_primitives::secondary_key::Signatory<AccountId>,
                    pub permissions: polymesh_primitives::secondary_key::Permissions,
                }
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum KeyRecord<AccountId> {
                #[codec(index = 0u8)]
                PrimaryKey(polymesh_primitives::identity_id::IdentityId),
                #[codec(index = 1u8)]
                SecondaryKey(
                    polymesh_primitives::identity_id::IdentityId,
                    polymesh_primitives::secondary_key::Permissions,
                ),
                #[codec(index = 2u8)]
                MultiSigSignerKey(AccountId),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct PalletPermissions {
                pub pallet_name: polymesh_primitives::PalletName,
                pub dispatchable_names: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::DispatchableName,
                >,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Signatory<AccountId> {
                #[codec(index = 0u8)]
                Identity(polymesh_primitives::identity_id::IdentityId),
                #[codec(index = 1u8)]
                Account(AccountId),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct Permissions {
                pub asset: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::ticker::Ticker,
                >,
                pub extrinsic: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
                pub portfolio: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::identity_id::PortfolioId,
                >,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct SecondaryKey<AccountId> {
                pub key: AccountId,
                pub permissions: polymesh_primitives::secondary_key::Permissions,
            }
        }
        pub mod asset_identifier {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AssetIdentifier {
                #[codec(index = 0u8)]
                CUSIP([u8; 9usize]),
                #[codec(index = 1u8)]
                CINS([u8; 9usize]),
                #[codec(index = 2u8)]
                ISIN([u8; 12usize]),
                #[codec(index = 3u8)]
                LEI([u8; 20usize]),
                #[codec(index = 4u8)]
                FIGI([u8; 12usize]),
            }
        }
        pub mod compliance_manager {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetCompliance {
                pub paused: bool,
                pub requirements:
                    Vec<polymesh_primitives::compliance_manager::ComplianceRequirement>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct ComplianceRequirement {
                pub sender_conditions: Vec<polymesh_primitives::condition::Condition>,
                pub receiver_conditions: Vec<polymesh_primitives::condition::Condition>,
                pub id: u32,
            }
        }
        pub mod jurisdiction {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum CountryCode {
                #[codec(index = 0u8)]
                AF,
                #[codec(index = 1u8)]
                AX,
                #[codec(index = 2u8)]
                AL,
                #[codec(index = 3u8)]
                DZ,
                #[codec(index = 4u8)]
                AS,
                #[codec(index = 5u8)]
                AD,
                #[codec(index = 6u8)]
                AO,
                #[codec(index = 7u8)]
                AI,
                #[codec(index = 8u8)]
                AQ,
                #[codec(index = 9u8)]
                AG,
                #[codec(index = 10u8)]
                AR,
                #[codec(index = 11u8)]
                AM,
                #[codec(index = 12u8)]
                AW,
                #[codec(index = 13u8)]
                AU,
                #[codec(index = 14u8)]
                AT,
                #[codec(index = 15u8)]
                AZ,
                #[codec(index = 16u8)]
                BS,
                #[codec(index = 17u8)]
                BH,
                #[codec(index = 18u8)]
                BD,
                #[codec(index = 19u8)]
                BB,
                #[codec(index = 20u8)]
                BY,
                #[codec(index = 21u8)]
                BE,
                #[codec(index = 22u8)]
                BZ,
                #[codec(index = 23u8)]
                BJ,
                #[codec(index = 24u8)]
                BM,
                #[codec(index = 25u8)]
                BT,
                #[codec(index = 26u8)]
                BO,
                #[codec(index = 27u8)]
                BA,
                #[codec(index = 28u8)]
                BW,
                #[codec(index = 29u8)]
                BV,
                #[codec(index = 30u8)]
                BR,
                #[codec(index = 31u8)]
                VG,
                #[codec(index = 32u8)]
                IO,
                #[codec(index = 33u8)]
                BN,
                #[codec(index = 34u8)]
                BG,
                #[codec(index = 35u8)]
                BF,
                #[codec(index = 36u8)]
                BI,
                #[codec(index = 37u8)]
                KH,
                #[codec(index = 38u8)]
                CM,
                #[codec(index = 39u8)]
                CA,
                #[codec(index = 40u8)]
                CV,
                #[codec(index = 41u8)]
                KY,
                #[codec(index = 42u8)]
                CF,
                #[codec(index = 43u8)]
                TD,
                #[codec(index = 44u8)]
                CL,
                #[codec(index = 45u8)]
                CN,
                #[codec(index = 46u8)]
                HK,
                #[codec(index = 47u8)]
                MO,
                #[codec(index = 48u8)]
                CX,
                #[codec(index = 49u8)]
                CC,
                #[codec(index = 50u8)]
                CO,
                #[codec(index = 51u8)]
                KM,
                #[codec(index = 52u8)]
                CG,
                #[codec(index = 53u8)]
                CD,
                #[codec(index = 54u8)]
                CK,
                #[codec(index = 55u8)]
                CR,
                #[codec(index = 56u8)]
                CI,
                #[codec(index = 57u8)]
                HR,
                #[codec(index = 58u8)]
                CU,
                #[codec(index = 59u8)]
                CY,
                #[codec(index = 60u8)]
                CZ,
                #[codec(index = 61u8)]
                DK,
                #[codec(index = 62u8)]
                DJ,
                #[codec(index = 63u8)]
                DM,
                #[codec(index = 64u8)]
                DO,
                #[codec(index = 65u8)]
                EC,
                #[codec(index = 66u8)]
                EG,
                #[codec(index = 67u8)]
                SV,
                #[codec(index = 68u8)]
                GQ,
                #[codec(index = 69u8)]
                ER,
                #[codec(index = 70u8)]
                EE,
                #[codec(index = 71u8)]
                ET,
                #[codec(index = 72u8)]
                FK,
                #[codec(index = 73u8)]
                FO,
                #[codec(index = 74u8)]
                FJ,
                #[codec(index = 75u8)]
                FI,
                #[codec(index = 76u8)]
                FR,
                #[codec(index = 77u8)]
                GF,
                #[codec(index = 78u8)]
                PF,
                #[codec(index = 79u8)]
                TF,
                #[codec(index = 80u8)]
                GA,
                #[codec(index = 81u8)]
                GM,
                #[codec(index = 82u8)]
                GE,
                #[codec(index = 83u8)]
                DE,
                #[codec(index = 84u8)]
                GH,
                #[codec(index = 85u8)]
                GI,
                #[codec(index = 86u8)]
                GR,
                #[codec(index = 87u8)]
                GL,
                #[codec(index = 88u8)]
                GD,
                #[codec(index = 89u8)]
                GP,
                #[codec(index = 90u8)]
                GU,
                #[codec(index = 91u8)]
                GT,
                #[codec(index = 92u8)]
                GG,
                #[codec(index = 93u8)]
                GN,
                #[codec(index = 94u8)]
                GW,
                #[codec(index = 95u8)]
                GY,
                #[codec(index = 96u8)]
                HT,
                #[codec(index = 97u8)]
                HM,
                #[codec(index = 98u8)]
                VA,
                #[codec(index = 99u8)]
                HN,
                #[codec(index = 100u8)]
                HU,
                #[codec(index = 101u8)]
                IS,
                #[codec(index = 102u8)]
                IN,
                #[codec(index = 103u8)]
                ID,
                #[codec(index = 104u8)]
                IR,
                #[codec(index = 105u8)]
                IQ,
                #[codec(index = 106u8)]
                IE,
                #[codec(index = 107u8)]
                IM,
                #[codec(index = 108u8)]
                IL,
                #[codec(index = 109u8)]
                IT,
                #[codec(index = 110u8)]
                JM,
                #[codec(index = 111u8)]
                JP,
                #[codec(index = 112u8)]
                JE,
                #[codec(index = 113u8)]
                JO,
                #[codec(index = 114u8)]
                KZ,
                #[codec(index = 115u8)]
                KE,
                #[codec(index = 116u8)]
                KI,
                #[codec(index = 117u8)]
                KP,
                #[codec(index = 118u8)]
                KR,
                #[codec(index = 119u8)]
                KW,
                #[codec(index = 120u8)]
                KG,
                #[codec(index = 121u8)]
                LA,
                #[codec(index = 122u8)]
                LV,
                #[codec(index = 123u8)]
                LB,
                #[codec(index = 124u8)]
                LS,
                #[codec(index = 125u8)]
                LR,
                #[codec(index = 126u8)]
                LY,
                #[codec(index = 127u8)]
                LI,
                #[codec(index = 128u8)]
                LT,
                #[codec(index = 129u8)]
                LU,
                #[codec(index = 130u8)]
                MK,
                #[codec(index = 131u8)]
                MG,
                #[codec(index = 132u8)]
                MW,
                #[codec(index = 133u8)]
                MY,
                #[codec(index = 134u8)]
                MV,
                #[codec(index = 135u8)]
                ML,
                #[codec(index = 136u8)]
                MT,
                #[codec(index = 137u8)]
                MH,
                #[codec(index = 138u8)]
                MQ,
                #[codec(index = 139u8)]
                MR,
                #[codec(index = 140u8)]
                MU,
                #[codec(index = 141u8)]
                YT,
                #[codec(index = 142u8)]
                MX,
                #[codec(index = 143u8)]
                FM,
                #[codec(index = 144u8)]
                MD,
                #[codec(index = 145u8)]
                MC,
                #[codec(index = 146u8)]
                MN,
                #[codec(index = 147u8)]
                ME,
                #[codec(index = 148u8)]
                MS,
                #[codec(index = 149u8)]
                MA,
                #[codec(index = 150u8)]
                MZ,
                #[codec(index = 151u8)]
                MM,
                #[codec(index = 152u8)]
                NA,
                #[codec(index = 153u8)]
                NR,
                #[codec(index = 154u8)]
                NP,
                #[codec(index = 155u8)]
                NL,
                #[codec(index = 156u8)]
                AN,
                #[codec(index = 157u8)]
                NC,
                #[codec(index = 158u8)]
                NZ,
                #[codec(index = 159u8)]
                NI,
                #[codec(index = 160u8)]
                NE,
                #[codec(index = 161u8)]
                NG,
                #[codec(index = 162u8)]
                NU,
                #[codec(index = 163u8)]
                NF,
                #[codec(index = 164u8)]
                MP,
                #[codec(index = 165u8)]
                NO,
                #[codec(index = 166u8)]
                OM,
                #[codec(index = 167u8)]
                PK,
                #[codec(index = 168u8)]
                PW,
                #[codec(index = 169u8)]
                PS,
                #[codec(index = 170u8)]
                PA,
                #[codec(index = 171u8)]
                PG,
                #[codec(index = 172u8)]
                PY,
                #[codec(index = 173u8)]
                PE,
                #[codec(index = 174u8)]
                PH,
                #[codec(index = 175u8)]
                PN,
                #[codec(index = 176u8)]
                PL,
                #[codec(index = 177u8)]
                PT,
                #[codec(index = 178u8)]
                PR,
                #[codec(index = 179u8)]
                QA,
                #[codec(index = 180u8)]
                RE,
                #[codec(index = 181u8)]
                RO,
                #[codec(index = 182u8)]
                RU,
                #[codec(index = 183u8)]
                RW,
                #[codec(index = 184u8)]
                BL,
                #[codec(index = 185u8)]
                SH,
                #[codec(index = 186u8)]
                KN,
                #[codec(index = 187u8)]
                LC,
                #[codec(index = 188u8)]
                MF,
                #[codec(index = 189u8)]
                PM,
                #[codec(index = 190u8)]
                VC,
                #[codec(index = 191u8)]
                WS,
                #[codec(index = 192u8)]
                SM,
                #[codec(index = 193u8)]
                ST,
                #[codec(index = 194u8)]
                SA,
                #[codec(index = 195u8)]
                SN,
                #[codec(index = 196u8)]
                RS,
                #[codec(index = 197u8)]
                SC,
                #[codec(index = 198u8)]
                SL,
                #[codec(index = 199u8)]
                SG,
                #[codec(index = 200u8)]
                SK,
                #[codec(index = 201u8)]
                SI,
                #[codec(index = 202u8)]
                SB,
                #[codec(index = 203u8)]
                SO,
                #[codec(index = 204u8)]
                ZA,
                #[codec(index = 205u8)]
                GS,
                #[codec(index = 206u8)]
                SS,
                #[codec(index = 207u8)]
                ES,
                #[codec(index = 208u8)]
                LK,
                #[codec(index = 209u8)]
                SD,
                #[codec(index = 210u8)]
                SR,
                #[codec(index = 211u8)]
                SJ,
                #[codec(index = 212u8)]
                SZ,
                #[codec(index = 213u8)]
                SE,
                #[codec(index = 214u8)]
                CH,
                #[codec(index = 215u8)]
                SY,
                #[codec(index = 216u8)]
                TW,
                #[codec(index = 217u8)]
                TJ,
                #[codec(index = 218u8)]
                TZ,
                #[codec(index = 219u8)]
                TH,
                #[codec(index = 220u8)]
                TL,
                #[codec(index = 221u8)]
                TG,
                #[codec(index = 222u8)]
                TK,
                #[codec(index = 223u8)]
                TO,
                #[codec(index = 224u8)]
                TT,
                #[codec(index = 225u8)]
                TN,
                #[codec(index = 226u8)]
                TR,
                #[codec(index = 227u8)]
                TM,
                #[codec(index = 228u8)]
                TC,
                #[codec(index = 229u8)]
                TV,
                #[codec(index = 230u8)]
                UG,
                #[codec(index = 231u8)]
                UA,
                #[codec(index = 232u8)]
                AE,
                #[codec(index = 233u8)]
                GB,
                #[codec(index = 234u8)]
                US,
                #[codec(index = 235u8)]
                UM,
                #[codec(index = 236u8)]
                UY,
                #[codec(index = 237u8)]
                UZ,
                #[codec(index = 238u8)]
                VU,
                #[codec(index = 239u8)]
                VE,
                #[codec(index = 240u8)]
                VN,
                #[codec(index = 241u8)]
                VI,
                #[codec(index = 242u8)]
                WF,
                #[codec(index = 243u8)]
                EH,
                #[codec(index = 244u8)]
                YE,
                #[codec(index = 245u8)]
                ZM,
                #[codec(index = 246u8)]
                ZW,
                #[codec(index = 247u8)]
                BQ,
                #[codec(index = 248u8)]
                CW,
                #[codec(index = 249u8)]
                SX,
            }
        }
        pub mod event_only {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct EventOnly<T>(pub T);
        }
        pub mod transfer_compliance {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetTransferCompliance {
                pub paused: bool,
                pub requirements: Vec<polymesh_primitives::transfer_compliance::TransferCondition>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum TransferCondition {
                #[codec(index = 0u8)]
                MaxInvestorCount(u64),
                #[codec(index = 1u8)]
                MaxInvestorOwnership(polymesh_primitives::statistics::HashablePermill),
                #[codec(index = 2u8)]
                ClaimCount(
                    polymesh_primitives::statistics::StatClaim,
                    polymesh_primitives::identity_id::IdentityId,
                    u64,
                    Option<u64>,
                ),
                #[codec(index = 3u8)]
                ClaimOwnership(
                    polymesh_primitives::statistics::StatClaim,
                    polymesh_primitives::identity_id::IdentityId,
                    polymesh_primitives::statistics::HashablePermill,
                    polymesh_primitives::statistics::HashablePermill,
                ),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct TransferConditionExemptKey {
                pub asset: polymesh_primitives::statistics::AssetScope,
                pub op: polymesh_primitives::statistics::StatOpType,
                pub claim_type: Option<polymesh_primitives::identity_claim::ClaimType>,
            }
        }
        pub mod asset_metadata {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataGlobalKey(pub u64);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AssetMetadataKey {
                #[codec(index = 0u8)]
                Global(polymesh_primitives::asset_metadata::AssetMetadataGlobalKey),
                #[codec(index = 1u8)]
                Local(polymesh_primitives::asset_metadata::AssetMetadataLocalKey),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataValueDetail<Moment> {
                pub expire: Option<Moment>,
                pub lock_status:
                    polymesh_primitives::asset_metadata::AssetMetadataLockStatus<Moment>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataName(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataDescription(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataLocalKey(pub u64);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataValue(pub Vec<u8>);
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct AssetMetadataSpec {
                pub url: Option<polymesh_primitives::Url>,
                pub description:
                    Option<polymesh_primitives::asset_metadata::AssetMetadataDescription>,
                pub type_def: Option<Vec<u8>>,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum AssetMetadataLockStatus<Moment> {
                #[codec(index = 0u8)]
                Unlocked,
                #[codec(index = 1u8)]
                Locked,
                #[codec(index = 2u8)]
                LockedUntil(Moment),
            }
        }
        pub mod identity {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct DidRecord<AccountId> {
                pub primary_key: Option<AccountId>,
            }
        }
        pub mod document_hash {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum DocumentHash {
                #[codec(index = 0u8)]
                None,
                #[codec(index = 1u8)]
                H512([u8; 64usize]),
                #[codec(index = 2u8)]
                H384([u8; 48usize]),
                #[codec(index = 3u8)]
                H320([u8; 40usize]),
                #[codec(index = 4u8)]
                H256([u8; 32usize]),
                #[codec(index = 5u8)]
                H224([u8; 28usize]),
                #[codec(index = 6u8)]
                H192([u8; 24usize]),
                #[codec(index = 7u8)]
                H160([u8; 20usize]),
                #[codec(index = 8u8)]
                H128([u8; 16usize]),
            }
        }
        pub mod identity_claim {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum ClaimType {
                #[codec(index = 0u8)]
                Accredited,
                #[codec(index = 1u8)]
                Affiliate,
                #[codec(index = 2u8)]
                BuyLockup,
                #[codec(index = 3u8)]
                SellLockup,
                #[codec(index = 4u8)]
                CustomerDueDiligence,
                #[codec(index = 5u8)]
                KnowYourCustomer,
                #[codec(index = 6u8)]
                Jurisdiction,
                #[codec(index = 7u8)]
                Exempted,
                #[codec(index = 8u8)]
                Blocked,
                #[codec(index = 9u8)]
                InvestorUniqueness,
                #[codec(index = 10u8)]
                NoType,
                #[codec(index = 11u8)]
                InvestorUniquenessV2,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Claim {
                #[codec(index = 0u8)]
                Accredited(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 1u8)]
                Affiliate(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 2u8)]
                BuyLockup(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 3u8)]
                SellLockup(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 4u8)]
                CustomerDueDiligence(polymesh_primitives::cdd_id::CddId),
                #[codec(index = 5u8)]
                KnowYourCustomer(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 6u8)]
                Jurisdiction(
                    polymesh_primitives::jurisdiction::CountryCode,
                    polymesh_primitives::identity_claim::Scope,
                ),
                #[codec(index = 7u8)]
                Exempted(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 8u8)]
                Blocked(polymesh_primitives::identity_claim::Scope),
                #[codec(index = 9u8)]
                InvestorUniqueness(
                    polymesh_primitives::identity_claim::Scope,
                    polymesh_primitives::identity_id::IdentityId,
                    polymesh_primitives::cdd_id::CddId,
                ),
                #[codec(index = 10u8)]
                NoData,
                #[codec(index = 11u8)]
                InvestorUniquenessV2(polymesh_primitives::cdd_id::CddId),
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct IdentityClaim {
                pub claim_issuer: polymesh_primitives::identity_id::IdentityId,
                pub issuance_date: u64,
                pub last_update_date: u64,
                pub expiry: Option<u64>,
                pub claim: polymesh_primitives::identity_claim::Claim,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Scope {
                #[codec(index = 0u8)]
                Identity(polymesh_primitives::identity_id::IdentityId),
                #[codec(index = 1u8)]
                Ticker(polymesh_primitives::ticker::Ticker),
                #[codec(index = 2u8)]
                Custom(Vec<u8>),
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Url(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Beneficiary<Balance> {
            pub id: polymesh_primitives::identity_id::IdentityId,
            pub amount: Balance,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PosRatio(pub u32, pub u32);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PalletName(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct DispatchableName(pub Vec<u8>);
    }
    pub mod pallet_pips {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct SnapshotMetadata<BlockNumber, AccountId> {
            pub created_at: BlockNumber,
            pub made_by: AccountId,
            pub id: pallet_pips::SnapshotId,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct VotingResult {
            pub ayes_count: u32,
            pub ayes_stake: u128,
            pub nays_count: u32,
            pub nays_stake: u128,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Proposer<AccountId> {
            #[codec(index = 0u8)]
            Community(AccountId),
            #[codec(index = 1u8)]
            Committee(pallet_pips::Committee),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct SnapshotId(pub u32);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct DepositInfo<AccountId> {
            pub owner: AccountId,
            pub amount: u128,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            RescheduleNotByReleaseCoordinator,
            #[codec(index = 1u8)]
            NotFromCommunity,
            #[codec(index = 2u8)]
            NotByCommittee,
            #[codec(index = 3u8)]
            TooManyActivePips,
            #[codec(index = 4u8)]
            IncorrectDeposit,
            #[codec(index = 5u8)]
            InsufficientDeposit,
            #[codec(index = 6u8)]
            NoSuchProposal,
            #[codec(index = 7u8)]
            NotACommitteeMember,
            #[codec(index = 8u8)]
            InvalidFutureBlockNumber,
            #[codec(index = 9u8)]
            NumberOfVotesExceeded,
            #[codec(index = 10u8)]
            StakeAmountOfVotesExceeded,
            #[codec(index = 11u8)]
            MissingCurrentIdentity,
            #[codec(index = 12u8)]
            IncorrectProposalState,
            #[codec(index = 13u8)]
            CannotSkipPip,
            #[codec(index = 14u8)]
            SnapshotResultTooLarge,
            #[codec(index = 15u8)]
            SnapshotIdMismatch,
            #[codec(index = 16u8)]
            ScheduledProposalDoesntExist,
            #[codec(index = 17u8)]
            ProposalNotInScheduledState,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum ProposalData {
            #[codec(index = 0u8)]
            Hash(primitive_types::H256),
            #[codec(index = 1u8)]
            Proposal(Vec<u8>),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Vote(pub bool, pub u128);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Pip<Proposal, AccountId> {
            pub id: pallet_pips::PipId,
            pub proposal: Proposal,
            pub state: pallet_pips::ProposalState,
            pub proposer: pallet_pips::Proposer<AccountId>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId, BlockNumber> {
            #[codec(index = 0u8)]
            HistoricalPipsPruned(polymesh_primitives::identity_id::IdentityId, bool, bool),
            #[codec(index = 1u8)]
            ProposalCreated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::Proposer<AccountId>,
                pallet_pips::PipId,
                u128,
                Option<polymesh_primitives::Url>,
                Option<pallet_pips::PipDescription>,
                polymesh_common_utilities::MaybeBlock<BlockNumber>,
                pallet_pips::ProposalData,
            ),
            #[codec(index = 2u8)]
            ProposalStateUpdated(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                pallet_pips::ProposalState,
            ),
            #[codec(index = 3u8)]
            Voted(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                pallet_pips::PipId,
                bool,
                u128,
            ),
            #[codec(index = 4u8)]
            PipClosed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                bool,
            ),
            #[codec(index = 5u8)]
            ExecutionScheduled(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                BlockNumber,
            ),
            #[codec(index = 6u8)]
            DefaultEnactmentPeriodChanged(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                BlockNumber,
            ),
            #[codec(index = 7u8)]
            MinimumProposalDepositChanged(polymesh_primitives::identity_id::IdentityId, u128, u128),
            #[codec(index = 8u8)]
            PendingPipExpiryChanged(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_common_utilities::MaybeBlock<BlockNumber>,
                polymesh_common_utilities::MaybeBlock<BlockNumber>,
            ),
            #[codec(index = 9u8)]
            MaxPipSkipCountChanged(polymesh_primitives::identity_id::IdentityId, u8, u8),
            #[codec(index = 10u8)]
            ActivePipLimitChanged(
                polymesh_primitives::identity_id::IdentityId,
                BlockNumber,
                BlockNumber,
            ),
            #[codec(index = 11u8)]
            ProposalRefund(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                u128,
            ),
            #[codec(index = 12u8)]
            SnapshotCleared(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::SnapshotId,
            ),
            #[codec(index = 13u8)]
            SnapshotTaken(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::SnapshotId,
                Vec<pallet_pips::SnapshottedPip>,
            ),
            #[codec(index = 14u8)]
            PipSkipped(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                u8,
            ),
            #[codec(index = 15u8)]
            SnapshotResultsEnacted(
                polymesh_primitives::identity_id::IdentityId,
                Option<pallet_pips::SnapshotId>,
                Vec<(pallet_pips::PipId, u8)>,
                Vec<pallet_pips::PipId>,
                Vec<pallet_pips::PipId>,
            ),
            #[codec(index = 16u8)]
            ExecutionSchedulingFailed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                BlockNumber,
            ),
            #[codec(index = 17u8)]
            ExpiryScheduled(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                BlockNumber,
            ),
            #[codec(index = 18u8)]
            ExpirySchedulingFailed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_pips::PipId,
                BlockNumber,
            ),
            #[codec(index = 19u8)]
            ExecutionCancellingFailed(pallet_pips::PipId),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Committee {
            #[codec(index = 0u8)]
            Technical,
            #[codec(index = 1u8)]
            Upgrade,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct SnapshottedPip {
            pub id: pallet_pips::PipId,
            pub weight: (bool, u128),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum SnapshotResult {
            #[codec(index = 0u8)]
            Approve,
            #[codec(index = 1u8)]
            Reject,
            #[codec(index = 2u8)]
            Skip,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PipDescription(pub Vec<u8>);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum ProposalState {
            #[codec(index = 0u8)]
            Pending,
            #[codec(index = 1u8)]
            Rejected,
            #[codec(index = 2u8)]
            Scheduled,
            #[codec(index = 3u8)]
            Failed,
            #[codec(index = 4u8)]
            Executed,
            #[codec(index = 5u8)]
            Expired,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PipId(pub u32);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            set_prune_historical_pips { prune: bool },
            #[codec(index = 1u8)]
            set_min_proposal_deposit { deposit: u128 },
            #[codec(index = 2u8)]
            set_default_enactment_period { duration: u32 },
            #[codec(index = 3u8)]
            set_pending_pip_expiry {
                expiry: polymesh_common_utilities::MaybeBlock<u32>,
            },
            #[codec(index = 4u8)]
            set_max_pip_skip_count { max: u8 },
            #[codec(index = 5u8)]
            set_active_pip_limit { limit: u32 },
            #[codec(index = 6u8)]
            propose {
                proposal: ::std::boxed::Box<polymesh_runtime_develop::runtime::Call>,
                deposit: u128,
                url: Option<polymesh_primitives::Url>,
                description: Option<pallet_pips::PipDescription>,
            },
            #[codec(index = 7u8)]
            vote {
                id: pallet_pips::PipId,
                aye_or_nay: bool,
                deposit: u128,
            },
            #[codec(index = 8u8)]
            approve_committee_proposal { id: pallet_pips::PipId },
            #[codec(index = 9u8)]
            reject_proposal { id: pallet_pips::PipId },
            #[codec(index = 10u8)]
            prune_proposal { id: pallet_pips::PipId },
            #[codec(index = 11u8)]
            reschedule_execution {
                id: pallet_pips::PipId,
                until: Option<u32>,
            },
            #[codec(index = 12u8)]
            clear_snapshot,
            #[codec(index = 13u8)]
            snapshot,
            #[codec(index = 14u8)]
            enact_snapshot_results {
                results: Vec<(pallet_pips::PipId, pallet_pips::SnapshotResult)>,
            },
            #[codec(index = 15u8)]
            execute_scheduled_pip { id: pallet_pips::PipId },
            #[codec(index = 16u8)]
            expire_scheduled_pip {
                did: polymesh_primitives::identity_id::IdentityId,
                id: pallet_pips::PipId,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PipsMetadata<BlockNumber> {
            pub id: pallet_pips::PipId,
            pub url: Option<polymesh_primitives::Url>,
            pub description: Option<pallet_pips::PipDescription>,
            pub created_at: BlockNumber,
            pub transaction_version: BlockNumber,
            pub expiry: polymesh_common_utilities::MaybeBlock<BlockNumber>,
        }
    }
    pub mod pallet_bridge {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<AccountId, BlockNumber> {
            #[codec(index = 0u8)]
            ControllerChanged(polymesh_primitives::identity_id::IdentityId, AccountId),
            #[codec(index = 1u8)]
            AdminChanged(polymesh_primitives::identity_id::IdentityId, AccountId),
            #[codec(index = 2u8)]
            TimelockChanged(polymesh_primitives::identity_id::IdentityId, BlockNumber),
            #[codec(index = 3u8)]
            Bridged(
                polymesh_primitives::identity_id::IdentityId,
                pallet_bridge::BridgeTx<AccountId>,
            ),
            #[codec(index = 4u8)]
            Frozen(polymesh_primitives::identity_id::IdentityId),
            #[codec(index = 5u8)]
            Unfrozen(polymesh_primitives::identity_id::IdentityId),
            #[codec(index = 6u8)]
            FrozenTx(
                polymesh_primitives::identity_id::IdentityId,
                pallet_bridge::BridgeTx<AccountId>,
            ),
            #[codec(index = 7u8)]
            UnfrozenTx(
                polymesh_primitives::identity_id::IdentityId,
                pallet_bridge::BridgeTx<AccountId>,
            ),
            #[codec(index = 8u8)]
            ExemptedUpdated(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::IdentityId,
                bool,
            ),
            #[codec(index = 9u8)]
            BridgeLimitUpdated(
                polymesh_primitives::identity_id::IdentityId,
                u128,
                BlockNumber,
            ),
            #[codec(index = 10u8)]
            TxsHandled(Vec<(AccountId, BlockNumber, pallet_bridge::HandledTxStatus)>),
            #[codec(index = 11u8)]
            BridgeTxScheduled(
                polymesh_primitives::identity_id::IdentityId,
                pallet_bridge::BridgeTx<AccountId>,
                BlockNumber,
            ),
            #[codec(index = 12u8)]
            BridgeTxScheduleFailed(
                polymesh_primitives::identity_id::IdentityId,
                pallet_bridge::BridgeTx<AccountId>,
                Vec<u8>,
            ),
            #[codec(index = 13u8)]
            FreezeAdminAdded(polymesh_primitives::identity_id::IdentityId, AccountId),
            #[codec(index = 14u8)]
            FreezeAdminRemoved(polymesh_primitives::identity_id::IdentityId, AccountId),
            #[codec(index = 15u8)]
            TxRemoved(
                polymesh_primitives::identity_id::IdentityId,
                pallet_bridge::BridgeTx<AccountId>,
            ),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum HandledTxStatus {
            #[codec(index = 0u8)]
            Success,
            #[codec(index = 1u8)]
            Error(Vec<u8>),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct BridgeTxDetail<BlockNumber> {
            pub amount: u128,
            pub status: pallet_bridge::BridgeTxStatus,
            pub execution_block: BlockNumber,
            pub tx_hash: primitive_types::H256,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            change_controller {
                controller: sp_core::crypto::AccountId32,
            },
            #[codec(index = 1u8)]
            change_admin { admin: sp_core::crypto::AccountId32 },
            #[codec(index = 2u8)]
            change_timelock { timelock: u32 },
            #[codec(index = 3u8)]
            freeze,
            #[codec(index = 4u8)]
            unfreeze,
            #[codec(index = 5u8)]
            change_bridge_limit { amount: u128, duration: u32 },
            #[codec(index = 6u8)]
            change_bridge_exempted {
                exempted: Vec<(polymesh_primitives::identity_id::IdentityId, bool)>,
            },
            #[codec(index = 7u8)]
            force_handle_bridge_tx {
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 8u8)]
            batch_propose_bridge_tx {
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            },
            #[codec(index = 9u8)]
            propose_bridge_tx {
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 10u8)]
            handle_bridge_tx {
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 11u8)]
            freeze_txs {
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            },
            #[codec(index = 12u8)]
            unfreeze_txs {
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            },
            #[codec(index = 13u8)]
            handle_scheduled_bridge_tx {
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 14u8)]
            add_freeze_admin {
                freeze_admin: sp_core::crypto::AccountId32,
            },
            #[codec(index = 15u8)]
            remove_freeze_admin {
                freeze_admin: sp_core::crypto::AccountId32,
            },
            #[codec(index = 16u8)]
            remove_txs {
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct BridgeTx<Account> {
            pub nonce: u32,
            pub recipient: Account,
            pub amount: u128,
            pub tx_hash: primitive_types::H256,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum BridgeTxStatus {
            #[codec(index = 0u8)]
            Absent,
            #[codec(index = 1u8)]
            Pending(u8),
            #[codec(index = 2u8)]
            Frozen,
            #[codec(index = 3u8)]
            Timelocked,
            #[codec(index = 4u8)]
            Handled,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            ControllerNotSet,
            #[codec(index = 1u8)]
            BadCaller,
            #[codec(index = 2u8)]
            BadAdmin,
            #[codec(index = 3u8)]
            NoValidCdd,
            #[codec(index = 4u8)]
            ProposalAlreadyHandled,
            #[codec(index = 5u8)]
            Unauthorized,
            #[codec(index = 6u8)]
            Frozen,
            #[codec(index = 7u8)]
            NotFrozen,
            #[codec(index = 8u8)]
            FrozenTx,
            #[codec(index = 9u8)]
            BridgeLimitReached,
            #[codec(index = 10u8)]
            Overflow,
            #[codec(index = 11u8)]
            DivisionByZero,
            #[codec(index = 12u8)]
            TimelockedTx,
        }
    }
    pub mod pallet_session {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                set_keys {
                    keys: polymesh_runtime_develop::runtime::SessionKeys,
                    proof: Vec<u8>,
                },
                #[codec(index = 1u8)]
                purge_keys,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                InvalidProof,
                #[codec(index = 1u8)]
                NoAssociatedValidatorId,
                #[codec(index = 2u8)]
                DuplicatedKey,
                #[codec(index = 3u8)]
                NoKeys,
                #[codec(index = 4u8)]
                NoAccount,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                NewSession { session_index: u32 },
            }
        }
    }
    pub mod polymesh_extensions {
        use super::*;
        pub mod check_weight {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct CheckWeight(pub frame_system::extensions::check_weight::CheckWeight);
        }
    }
    pub mod pallet_statistics {
        use super::*;
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            InvalidTransfer,
            #[codec(index = 1u8)]
            StatTypeMissing,
            #[codec(index = 2u8)]
            StatTypeNeededByTransferCondition,
            #[codec(index = 3u8)]
            CannotRemoveStatTypeInUse,
            #[codec(index = 4u8)]
            StatTypeLimitReached,
            #[codec(index = 5u8)]
            TransferConditionLimitReached,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            set_active_asset_stats {
                asset: polymesh_primitives::statistics::AssetScope,
                stat_types: Vec<polymesh_primitives::statistics::StatType>,
            },
            #[codec(index = 1u8)]
            batch_update_asset_stats {
                asset: polymesh_primitives::statistics::AssetScope,
                stat_type: polymesh_primitives::statistics::StatType,
                values: Vec<polymesh_primitives::statistics::StatUpdate>,
            },
            #[codec(index = 2u8)]
            set_asset_transfer_compliance {
                asset: polymesh_primitives::statistics::AssetScope,
                transfer_conditions:
                    Vec<polymesh_primitives::transfer_compliance::TransferCondition>,
            },
            #[codec(index = 3u8)]
            set_entities_exempt {
                is_exempt: bool,
                exempt_key: polymesh_primitives::transfer_compliance::TransferConditionExemptKey,
                entities: Vec<polymesh_primitives::identity_id::IdentityId>,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Version(pub u8);
    }
    pub mod pallet_indices {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                IndexAssigned {
                    who: sp_core::crypto::AccountId32,
                    index: u32,
                },
                #[codec(index = 1u8)]
                IndexFreed { index: u32 },
                #[codec(index = 2u8)]
                IndexFrozen {
                    index: u32,
                    who: sp_core::crypto::AccountId32,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                NotAssigned,
                #[codec(index = 1u8)]
                NotOwner,
                #[codec(index = 2u8)]
                InUse,
                #[codec(index = 3u8)]
                NotTransfer,
                #[codec(index = 4u8)]
                Permanent,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                claim { index: u32 },
                #[codec(index = 1u8)]
                transfer {
                    new: sp_core::crypto::AccountId32,
                    index: u32,
                },
                #[codec(index = 2u8)]
                free { index: u32 },
                #[codec(index = 3u8)]
                force_transfer {
                    new: sp_core::crypto::AccountId32,
                    index: u32,
                    freeze: bool,
                },
                #[codec(index = 4u8)]
                freeze { index: u32 },
            }
        }
    }
    pub mod pallet_im_online {
        use super::*;
        pub mod pallet {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Error {
                #[codec(index = 0u8)]
                InvalidKey,
                #[codec(index = 1u8)]
                DuplicatedHeartbeat,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Event {
                #[codec(index = 0u8)]
                HeartbeatReceived {
                    authority_id: pallet_im_online::sr25519::app_sr25519::Public,
                },
                #[codec(index = 1u8)]
                AllGood,
                #[codec(index = 2u8)]
                SomeOffline {
                    offline: Vec<(
                        sp_core::crypto::AccountId32,
                        pallet_staking::Exposure<sp_core::crypto::AccountId32, u128>,
                    )>,
                },
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub enum Call {
                #[codec(index = 0u8)]
                heartbeat {
                    heartbeat: pallet_im_online::Heartbeat<u32>,
                    signature: pallet_im_online::sr25519::app_sr25519::Signature,
                },
            }
        }
        pub mod sr25519 {
            use super::*;
            pub mod app_sr25519 {
                use super::*;
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct Signature(pub sp_core::sr25519::Signature);
                #[derive(:: codec :: Encode, :: codec :: Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct Public(pub sp_core::sr25519::Public);
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Heartbeat<BlockNumber> {
            pub block_number: BlockNumber,
            pub network_state: sp_core::offchain::OpaqueNetworkState,
            pub session_index: BlockNumber,
            pub authority_index: BlockNumber,
            pub validators_len: BlockNumber,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct BoundedOpaqueNetworkState {
            pub peer_id: Vec<u8>,
            pub external_addresses: Vec<Vec<u8>>,
        }
    }
    pub mod pallet_staking {
        use super::*;
        pub mod slashing {
            use super::*;
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct SpanRecord<Balance> {
                pub slashed: Balance,
                pub paid_out: Balance,
            }
            #[derive(:: codec :: Encode, :: codec :: Decode)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct SlashingSpans {
                pub span_index: u32,
                pub last_start: u32,
                pub last_nonzero_slash: u32,
                pub prior: Vec<u32>,
            }
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct PermissionedIdentityPrefs {
            pub intended_count: u32,
            pub running_count: u32,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum SlashingSwitch {
            #[codec(index = 0u8)]
            Validator,
            #[codec(index = 1u8)]
            ValidatorAndNominator,
            #[codec(index = 2u8)]
            None,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ActiveEraInfo {
            pub index: u32,
            pub start: Option<u64>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RewardDestination<AccountId> {
            #[codec(index = 0u8)]
            Staked,
            #[codec(index = 1u8)]
            Stash,
            #[codec(index = 2u8)]
            Controller,
            #[codec(index = 3u8)]
            Account(AccountId),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ValidatorPrefs {
            pub commission: ::codec::Compact<sp_arithmetic::per_things::Perbill>,
            pub blocked: bool,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Exposure<AccountId, Balance> {
            pub total: ::codec::Compact<Balance>,
            pub own: ::codec::Compact<Balance>,
            pub others: Vec<pallet_staking::IndividualExposure<AccountId, Balance>>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct UnappliedSlash<AccountId, Balance> {
            pub validator: AccountId,
            pub own: Balance,
            pub others: Vec<(AccountId, Balance)>,
            pub reporters: Vec<AccountId>,
            pub payout: Balance,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct EraRewardPoints<AccountId> {
            pub total: u32,
            pub individual: std::collections::BTreeMap<AccountId, u32>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Nominations<AccountId> {
            pub targets: Vec<AccountId>,
            pub submitted_in: u32,
            pub suppressed: bool,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct CompactAssignments {
            pub votes1: Vec<(::codec::Compact<u32>, ::codec::Compact<u16>)>,
            pub votes2: Vec<(
                ::codec::Compact<u32>,
                (
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ),
                ::codec::Compact<u16>,
            )>,
            pub votes3: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 2usize],
                ::codec::Compact<u16>,
            )>,
            pub votes4: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 3usize],
                ::codec::Compact<u16>,
            )>,
            pub votes5: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 4usize],
                ::codec::Compact<u16>,
            )>,
            pub votes6: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 5usize],
                ::codec::Compact<u16>,
            )>,
            pub votes7: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 6usize],
                ::codec::Compact<u16>,
            )>,
            pub votes8: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 7usize],
                ::codec::Compact<u16>,
            )>,
            pub votes9: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 8usize],
                ::codec::Compact<u16>,
            )>,
            pub votes10: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 9usize],
                ::codec::Compact<u16>,
            )>,
            pub votes11: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 10usize],
                ::codec::Compact<u16>,
            )>,
            pub votes12: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 11usize],
                ::codec::Compact<u16>,
            )>,
            pub votes13: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 12usize],
                ::codec::Compact<u16>,
            )>,
            pub votes14: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 13usize],
                ::codec::Compact<u16>,
            )>,
            pub votes15: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 14usize],
                ::codec::Compact<u16>,
            )>,
            pub votes16: Vec<(
                ::codec::Compact<u32>,
                [(
                    ::codec::Compact<u16>,
                    ::codec::Compact<sp_arithmetic::per_things::PerU16>,
                ); 15usize],
                ::codec::Compact<u16>,
            )>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Call {
            #[codec(index = 0u8)]
            bond {
                controller: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
                payee: pallet_staking::RewardDestination<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 1u8)]
            bond_extra {
                max_additional: ::codec::Compact<u128>,
            },
            #[codec(index = 2u8)]
            unbond { value: ::codec::Compact<u128> },
            #[codec(index = 3u8)]
            withdraw_unbonded { num_slashing_spans: u32 },
            #[codec(index = 4u8)]
            validate {
                prefs: pallet_staking::ValidatorPrefs,
            },
            #[codec(index = 5u8)]
            nominate {
                targets: Vec<sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>>,
            },
            #[codec(index = 6u8)]
            chill,
            #[codec(index = 7u8)]
            set_payee {
                payee: pallet_staking::RewardDestination<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 8u8)]
            set_controller {
                controller: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
            },
            #[codec(index = 9u8)]
            set_validator_count { new: ::codec::Compact<u32> },
            #[codec(index = 10u8)]
            increase_validator_count { additional: ::codec::Compact<u32> },
            #[codec(index = 11u8)]
            scale_validator_count {
                factor: sp_arithmetic::per_things::Percent,
            },
            #[codec(index = 12u8)]
            add_permissioned_validator {
                identity: polymesh_primitives::identity_id::IdentityId,
                intended_count: Option<u32>,
            },
            #[codec(index = 13u8)]
            remove_permissioned_validator {
                identity: polymesh_primitives::identity_id::IdentityId,
            },
            #[codec(index = 14u8)]
            validate_cdd_expiry_nominators {
                targets: Vec<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 15u8)]
            set_commission_cap {
                new_cap: sp_arithmetic::per_things::Perbill,
            },
            #[codec(index = 16u8)]
            set_min_bond_threshold { new_value: u128 },
            #[codec(index = 17u8)]
            force_no_eras,
            #[codec(index = 18u8)]
            force_new_era,
            #[codec(index = 19u8)]
            set_invulnerables {
                invulnerables: Vec<sp_core::crypto::AccountId32>,
            },
            #[codec(index = 20u8)]
            force_unstake {
                stash: sp_core::crypto::AccountId32,
                num_slashing_spans: u32,
            },
            #[codec(index = 21u8)]
            force_new_era_always,
            #[codec(index = 22u8)]
            cancel_deferred_slash { era: u32, slash_indices: Vec<u32> },
            #[codec(index = 23u8)]
            payout_stakers {
                validator_stash: sp_core::crypto::AccountId32,
                era: u32,
            },
            #[codec(index = 24u8)]
            rebond { value: ::codec::Compact<u128> },
            #[codec(index = 25u8)]
            set_history_depth {
                new_history_depth: ::codec::Compact<u32>,
                _era_items_deleted: ::codec::Compact<u32>,
            },
            #[codec(index = 26u8)]
            reap_stash {
                stash: sp_core::crypto::AccountId32,
                num_slashing_spans: u32,
            },
            #[codec(index = 27u8)]
            submit_election_solution {
                winners: Vec<u16>,
                compact: pallet_staking::CompactAssignments,
                score: sp_npos_elections::ElectionScore,
                era: u32,
                size: pallet_staking::ElectionSize,
            },
            #[codec(index = 28u8)]
            submit_election_solution_unsigned {
                winners: Vec<u16>,
                compact: pallet_staking::CompactAssignments,
                score: sp_npos_elections::ElectionScore,
                era: u32,
                size: pallet_staking::ElectionSize,
            },
            #[codec(index = 29u8)]
            payout_stakers_by_system {
                validator_stash: sp_core::crypto::AccountId32,
                era: u32,
            },
            #[codec(index = 30u8)]
            change_slashing_allowed_for {
                slashing_switch: pallet_staking::SlashingSwitch,
            },
            #[codec(index = 31u8)]
            update_permissioned_validator_intended_count {
                identity: polymesh_primitives::identity_id::IdentityId,
                new_intended_count: u32,
            },
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct IndividualExposure<AccountId, Balance> {
            pub who: AccountId,
            pub value: ::codec::Compact<Balance>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct UnlockChunk<Balance> {
            pub value: ::codec::Compact<Balance>,
            pub era: ::codec::Compact<u32>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RawEvent<Balance, AccountId> {
            #[codec(index = 0u8)]
            EraPayout(u32, Balance, Balance),
            #[codec(index = 1u8)]
            Reward(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Balance,
            ),
            #[codec(index = 2u8)]
            Slash(AccountId, Balance),
            #[codec(index = 3u8)]
            OldSlashingReportDiscarded(u32),
            #[codec(index = 4u8)]
            StakingElection(pallet_staking::ElectionCompute),
            #[codec(index = 5u8)]
            SolutionStored(pallet_staking::ElectionCompute),
            #[codec(index = 6u8)]
            Bonded(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Balance,
            ),
            #[codec(index = 7u8)]
            Unbonded(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Balance,
            ),
            #[codec(index = 8u8)]
            Nominated(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Vec<AccountId>,
            ),
            #[codec(index = 9u8)]
            Withdrawn(AccountId, Balance),
            #[codec(index = 10u8)]
            PermissionedIdentityAdded(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::IdentityId,
            ),
            #[codec(index = 11u8)]
            PermissionedIdentityRemoved(
                polymesh_primitives::identity_id::IdentityId,
                polymesh_primitives::identity_id::IdentityId,
            ),
            #[codec(index = 12u8)]
            InvalidatedNominators(
                polymesh_primitives::identity_id::IdentityId,
                AccountId,
                Vec<AccountId>,
            ),
            #[codec(index = 13u8)]
            CommissionCapUpdated(
                polymesh_primitives::identity_id::IdentityId,
                sp_arithmetic::per_things::Perbill,
                sp_arithmetic::per_things::Perbill,
            ),
            #[codec(index = 14u8)]
            MinimumBondThresholdUpdated(
                Option<polymesh_primitives::identity_id::IdentityId>,
                Balance,
            ),
            #[codec(index = 15u8)]
            RewardPaymentSchedulingInterrupted(AccountId, u32, sp_runtime::DispatchError),
            #[codec(index = 16u8)]
            SlashingAllowedForChanged(pallet_staking::SlashingSwitch),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Forcing {
            #[codec(index = 0u8)]
            NotForcing,
            #[codec(index = 1u8)]
            ForceNew,
            #[codec(index = 2u8)]
            ForceNone,
            #[codec(index = 3u8)]
            ForceAlways,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Releases {
            #[codec(index = 0u8)]
            V1_0_0Ancient,
            #[codec(index = 1u8)]
            V2_0_0,
            #[codec(index = 2u8)]
            V3_0_0,
            #[codec(index = 3u8)]
            V4_0_0,
            #[codec(index = 4u8)]
            V5_0_0,
            #[codec(index = 5u8)]
            V6_0_0,
            #[codec(index = 6u8)]
            V6_0_1,
            #[codec(index = 7u8)]
            V7_0_0,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum ElectionCompute {
            #[codec(index = 0u8)]
            OnChain,
            #[codec(index = 1u8)]
            Signed,
            #[codec(index = 2u8)]
            Unsigned,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ElectionSize {
            pub validators: ::codec::Compact<u16>,
            pub nominators: ::codec::Compact<u32>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum ElectionStatus<BlockNumber> {
            #[codec(index = 0u8)]
            Closed,
            #[codec(index = 1u8)]
            Open(BlockNumber),
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum Error {
            #[codec(index = 0u8)]
            NotController,
            #[codec(index = 1u8)]
            NotStash,
            #[codec(index = 2u8)]
            AlreadyBonded,
            #[codec(index = 3u8)]
            AlreadyPaired,
            #[codec(index = 4u8)]
            EmptyTargets,
            #[codec(index = 5u8)]
            InvalidSlashIndex,
            #[codec(index = 6u8)]
            InsufficientValue,
            #[codec(index = 7u8)]
            NoMoreChunks,
            #[codec(index = 8u8)]
            NoUnlockChunk,
            #[codec(index = 9u8)]
            FundedTarget,
            #[codec(index = 10u8)]
            InvalidEraToReward,
            #[codec(index = 11u8)]
            NotSortedAndUnique,
            #[codec(index = 12u8)]
            AlreadyClaimed,
            #[codec(index = 13u8)]
            OffchainElectionEarlySubmission,
            #[codec(index = 14u8)]
            OffchainElectionWeakSubmission,
            #[codec(index = 15u8)]
            SnapshotUnavailable,
            #[codec(index = 16u8)]
            OffchainElectionBogusWinnerCount,
            #[codec(index = 17u8)]
            OffchainElectionBogusWinner,
            #[codec(index = 18u8)]
            OffchainElectionBogusCompact,
            #[codec(index = 19u8)]
            OffchainElectionBogusNominator,
            #[codec(index = 20u8)]
            OffchainElectionBogusNomination,
            #[codec(index = 21u8)]
            OffchainElectionSlashedNomination,
            #[codec(index = 22u8)]
            OffchainElectionBogusSelfVote,
            #[codec(index = 23u8)]
            OffchainElectionBogusEdge,
            #[codec(index = 24u8)]
            OffchainElectionBogusScore,
            #[codec(index = 25u8)]
            OffchainElectionBogusElectionSize,
            #[codec(index = 26u8)]
            CallNotAllowed,
            #[codec(index = 27u8)]
            IncorrectSlashingSpans,
            #[codec(index = 28u8)]
            AlreadyExists,
            #[codec(index = 29u8)]
            NotExists,
            #[codec(index = 30u8)]
            NoChange,
            #[codec(index = 31u8)]
            InvalidValidatorIdentity,
            #[codec(index = 32u8)]
            InvalidValidatorCommission,
            #[codec(index = 33u8)]
            StashIdentityDoesNotExist,
            #[codec(index = 34u8)]
            StashIdentityNotPermissioned,
            #[codec(index = 35u8)]
            StashIdentityNotCDDed,
            #[codec(index = 36u8)]
            HitIntendedValidatorCount,
            #[codec(index = 37u8)]
            IntendedCountIsExceedingConsensusLimit,
            #[codec(index = 38u8)]
            BondTooSmall,
            #[codec(index = 39u8)]
            BadState,
            #[codec(index = 40u8)]
            TooManyTargets,
            #[codec(index = 41u8)]
            BadTarget,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct StakingLedger<AccountId, Balance> {
            pub stash: AccountId,
            pub total: ::codec::Compact<Balance>,
            pub active: ::codec::Compact<Balance>,
            pub unlocking: Vec<pallet_staking::UnlockChunk<Balance>>,
            pub claimed_rewards: Vec<u32>,
        }
        #[derive(:: codec :: Encode, :: codec :: Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct ElectionResult<AccountId, Balance> {
            pub elected_stashes: Vec<AccountId>,
            pub exposures: Vec<(AccountId, pallet_staking::Exposure<AccountId, Balance>)>,
            pub compute: pallet_staking::ElectionCompute,
        }
    }
}
#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::types;
    use super::types::*;
    pub mod system {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn fill_block(
                &self,
                ratio: sp_arithmetic::per_things::Perbill,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::fill_block { ratio },
                )
            }
            pub fn remark(&self, remark: Vec<u8>) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::remark { remark },
                )
            }
            pub fn set_heap_pages(&self, pages: u64) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::set_heap_pages { pages },
                )
            }
            pub fn set_code(&self, code: Vec<u8>) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::set_code { code },
                )
            }
            pub fn set_code_without_checks(
                &self,
                code: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::set_code_without_checks { code },
                )
            }
            pub fn set_storage(
                &self,
                items: Vec<(Vec<u8>, Vec<u8>)>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::set_storage { items },
                )
            }
            pub fn kill_storage(
                &self,
                keys: Vec<Vec<u8>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::kill_storage { keys },
                )
            }
            pub fn kill_prefix(
                &self,
                prefix: Vec<u8>,
                subkeys: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::kill_prefix { prefix, subkeys },
                )
            }
            pub fn remark_with_event(
                &self,
                remark: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::System(
                    types::frame_system::pallet::Call::remark_with_event { remark },
                )
            }
        }
    }
    pub mod babe {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn report_equivocation(
                &self,
                equivocation_proof: sp_consensus_slots::EquivocationProof<
                    sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>,
                    sp_consensus_babe::app::Public,
                >,
                key_owner_proof: sp_session::MembershipProof,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Babe(
                    types::pallet_babe::pallet::Call::report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    },
                )
            }
            pub fn report_equivocation_unsigned(
                &self,
                equivocation_proof: sp_consensus_slots::EquivocationProof<
                    sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>,
                    sp_consensus_babe::app::Public,
                >,
                key_owner_proof: sp_session::MembershipProof,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Babe(
                    types::pallet_babe::pallet::Call::report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    },
                )
            }
            pub fn plan_config_change(
                &self,
                config: sp_consensus_babe::digests::NextConfigDescriptor,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Babe(
                    types::pallet_babe::pallet::Call::plan_config_change { config },
                )
            }
        }
    }
    pub mod timestamp {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set(
                &self,
                now: ::codec::Compact<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Timestamp(
                    types::pallet_timestamp::pallet::Call::set { now },
                )
            }
        }
    }
    pub mod indices {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn claim(&self, index: u32) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Indices(
                    types::pallet_indices::pallet::Call::claim { index },
                )
            }
            pub fn transfer(
                &self,
                new: sp_core::crypto::AccountId32,
                index: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Indices(
                    types::pallet_indices::pallet::Call::transfer { new, index },
                )
            }
            pub fn free(&self, index: u32) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Indices(
                    types::pallet_indices::pallet::Call::free { index },
                )
            }
            pub fn force_transfer(
                &self,
                new: sp_core::crypto::AccountId32,
                index: u32,
                freeze: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Indices(
                    types::pallet_indices::pallet::Call::force_transfer { new, index, freeze },
                )
            }
            pub fn freeze(&self, index: u32) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Indices(
                    types::pallet_indices::pallet::Call::freeze { index },
                )
            }
        }
    }
    pub mod authorship {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_uncles(
                &self,
                new_uncles: Vec<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Authorship(
                    types::pallet_authorship::pallet::Call::set_uncles { new_uncles },
                )
            }
        }
    }
    pub mod balances {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn transfer(
                &self,
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Balances(
                    types::pallet_balances::Call::transfer { dest, value },
                )
            }
            pub fn transfer_with_memo(
                &self,
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
                memo: Option<polymesh_common_utilities::traits::balances::Memo>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Balances(
                    types::pallet_balances::Call::transfer_with_memo { dest, value, memo },
                )
            }
            pub fn deposit_block_reward_reserve_balance(
                &self,
                value: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Balances(
                    types::pallet_balances::Call::deposit_block_reward_reserve_balance { value },
                )
            }
            pub fn set_balance(
                &self,
                who: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                new_free: ::codec::Compact<u128>,
                new_reserved: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Balances(
                    types::pallet_balances::Call::set_balance {
                        who,
                        new_free,
                        new_reserved,
                    },
                )
            }
            pub fn force_transfer(
                &self,
                source: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Balances(
                    types::pallet_balances::Call::force_transfer {
                        source,
                        dest,
                        value,
                    },
                )
            }
            pub fn burn_account_balance(
                &self,
                amount: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Balances(
                    types::pallet_balances::Call::burn_account_balance { amount },
                )
            }
        }
    }
    pub mod transaction_payment {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod identity {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn cdd_register_did(
                &self,
                target_account: sp_core::crypto::AccountId32,
                secondary_keys: Vec<
                    polymesh_primitives::secondary_key::SecondaryKey<sp_core::crypto::AccountId32>,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::cdd_register_did {
                        target_account,
                        secondary_keys,
                    },
                )
            }
            pub fn invalidate_cdd_claims(
                &self,
                cdd: polymesh_primitives::identity_id::IdentityId,
                disable_from: u64,
                expiry: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::invalidate_cdd_claims {
                        cdd,
                        disable_from,
                        expiry,
                    },
                )
            }
            pub fn remove_secondary_keys_old(
                &self,
                keys_to_remove: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::remove_secondary_keys_old { keys_to_remove },
                )
            }
            pub fn accept_primary_key(
                &self,
                rotation_auth_id: u64,
                optional_cdd_auth_id: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::accept_primary_key {
                        rotation_auth_id,
                        optional_cdd_auth_id,
                    },
                )
            }
            pub fn change_cdd_requirement_for_mk_rotation(
                &self,
                auth_required: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::change_cdd_requirement_for_mk_rotation {
                        auth_required,
                    },
                )
            }
            pub fn join_identity_as_key(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::join_identity_as_key { auth_id },
                )
            }
            pub fn leave_identity_as_key(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::leave_identity_as_key,
                )
            }
            pub fn add_claim(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
                claim: polymesh_primitives::identity_claim::Claim,
                expiry: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::add_claim {
                        target,
                        claim,
                        expiry,
                    },
                )
            }
            pub fn revoke_claim(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
                claim: polymesh_primitives::identity_claim::Claim,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::revoke_claim { target, claim },
                )
            }
            pub fn set_permission_to_signer(
                &self,
                key: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                perms: polymesh_primitives::secondary_key::Permissions,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::set_permission_to_signer { key, perms },
                )
            }
            pub fn placeholder_legacy_set_permission_to_signer(
                &self,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::placeholder_legacy_set_permission_to_signer,
                )
            }
            pub fn freeze_secondary_keys(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::freeze_secondary_keys,
                )
            }
            pub fn unfreeze_secondary_keys(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::unfreeze_secondary_keys,
                )
            }
            pub fn add_authorization(
                &self,
                target: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                data: polymesh_primitives::authorization::AuthorizationData<
                    sp_core::crypto::AccountId32,
                >,
                expiry: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::add_authorization {
                        target,
                        data,
                        expiry,
                    },
                )
            }
            pub fn remove_authorization(
                &self,
                target: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                auth_id: u64,
                _auth_issuer_pays: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::remove_authorization {
                        target,
                        auth_id,
                        _auth_issuer_pays,
                    },
                )
            }
            pub fn add_secondary_keys_with_authorization_old(
                &self,
                additional_keys: Vec<
                    polymesh_common_utilities::traits::identity::SecondaryKeyWithAuthV1<
                        sp_core::crypto::AccountId32,
                    >,
                >,
                expires_at: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::add_secondary_keys_with_authorization_old {
                        additional_keys,
                        expires_at,
                    },
                )
            }
            pub fn add_investor_uniqueness_claim(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
                claim: polymesh_primitives::identity_claim::Claim,
                proof: [u8; 64usize],
                expiry: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::add_investor_uniqueness_claim {
                        target,
                        claim,
                        proof,
                        expiry,
                    },
                )
            }
            pub fn gc_add_cdd_claim(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::gc_add_cdd_claim { target },
                )
            }
            pub fn gc_revoke_cdd_claim(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::gc_revoke_cdd_claim { target },
                )
            }
            pub fn add_investor_uniqueness_claim_v2(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
                scope: polymesh_primitives::identity_claim::Scope,
                claim: polymesh_primitives::identity_claim::Claim,
                proof: confidential_identity::claim_proofs::ScopeClaimProof,
                expiry: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::add_investor_uniqueness_claim_v2 {
                        target,
                        scope,
                        claim,
                        proof,
                        expiry,
                    },
                )
            }
            pub fn revoke_claim_by_index(
                &self,
                target: polymesh_primitives::identity_id::IdentityId,
                claim_type: polymesh_primitives::identity_claim::ClaimType,
                scope: Option<polymesh_primitives::identity_claim::Scope>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::revoke_claim_by_index {
                        target,
                        claim_type,
                        scope,
                    },
                )
            }
            pub fn rotate_primary_key_to_secondary(
                &self,
                auth_id: u64,
                optional_cdd_auth_id: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::rotate_primary_key_to_secondary {
                        auth_id,
                        optional_cdd_auth_id,
                    },
                )
            }
            pub fn add_secondary_keys_with_authorization(
                &self,
                additional_keys: Vec<
                    polymesh_common_utilities::traits::identity::SecondaryKeyWithAuth<
                        sp_core::crypto::AccountId32,
                    >,
                >,
                expires_at: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::add_secondary_keys_with_authorization {
                        additional_keys,
                        expires_at,
                    },
                )
            }
            pub fn set_secondary_key_permissions(
                &self,
                key: sp_core::crypto::AccountId32,
                perms: polymesh_primitives::secondary_key::Permissions,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::set_secondary_key_permissions { key, perms },
                )
            }
            pub fn remove_secondary_keys(
                &self,
                keys_to_remove: Vec<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Identity(
                    types::pallet_identity::Call::remove_secondary_keys { keys_to_remove },
                )
            }
        }
    }
    pub mod cdd_service_providers {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_active_members_limit(
                &self,
                limit: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::set_active_members_limit { limit },
                )
            }
            pub fn disable_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
                at: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::disable_member { who, expiry, at },
                )
            }
            pub fn add_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::add_member { who },
                )
            }
            pub fn remove_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::remove_member { who },
                )
            }
            pub fn swap_member(
                &self,
                remove: polymesh_primitives::identity_id::IdentityId,
                add: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::swap_member { remove, add },
                )
            }
            pub fn reset_members(
                &self,
                members: Vec<polymesh_primitives::identity_id::IdentityId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::reset_members { members },
                )
            }
            pub fn abdicate_membership(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CddServiceProviders(
                    types::pallet_group::Call::abdicate_membership,
                )
            }
        }
    }
    pub mod polymesh_committee {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_vote_threshold(
                &self,
                n: u32,
                d: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshCommittee(
                    types::pallet_committee::Call::set_vote_threshold { n, d },
                )
            }
            pub fn set_release_coordinator(
                &self,
                id: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshCommittee(
                    types::pallet_committee::Call::set_release_coordinator { id },
                )
            }
            pub fn set_expires_after(
                &self,
                expiry: polymesh_common_utilities::MaybeBlock<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshCommittee(
                    types::pallet_committee::Call::set_expires_after { expiry },
                )
            }
            pub fn vote_or_propose(
                &self,
                approve: bool,
                call: polymesh_runtime_develop::runtime::Call,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshCommittee(
                    types::pallet_committee::Call::vote_or_propose {
                        approve,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn vote(
                &self,
                proposal: primitive_types::H256,
                index: u32,
                approve: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshCommittee(
                    types::pallet_committee::Call::vote {
                        proposal,
                        index,
                        approve,
                    },
                )
            }
        }
    }
    pub mod committee_membership {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_active_members_limit(
                &self,
                limit: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::set_active_members_limit { limit },
                )
            }
            pub fn disable_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
                at: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::disable_member { who, expiry, at },
                )
            }
            pub fn add_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::add_member { who },
                )
            }
            pub fn remove_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::remove_member { who },
                )
            }
            pub fn swap_member(
                &self,
                remove: polymesh_primitives::identity_id::IdentityId,
                add: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::swap_member { remove, add },
                )
            }
            pub fn reset_members(
                &self,
                members: Vec<polymesh_primitives::identity_id::IdentityId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::reset_members { members },
                )
            }
            pub fn abdicate_membership(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CommitteeMembership(
                    types::pallet_group::Call::abdicate_membership,
                )
            }
        }
    }
    pub mod technical_committee {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_vote_threshold(
                &self,
                n: u32,
                d: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommittee(
                    types::pallet_committee::Call::set_vote_threshold { n, d },
                )
            }
            pub fn set_release_coordinator(
                &self,
                id: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommittee(
                    types::pallet_committee::Call::set_release_coordinator { id },
                )
            }
            pub fn set_expires_after(
                &self,
                expiry: polymesh_common_utilities::MaybeBlock<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommittee(
                    types::pallet_committee::Call::set_expires_after { expiry },
                )
            }
            pub fn vote_or_propose(
                &self,
                approve: bool,
                call: polymesh_runtime_develop::runtime::Call,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommittee(
                    types::pallet_committee::Call::vote_or_propose {
                        approve,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn vote(
                &self,
                proposal: primitive_types::H256,
                index: u32,
                approve: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommittee(
                    types::pallet_committee::Call::vote {
                        proposal,
                        index,
                        approve,
                    },
                )
            }
        }
    }
    pub mod technical_committee_membership {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_active_members_limit(
                &self,
                limit: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::set_active_members_limit { limit },
                )
            }
            pub fn disable_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
                at: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::disable_member { who, expiry, at },
                )
            }
            pub fn add_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::add_member { who },
                )
            }
            pub fn remove_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::remove_member { who },
                )
            }
            pub fn swap_member(
                &self,
                remove: polymesh_primitives::identity_id::IdentityId,
                add: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::swap_member { remove, add },
                )
            }
            pub fn reset_members(
                &self,
                members: Vec<polymesh_primitives::identity_id::IdentityId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::reset_members { members },
                )
            }
            pub fn abdicate_membership(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TechnicalCommitteeMembership(
                    types::pallet_group::Call::abdicate_membership,
                )
            }
        }
    }
    pub mod upgrade_committee {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_vote_threshold(
                &self,
                n: u32,
                d: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommittee(
                    types::pallet_committee::Call::set_vote_threshold { n, d },
                )
            }
            pub fn set_release_coordinator(
                &self,
                id: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommittee(
                    types::pallet_committee::Call::set_release_coordinator { id },
                )
            }
            pub fn set_expires_after(
                &self,
                expiry: polymesh_common_utilities::MaybeBlock<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommittee(
                    types::pallet_committee::Call::set_expires_after { expiry },
                )
            }
            pub fn vote_or_propose(
                &self,
                approve: bool,
                call: polymesh_runtime_develop::runtime::Call,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommittee(
                    types::pallet_committee::Call::vote_or_propose {
                        approve,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn vote(
                &self,
                proposal: primitive_types::H256,
                index: u32,
                approve: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommittee(
                    types::pallet_committee::Call::vote {
                        proposal,
                        index,
                        approve,
                    },
                )
            }
        }
    }
    pub mod upgrade_committee_membership {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_active_members_limit(
                &self,
                limit: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::set_active_members_limit { limit },
                )
            }
            pub fn disable_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
                at: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::disable_member { who, expiry, at },
                )
            }
            pub fn add_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::add_member { who },
                )
            }
            pub fn remove_member(
                &self,
                who: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::remove_member { who },
                )
            }
            pub fn swap_member(
                &self,
                remove: polymesh_primitives::identity_id::IdentityId,
                add: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::swap_member { remove, add },
                )
            }
            pub fn reset_members(
                &self,
                members: Vec<polymesh_primitives::identity_id::IdentityId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::reset_members { members },
                )
            }
            pub fn abdicate_membership(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::UpgradeCommitteeMembership(
                    types::pallet_group::Call::abdicate_membership,
                )
            }
        }
    }
    pub mod multi_sig {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn create_multisig(
                &self,
                signers: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
                sigs_required: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::create_multisig {
                        signers,
                        sigs_required,
                    },
                )
            }
            pub fn create_or_approve_proposal_as_identity(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal: polymesh_runtime_develop::runtime::Call,
                expiry: Option<u64>,
                auto_close: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::create_or_approve_proposal_as_identity {
                        multisig,
                        proposal: ::std::boxed::Box::new(proposal),
                        expiry,
                        auto_close,
                    },
                )
            }
            pub fn create_or_approve_proposal_as_key(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal: polymesh_runtime_develop::runtime::Call,
                expiry: Option<u64>,
                auto_close: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::create_or_approve_proposal_as_key {
                        multisig,
                        proposal: ::std::boxed::Box::new(proposal),
                        expiry,
                        auto_close,
                    },
                )
            }
            pub fn create_proposal_as_identity(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal: polymesh_runtime_develop::runtime::Call,
                expiry: Option<u64>,
                auto_close: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::create_proposal_as_identity {
                        multisig,
                        proposal: ::std::boxed::Box::new(proposal),
                        expiry,
                        auto_close,
                    },
                )
            }
            pub fn create_proposal_as_key(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal: polymesh_runtime_develop::runtime::Call,
                expiry: Option<u64>,
                auto_close: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::create_proposal_as_key {
                        multisig,
                        proposal: ::std::boxed::Box::new(proposal),
                        expiry,
                        auto_close,
                    },
                )
            }
            pub fn approve_as_identity(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::approve_as_identity {
                        multisig,
                        proposal_id,
                    },
                )
            }
            pub fn approve_as_key(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::approve_as_key {
                        multisig,
                        proposal_id,
                    },
                )
            }
            pub fn reject_as_identity(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::reject_as_identity {
                        multisig,
                        proposal_id,
                    },
                )
            }
            pub fn reject_as_key(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::reject_as_key {
                        multisig,
                        proposal_id,
                    },
                )
            }
            pub fn accept_multisig_signer_as_identity(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::accept_multisig_signer_as_identity { auth_id },
                )
            }
            pub fn accept_multisig_signer_as_key(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::accept_multisig_signer_as_key { auth_id },
                )
            }
            pub fn add_multisig_signer(
                &self,
                signer: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::add_multisig_signer { signer },
                )
            }
            pub fn remove_multisig_signer(
                &self,
                signer: polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::remove_multisig_signer { signer },
                )
            }
            pub fn add_multisig_signers_via_creator(
                &self,
                multisig: sp_core::crypto::AccountId32,
                signers: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::add_multisig_signers_via_creator {
                        multisig,
                        signers,
                    },
                )
            }
            pub fn remove_multisig_signers_via_creator(
                &self,
                multisig: sp_core::crypto::AccountId32,
                signers: Vec<
                    polymesh_primitives::secondary_key::Signatory<sp_core::crypto::AccountId32>,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::remove_multisig_signers_via_creator {
                        multisig,
                        signers,
                    },
                )
            }
            pub fn change_sigs_required(
                &self,
                sigs_required: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::change_sigs_required { sigs_required },
                )
            }
            pub fn make_multisig_secondary(
                &self,
                multisig: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::make_multisig_secondary { multisig },
                )
            }
            pub fn make_multisig_primary(
                &self,
                multisig: sp_core::crypto::AccountId32,
                optional_cdd_auth_id: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::make_multisig_primary {
                        multisig,
                        optional_cdd_auth_id,
                    },
                )
            }
            pub fn execute_scheduled_proposal(
                &self,
                multisig: sp_core::crypto::AccountId32,
                proposal_id: u64,
                multisig_did: polymesh_primitives::identity_id::IdentityId,
                _proposal_weight: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::MultiSig(
                    types::pallet_multisig::Call::execute_scheduled_proposal {
                        multisig,
                        proposal_id,
                        multisig_did,
                        _proposal_weight,
                    },
                )
            }
        }
    }
    pub mod bridge {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn change_controller(
                &self,
                controller: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::change_controller { controller },
                )
            }
            pub fn change_admin(
                &self,
                admin: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::change_admin { admin },
                )
            }
            pub fn change_timelock(
                &self,
                timelock: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::change_timelock { timelock },
                )
            }
            pub fn freeze(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::freeze,
                )
            }
            pub fn unfreeze(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::unfreeze,
                )
            }
            pub fn change_bridge_limit(
                &self,
                amount: u128,
                duration: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::change_bridge_limit { amount, duration },
                )
            }
            pub fn change_bridge_exempted(
                &self,
                exempted: Vec<(polymesh_primitives::identity_id::IdentityId, bool)>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::change_bridge_exempted { exempted },
                )
            }
            pub fn force_handle_bridge_tx(
                &self,
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::force_handle_bridge_tx { bridge_tx },
                )
            }
            pub fn batch_propose_bridge_tx(
                &self,
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::batch_propose_bridge_tx { bridge_txs },
                )
            }
            pub fn propose_bridge_tx(
                &self,
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::propose_bridge_tx { bridge_tx },
                )
            }
            pub fn handle_bridge_tx(
                &self,
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::handle_bridge_tx { bridge_tx },
                )
            }
            pub fn freeze_txs(
                &self,
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::freeze_txs { bridge_txs },
                )
            }
            pub fn unfreeze_txs(
                &self,
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::unfreeze_txs { bridge_txs },
                )
            }
            pub fn handle_scheduled_bridge_tx(
                &self,
                bridge_tx: pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::handle_scheduled_bridge_tx { bridge_tx },
                )
            }
            pub fn add_freeze_admin(
                &self,
                freeze_admin: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::add_freeze_admin { freeze_admin },
                )
            }
            pub fn remove_freeze_admin(
                &self,
                freeze_admin: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::remove_freeze_admin { freeze_admin },
                )
            }
            pub fn remove_txs(
                &self,
                bridge_txs: Vec<pallet_bridge::BridgeTx<sp_core::crypto::AccountId32>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Bridge(
                    types::pallet_bridge::Call::remove_txs { bridge_txs },
                )
            }
        }
    }
    pub mod staking {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn bond(
                &self,
                controller: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
                payee: pallet_staking::RewardDestination<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::bond {
                        controller,
                        value,
                        payee,
                    },
                )
            }
            pub fn bond_extra(
                &self,
                max_additional: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::bond_extra { max_additional },
                )
            }
            pub fn unbond(
                &self,
                value: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::unbond { value },
                )
            }
            pub fn withdraw_unbonded(
                &self,
                num_slashing_spans: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::withdraw_unbonded { num_slashing_spans },
                )
            }
            pub fn validate(
                &self,
                prefs: pallet_staking::ValidatorPrefs,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::validate { prefs },
                )
            }
            pub fn nominate(
                &self,
                targets: Vec<sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::nominate { targets },
                )
            }
            pub fn chill(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::chill,
                )
            }
            pub fn set_payee(
                &self,
                payee: pallet_staking::RewardDestination<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_payee { payee },
                )
            }
            pub fn set_controller(
                &self,
                controller: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_controller { controller },
                )
            }
            pub fn set_validator_count(
                &self,
                new: ::codec::Compact<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_validator_count { new },
                )
            }
            pub fn increase_validator_count(
                &self,
                additional: ::codec::Compact<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::increase_validator_count { additional },
                )
            }
            pub fn scale_validator_count(
                &self,
                factor: sp_arithmetic::per_things::Percent,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::scale_validator_count { factor },
                )
            }
            pub fn add_permissioned_validator(
                &self,
                identity: polymesh_primitives::identity_id::IdentityId,
                intended_count: Option<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::add_permissioned_validator {
                        identity,
                        intended_count,
                    },
                )
            }
            pub fn remove_permissioned_validator(
                &self,
                identity: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::remove_permissioned_validator { identity },
                )
            }
            pub fn validate_cdd_expiry_nominators(
                &self,
                targets: Vec<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::validate_cdd_expiry_nominators { targets },
                )
            }
            pub fn set_commission_cap(
                &self,
                new_cap: sp_arithmetic::per_things::Perbill,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_commission_cap { new_cap },
                )
            }
            pub fn set_min_bond_threshold(
                &self,
                new_value: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_min_bond_threshold { new_value },
                )
            }
            pub fn force_no_eras(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::force_no_eras,
                )
            }
            pub fn force_new_era(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::force_new_era,
                )
            }
            pub fn set_invulnerables(
                &self,
                invulnerables: Vec<sp_core::crypto::AccountId32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_invulnerables { invulnerables },
                )
            }
            pub fn force_unstake(
                &self,
                stash: sp_core::crypto::AccountId32,
                num_slashing_spans: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::force_unstake {
                        stash,
                        num_slashing_spans,
                    },
                )
            }
            pub fn force_new_era_always(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::force_new_era_always,
                )
            }
            pub fn cancel_deferred_slash(
                &self,
                era: u32,
                slash_indices: Vec<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::cancel_deferred_slash { era, slash_indices },
                )
            }
            pub fn payout_stakers(
                &self,
                validator_stash: sp_core::crypto::AccountId32,
                era: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::payout_stakers {
                        validator_stash,
                        era,
                    },
                )
            }
            pub fn rebond(
                &self,
                value: ::codec::Compact<u128>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::rebond { value },
                )
            }
            pub fn set_history_depth(
                &self,
                new_history_depth: ::codec::Compact<u32>,
                _era_items_deleted: ::codec::Compact<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::set_history_depth {
                        new_history_depth,
                        _era_items_deleted,
                    },
                )
            }
            pub fn reap_stash(
                &self,
                stash: sp_core::crypto::AccountId32,
                num_slashing_spans: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::reap_stash {
                        stash,
                        num_slashing_spans,
                    },
                )
            }
            pub fn submit_election_solution(
                &self,
                winners: Vec<u16>,
                compact: pallet_staking::CompactAssignments,
                score: sp_npos_elections::ElectionScore,
                era: u32,
                size: pallet_staking::ElectionSize,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::submit_election_solution {
                        winners,
                        compact,
                        score,
                        era,
                        size,
                    },
                )
            }
            pub fn submit_election_solution_unsigned(
                &self,
                winners: Vec<u16>,
                compact: pallet_staking::CompactAssignments,
                score: sp_npos_elections::ElectionScore,
                era: u32,
                size: pallet_staking::ElectionSize,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::submit_election_solution_unsigned {
                        winners,
                        compact,
                        score,
                        era,
                        size,
                    },
                )
            }
            pub fn payout_stakers_by_system(
                &self,
                validator_stash: sp_core::crypto::AccountId32,
                era: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::payout_stakers_by_system {
                        validator_stash,
                        era,
                    },
                )
            }
            pub fn change_slashing_allowed_for(
                &self,
                slashing_switch: pallet_staking::SlashingSwitch,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::change_slashing_allowed_for { slashing_switch },
                )
            }
            pub fn update_permissioned_validator_intended_count(
                &self,
                identity: polymesh_primitives::identity_id::IdentityId,
                new_intended_count: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Staking(
                    types::pallet_staking::Call::update_permissioned_validator_intended_count {
                        identity,
                        new_intended_count,
                    },
                )
            }
        }
    }
    pub mod offences {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod session {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_keys(
                &self,
                keys: polymesh_runtime_develop::runtime::SessionKeys,
                proof: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Session(
                    types::pallet_session::pallet::Call::set_keys { keys, proof },
                )
            }
            pub fn purge_keys(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Session(
                    types::pallet_session::pallet::Call::purge_keys,
                )
            }
        }
    }
    pub mod authority_discovery {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod grandpa {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn report_equivocation(
                &self,
                equivocation_proof: sp_finality_grandpa::EquivocationProof<
                    primitive_types::H256,
                    u32,
                >,
                key_owner_proof: sp_session::MembershipProof,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Grandpa(
                    types::pallet_grandpa::pallet::Call::report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    },
                )
            }
            pub fn report_equivocation_unsigned(
                &self,
                equivocation_proof: sp_finality_grandpa::EquivocationProof<
                    primitive_types::H256,
                    u32,
                >,
                key_owner_proof: sp_session::MembershipProof,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Grandpa(
                    types::pallet_grandpa::pallet::Call::report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    },
                )
            }
            pub fn note_stalled(
                &self,
                delay: u32,
                best_finalized_block_number: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Grandpa(
                    types::pallet_grandpa::pallet::Call::note_stalled {
                        delay,
                        best_finalized_block_number,
                    },
                )
            }
        }
    }
    pub mod historical {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod im_online {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn heartbeat(
                &self,
                heartbeat: pallet_im_online::Heartbeat<u32>,
                signature: pallet_im_online::sr25519::app_sr25519::Signature,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ImOnline(
                    types::pallet_im_online::pallet::Call::heartbeat {
                        heartbeat,
                        signature,
                    },
                )
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod sudo {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn sudo(
                &self,
                call: polymesh_runtime_develop::runtime::Call,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sudo(
                    types::pallet_sudo::Call::sudo {
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn sudo_unchecked_weight(
                &self,
                call: polymesh_runtime_develop::runtime::Call,
                _weight: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sudo(
                    types::pallet_sudo::Call::sudo_unchecked_weight {
                        call: ::std::boxed::Box::new(call),
                        _weight,
                    },
                )
            }
            pub fn set_key(
                &self,
                new: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sudo(
                    types::pallet_sudo::Call::set_key { new },
                )
            }
            pub fn sudo_as(
                &self,
                who: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                call: polymesh_runtime_develop::runtime::Call,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sudo(
                    types::pallet_sudo::Call::sudo_as {
                        who,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
        }
    }
    pub mod asset {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn register_ticker(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::register_ticker { ticker },
                )
            }
            pub fn accept_ticker_transfer(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::accept_ticker_transfer { auth_id },
                )
            }
            pub fn accept_asset_ownership_transfer(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::accept_asset_ownership_transfer { auth_id },
                )
            }
            pub fn create_asset(
                &self,
                name: polymesh_primitives::asset::AssetName,
                ticker: polymesh_primitives::ticker::Ticker,
                divisible: bool,
                asset_type: polymesh_primitives::asset::AssetType,
                identifiers: Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
                funding_round: Option<polymesh_primitives::asset::FundingRoundName>,
                disable_iu: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::create_asset {
                        name,
                        ticker,
                        divisible,
                        asset_type,
                        identifiers,
                        funding_round,
                        disable_iu,
                    },
                )
            }
            pub fn freeze(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::freeze { ticker },
                )
            }
            pub fn unfreeze(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::unfreeze { ticker },
                )
            }
            pub fn rename_asset(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset::AssetName,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::rename_asset { ticker, name },
                )
            }
            pub fn issue(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                amount: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::issue { ticker, amount },
                )
            }
            pub fn redeem(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                value: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::redeem { ticker, value },
                )
            }
            pub fn make_divisible(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::make_divisible { ticker },
                )
            }
            pub fn add_documents(
                &self,
                docs: Vec<polymesh_primitives::document::Document>,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::add_documents { docs, ticker },
                )
            }
            pub fn remove_documents(
                &self,
                ids: Vec<polymesh_primitives::document::DocumentId>,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::remove_documents { ids, ticker },
                )
            }
            pub fn set_funding_round(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset::FundingRoundName,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::set_funding_round { ticker, name },
                )
            }
            pub fn update_identifiers(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                identifiers: Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::update_identifiers {
                        ticker,
                        identifiers,
                    },
                )
            }
            pub fn claim_classic_ticker(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                ethereum_signature: polymesh_primitives::ethereum::EcdsaSignature,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::claim_classic_ticker {
                        ticker,
                        ethereum_signature,
                    },
                )
            }
            pub fn reserve_classic_ticker(
                &self,
                classic_ticker_import: pallet_asset::ClassicTickerImport,
                contract_did: polymesh_primitives::identity_id::IdentityId,
                config: pallet_asset::TickerRegistrationConfig<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::reserve_classic_ticker {
                        classic_ticker_import,
                        contract_did,
                        config,
                    },
                )
            }
            pub fn controller_transfer(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                value: u128,
                from_portfolio: polymesh_primitives::identity_id::PortfolioId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::controller_transfer {
                        ticker,
                        value,
                        from_portfolio,
                    },
                )
            }
            pub fn register_custom_asset_type(
                &self,
                ty: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::register_custom_asset_type { ty },
                )
            }
            pub fn create_asset_with_custom_type(
                &self,
                name: polymesh_primitives::asset::AssetName,
                ticker: polymesh_primitives::ticker::Ticker,
                divisible: bool,
                custom_asset_type: Vec<u8>,
                identifiers: Vec<polymesh_primitives::asset_identifier::AssetIdentifier>,
                funding_round: Option<polymesh_primitives::asset::FundingRoundName>,
                disable_iu: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::create_asset_with_custom_type {
                        name,
                        ticker,
                        divisible,
                        custom_asset_type,
                        identifiers,
                        funding_round,
                        disable_iu,
                    },
                )
            }
            pub fn set_asset_metadata(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                key: polymesh_primitives::asset_metadata::AssetMetadataKey,
                value: polymesh_primitives::asset_metadata::AssetMetadataValue,
                detail: Option<polymesh_primitives::asset_metadata::AssetMetadataValueDetail<u64>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::set_asset_metadata {
                        ticker,
                        key,
                        value,
                        detail,
                    },
                )
            }
            pub fn set_asset_metadata_details(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                key: polymesh_primitives::asset_metadata::AssetMetadataKey,
                detail: polymesh_primitives::asset_metadata::AssetMetadataValueDetail<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::set_asset_metadata_details {
                        ticker,
                        key,
                        detail,
                    },
                )
            }
            pub fn register_and_set_local_asset_metadata(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset_metadata::AssetMetadataName,
                spec: polymesh_primitives::asset_metadata::AssetMetadataSpec,
                value: polymesh_primitives::asset_metadata::AssetMetadataValue,
                detail: Option<polymesh_primitives::asset_metadata::AssetMetadataValueDetail<u64>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::register_and_set_local_asset_metadata {
                        ticker,
                        name,
                        spec,
                        value,
                        detail,
                    },
                )
            }
            pub fn register_asset_metadata_local_type(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                name: polymesh_primitives::asset_metadata::AssetMetadataName,
                spec: polymesh_primitives::asset_metadata::AssetMetadataSpec,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::register_asset_metadata_local_type {
                        ticker,
                        name,
                        spec,
                    },
                )
            }
            pub fn register_asset_metadata_global_type(
                &self,
                name: polymesh_primitives::asset_metadata::AssetMetadataName,
                spec: polymesh_primitives::asset_metadata::AssetMetadataSpec,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Asset(
                    types::pallet_asset::Call::register_asset_metadata_global_type { name, spec },
                )
            }
        }
    }
    pub mod capital_distribution {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn distribute(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                portfolio: Option<polymesh_primitives::identity_id::PortfolioNumber>,
                currency: polymesh_primitives::ticker::Ticker,
                per_share: u128,
                amount: u128,
                payment_at: u64,
                expires_at: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CapitalDistribution(
                    types::pallet_corporate_actions::distribution::Call::distribute {
                        ca_id,
                        portfolio,
                        currency,
                        per_share,
                        amount,
                        payment_at,
                        expires_at,
                    },
                )
            }
            pub fn claim(
                &self,
                ca_id: pallet_corporate_actions::CAId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CapitalDistribution(
                    types::pallet_corporate_actions::distribution::Call::claim { ca_id },
                )
            }
            pub fn push_benefit(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                holder: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CapitalDistribution(
                    types::pallet_corporate_actions::distribution::Call::push_benefit {
                        ca_id,
                        holder,
                    },
                )
            }
            pub fn reclaim(
                &self,
                ca_id: pallet_corporate_actions::CAId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CapitalDistribution(
                    types::pallet_corporate_actions::distribution::Call::reclaim { ca_id },
                )
            }
            pub fn remove_distribution(
                &self,
                ca_id: pallet_corporate_actions::CAId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CapitalDistribution(
                    types::pallet_corporate_actions::distribution::Call::remove_distribution {
                        ca_id,
                    },
                )
            }
        }
    }
    pub mod checkpoint {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn create_checkpoint(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Checkpoint(
                    types::pallet_asset::checkpoint::Call::create_checkpoint { ticker },
                )
            }
            pub fn set_schedules_max_complexity(
                &self,
                max_complexity: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Checkpoint(
                    types::pallet_asset::checkpoint::Call::set_schedules_max_complexity {
                        max_complexity,
                    },
                )
            }
            pub fn create_schedule(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                schedule: pallet_asset::checkpoint::ScheduleSpec,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Checkpoint(
                    types::pallet_asset::checkpoint::Call::create_schedule { ticker, schedule },
                )
            }
            pub fn remove_schedule(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                id: polymesh_common_utilities::traits::checkpoint::ScheduleId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Checkpoint(
                    types::pallet_asset::checkpoint::Call::remove_schedule { ticker, id },
                )
            }
        }
    }
    pub mod compliance_manager {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn add_compliance_requirement(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                sender_conditions: Vec<polymesh_primitives::condition::Condition>,
                receiver_conditions: Vec<polymesh_primitives::condition::Condition>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::add_compliance_requirement {
                        ticker,
                        sender_conditions,
                        receiver_conditions,
                    },
                )
            }
            pub fn remove_compliance_requirement(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                id: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::remove_compliance_requirement {
                        ticker,
                        id,
                    },
                )
            }
            pub fn replace_asset_compliance(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                asset_compliance: Vec<
                    polymesh_primitives::compliance_manager::ComplianceRequirement,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::replace_asset_compliance {
                        ticker,
                        asset_compliance,
                    },
                )
            }
            pub fn reset_asset_compliance(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::reset_asset_compliance { ticker },
                )
            }
            pub fn pause_asset_compliance(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::pause_asset_compliance { ticker },
                )
            }
            pub fn resume_asset_compliance(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::resume_asset_compliance { ticker },
                )
            }
            pub fn add_default_trusted_claim_issuer(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                issuer: polymesh_primitives::condition::TrustedIssuer,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::add_default_trusted_claim_issuer {
                        ticker,
                        issuer,
                    },
                )
            }
            pub fn remove_default_trusted_claim_issuer(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                issuer: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::remove_default_trusted_claim_issuer {
                        ticker,
                        issuer,
                    },
                )
            }
            pub fn change_compliance_requirement(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                new_req: polymesh_primitives::compliance_manager::ComplianceRequirement,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ComplianceManager(
                    types::pallet_compliance_manager::Call::change_compliance_requirement {
                        ticker,
                        new_req,
                    },
                )
            }
        }
    }
    pub mod corporate_action {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_max_details_length(
                &self,
                length: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::set_max_details_length { length },
                )
            }
            pub fn set_default_targets(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                targets: pallet_corporate_actions::TargetIdentities,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::set_default_targets { ticker, targets },
                )
            }
            pub fn set_default_withholding_tax(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                tax: sp_arithmetic::per_things::Permill,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::set_default_withholding_tax {
                        ticker,
                        tax,
                    },
                )
            }
            pub fn set_did_withholding_tax(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                taxed_did: polymesh_primitives::identity_id::IdentityId,
                tax: Option<sp_arithmetic::per_things::Permill>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::set_did_withholding_tax {
                        ticker,
                        taxed_did,
                        tax,
                    },
                )
            }
            pub fn initiate_corporate_action(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                kind: pallet_corporate_actions::CAKind,
                decl_date: u64,
                record_date: Option<pallet_corporate_actions::RecordDateSpec>,
                details: pallet_corporate_actions::CADetails,
                targets: Option<pallet_corporate_actions::TargetIdentities>,
                default_withholding_tax: Option<sp_arithmetic::per_things::Permill>,
                withholding_tax: Option<
                    Vec<(
                        polymesh_primitives::identity_id::IdentityId,
                        sp_arithmetic::per_things::Permill,
                    )>,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::initiate_corporate_action {
                        ticker,
                        kind,
                        decl_date,
                        record_date,
                        details,
                        targets,
                        default_withholding_tax,
                        withholding_tax,
                    },
                )
            }
            pub fn link_ca_doc(
                &self,
                id: pallet_corporate_actions::CAId,
                docs: Vec<polymesh_primitives::document::DocumentId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::link_ca_doc { id, docs },
                )
            }
            pub fn remove_ca(
                &self,
                ca_id: pallet_corporate_actions::CAId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::remove_ca { ca_id },
                )
            }
            pub fn change_record_date(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                record_date: Option<pallet_corporate_actions::RecordDateSpec>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateAction(
                    types::pallet_corporate_actions::Call::change_record_date {
                        ca_id,
                        record_date,
                    },
                )
            }
            pub fn initiate_corporate_action_and_distribute(
                &self,
                ca_args: pallet_corporate_actions::InitiateCorporateActionArgs,
                portfolio: Option<polymesh_primitives::identity_id::PortfolioNumber>,
                currency: polymesh_primitives::ticker::Ticker,
                per_share: u128,
                amount: u128,
                payment_at: u64,
                expires_at: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types :: polymesh_runtime_develop :: runtime :: Call :: CorporateAction (types :: pallet_corporate_actions :: Call :: initiate_corporate_action_and_distribute { ca_args , portfolio , currency , per_share , amount , payment_at , expires_at , })
            }
        }
    }
    pub mod corporate_ballot {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn attach_ballot(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                range: pallet_corporate_actions::ballot::BallotTimeRange,
                meta: pallet_corporate_actions::ballot::BallotMeta,
                rcv: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateBallot(
                    types::pallet_corporate_actions::ballot::Call::attach_ballot {
                        ca_id,
                        range,
                        meta,
                        rcv,
                    },
                )
            }
            pub fn vote(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                votes: Vec<pallet_corporate_actions::ballot::BallotVote>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateBallot(
                    types::pallet_corporate_actions::ballot::Call::vote { ca_id, votes },
                )
            }
            pub fn change_end(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                end: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateBallot(
                    types::pallet_corporate_actions::ballot::Call::change_end { ca_id, end },
                )
            }
            pub fn change_meta(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                meta: pallet_corporate_actions::ballot::BallotMeta,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateBallot(
                    types::pallet_corporate_actions::ballot::Call::change_meta { ca_id, meta },
                )
            }
            pub fn change_rcv(
                &self,
                ca_id: pallet_corporate_actions::CAId,
                rcv: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateBallot(
                    types::pallet_corporate_actions::ballot::Call::change_rcv { ca_id, rcv },
                )
            }
            pub fn remove_ballot(
                &self,
                ca_id: pallet_corporate_actions::CAId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::CorporateBallot(
                    types::pallet_corporate_actions::ballot::Call::remove_ballot { ca_id },
                )
            }
        }
    }
    pub mod permissions {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod pips {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_prune_historical_pips(
                &self,
                prune: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::set_prune_historical_pips { prune },
                )
            }
            pub fn set_min_proposal_deposit(
                &self,
                deposit: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::set_min_proposal_deposit { deposit },
                )
            }
            pub fn set_default_enactment_period(
                &self,
                duration: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::set_default_enactment_period { duration },
                )
            }
            pub fn set_pending_pip_expiry(
                &self,
                expiry: polymesh_common_utilities::MaybeBlock<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::set_pending_pip_expiry { expiry },
                )
            }
            pub fn set_max_pip_skip_count(
                &self,
                max: u8,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::set_max_pip_skip_count { max },
                )
            }
            pub fn set_active_pip_limit(
                &self,
                limit: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::set_active_pip_limit { limit },
                )
            }
            pub fn propose(
                &self,
                proposal: polymesh_runtime_develop::runtime::Call,
                deposit: u128,
                url: Option<polymesh_primitives::Url>,
                description: Option<pallet_pips::PipDescription>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::propose {
                        proposal: ::std::boxed::Box::new(proposal),
                        deposit,
                        url,
                        description,
                    },
                )
            }
            pub fn vote(
                &self,
                id: pallet_pips::PipId,
                aye_or_nay: bool,
                deposit: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::vote {
                        id,
                        aye_or_nay,
                        deposit,
                    },
                )
            }
            pub fn approve_committee_proposal(
                &self,
                id: pallet_pips::PipId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::approve_committee_proposal { id },
                )
            }
            pub fn reject_proposal(
                &self,
                id: pallet_pips::PipId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::reject_proposal { id },
                )
            }
            pub fn prune_proposal(
                &self,
                id: pallet_pips::PipId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::prune_proposal { id },
                )
            }
            pub fn reschedule_execution(
                &self,
                id: pallet_pips::PipId,
                until: Option<u32>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::reschedule_execution { id, until },
                )
            }
            pub fn clear_snapshot(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::clear_snapshot,
                )
            }
            pub fn snapshot(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::snapshot,
                )
            }
            pub fn enact_snapshot_results(
                &self,
                results: Vec<(pallet_pips::PipId, pallet_pips::SnapshotResult)>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::enact_snapshot_results { results },
                )
            }
            pub fn execute_scheduled_pip(
                &self,
                id: pallet_pips::PipId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::execute_scheduled_pip { id },
                )
            }
            pub fn expire_scheduled_pip(
                &self,
                did: polymesh_primitives::identity_id::IdentityId,
                id: pallet_pips::PipId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Pips(
                    types::pallet_pips::Call::expire_scheduled_pip { did, id },
                )
            }
        }
    }
    pub mod portfolio {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn create_portfolio(
                &self,
                name: polymesh_primitives::identity_id::PortfolioName,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Portfolio(
                    types::pallet_portfolio::Call::create_portfolio { name },
                )
            }
            pub fn delete_portfolio(
                &self,
                num: polymesh_primitives::identity_id::PortfolioNumber,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Portfolio(
                    types::pallet_portfolio::Call::delete_portfolio { num },
                )
            }
            pub fn move_portfolio_funds(
                &self,
                from: polymesh_primitives::identity_id::PortfolioId,
                to: polymesh_primitives::identity_id::PortfolioId,
                items: Vec<pallet_portfolio::MovePortfolioItem>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Portfolio(
                    types::pallet_portfolio::Call::move_portfolio_funds { from, to, items },
                )
            }
            pub fn rename_portfolio(
                &self,
                num: polymesh_primitives::identity_id::PortfolioNumber,
                to_name: polymesh_primitives::identity_id::PortfolioName,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Portfolio(
                    types::pallet_portfolio::Call::rename_portfolio { num, to_name },
                )
            }
            pub fn quit_portfolio_custody(
                &self,
                pid: polymesh_primitives::identity_id::PortfolioId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Portfolio(
                    types::pallet_portfolio::Call::quit_portfolio_custody { pid },
                )
            }
            pub fn accept_portfolio_custody(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Portfolio(
                    types::pallet_portfolio::Call::accept_portfolio_custody { auth_id },
                )
            }
        }
    }
    pub mod protocol_fee {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn change_coefficient(
                &self,
                coefficient: polymesh_primitives::PosRatio,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ProtocolFee(
                    types::pallet_protocol_fee::Call::change_coefficient { coefficient },
                )
            }
            pub fn change_base_fee(
                &self,
                op: polymesh_common_utilities::protocol_fee::ProtocolOp,
                base_fee: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ProtocolFee(
                    types::pallet_protocol_fee::Call::change_base_fee { op, base_fee },
                )
            }
        }
    }
    pub mod scheduler {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn schedule(
                &self,
                when: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: frame_support::traits::schedule::MaybeHashed<
                    polymesh_runtime_develop::runtime::Call,
                    primitive_types::H256,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Scheduler(
                    types::pallet_scheduler::pallet::Call::schedule {
                        when,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn cancel(&self, when: u32, index: u32) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Scheduler(
                    types::pallet_scheduler::pallet::Call::cancel { when, index },
                )
            }
            pub fn schedule_named(
                &self,
                id: Vec<u8>,
                when: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: frame_support::traits::schedule::MaybeHashed<
                    polymesh_runtime_develop::runtime::Call,
                    primitive_types::H256,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Scheduler(
                    types::pallet_scheduler::pallet::Call::schedule_named {
                        id,
                        when,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn cancel_named(&self, id: Vec<u8>) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Scheduler(
                    types::pallet_scheduler::pallet::Call::cancel_named { id },
                )
            }
            pub fn schedule_after(
                &self,
                after: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: frame_support::traits::schedule::MaybeHashed<
                    polymesh_runtime_develop::runtime::Call,
                    primitive_types::H256,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Scheduler(
                    types::pallet_scheduler::pallet::Call::schedule_after {
                        after,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
            pub fn schedule_named_after(
                &self,
                id: Vec<u8>,
                after: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: frame_support::traits::schedule::MaybeHashed<
                    polymesh_runtime_develop::runtime::Call,
                    primitive_types::H256,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Scheduler(
                    types::pallet_scheduler::pallet::Call::schedule_named_after {
                        id,
                        after,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    },
                )
            }
        }
    }
    pub mod settlement {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn create_venue(
                &self,
                details: pallet_settlement::VenueDetails,
                signers: Vec<sp_core::crypto::AccountId32>,
                typ: pallet_settlement::VenueType,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::create_venue {
                        details,
                        signers,
                        typ,
                    },
                )
            }
            pub fn update_venue_details(
                &self,
                id: pallet_settlement::VenueId,
                details: pallet_settlement::VenueDetails,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::update_venue_details { id, details },
                )
            }
            pub fn update_venue_type(
                &self,
                id: pallet_settlement::VenueId,
                typ: pallet_settlement::VenueType,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::update_venue_type { id, typ },
                )
            }
            pub fn add_instruction(
                &self,
                venue_id: pallet_settlement::VenueId,
                settlement_type: pallet_settlement::SettlementType<u32>,
                trade_date: Option<u64>,
                value_date: Option<u64>,
                legs: Vec<pallet_settlement::Leg>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::add_instruction {
                        venue_id,
                        settlement_type,
                        trade_date,
                        value_date,
                        legs,
                    },
                )
            }
            pub fn add_and_affirm_instruction(
                &self,
                venue_id: pallet_settlement::VenueId,
                settlement_type: pallet_settlement::SettlementType<u32>,
                trade_date: Option<u64>,
                value_date: Option<u64>,
                legs: Vec<pallet_settlement::Leg>,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::add_and_affirm_instruction {
                        venue_id,
                        settlement_type,
                        trade_date,
                        value_date,
                        legs,
                        portfolios,
                    },
                )
            }
            pub fn affirm_instruction(
                &self,
                id: pallet_settlement::InstructionId,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
                max_legs_count: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::affirm_instruction {
                        id,
                        portfolios,
                        max_legs_count,
                    },
                )
            }
            pub fn withdraw_affirmation(
                &self,
                id: pallet_settlement::InstructionId,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
                max_legs_count: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::withdraw_affirmation {
                        id,
                        portfolios,
                        max_legs_count,
                    },
                )
            }
            pub fn reject_instruction(
                &self,
                id: pallet_settlement::InstructionId,
                portfolio: polymesh_primitives::identity_id::PortfolioId,
                num_of_legs: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::reject_instruction {
                        id,
                        portfolio,
                        num_of_legs,
                    },
                )
            }
            pub fn affirm_with_receipts(
                &self,
                id: pallet_settlement::InstructionId,
                receipt_details: Vec<
                    pallet_settlement::ReceiptDetails<
                        sp_core::crypto::AccountId32,
                        sp_runtime::MultiSignature,
                    >,
                >,
                portfolios: Vec<polymesh_primitives::identity_id::PortfolioId>,
                max_legs_count: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::affirm_with_receipts {
                        id,
                        receipt_details,
                        portfolios,
                        max_legs_count,
                    },
                )
            }
            pub fn claim_receipt(
                &self,
                id: pallet_settlement::InstructionId,
                receipt_details: pallet_settlement::ReceiptDetails<
                    sp_core::crypto::AccountId32,
                    sp_runtime::MultiSignature,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::claim_receipt {
                        id,
                        receipt_details,
                    },
                )
            }
            pub fn unclaim_receipt(
                &self,
                instruction_id: pallet_settlement::InstructionId,
                leg_id: pallet_settlement::LegId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::unclaim_receipt {
                        instruction_id,
                        leg_id,
                    },
                )
            }
            pub fn set_venue_filtering(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                enabled: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::set_venue_filtering { ticker, enabled },
                )
            }
            pub fn allow_venues(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                venues: Vec<pallet_settlement::VenueId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::allow_venues { ticker, venues },
                )
            }
            pub fn disallow_venues(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                venues: Vec<pallet_settlement::VenueId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::disallow_venues { ticker, venues },
                )
            }
            pub fn change_receipt_validity(
                &self,
                receipt_uid: u64,
                validity: bool,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::change_receipt_validity {
                        receipt_uid,
                        validity,
                    },
                )
            }
            pub fn execute_scheduled_instruction(
                &self,
                id: pallet_settlement::InstructionId,
                _legs_count: u32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::execute_scheduled_instruction {
                        id,
                        _legs_count,
                    },
                )
            }
            pub fn reschedule_instruction(
                &self,
                id: pallet_settlement::InstructionId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Settlement(
                    types::pallet_settlement::Call::reschedule_instruction { id },
                )
            }
        }
    }
    pub mod statistics {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_active_asset_stats(
                &self,
                asset: polymesh_primitives::statistics::AssetScope,
                stat_types: Vec<polymesh_primitives::statistics::StatType>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Statistics(
                    types::pallet_statistics::Call::set_active_asset_stats { asset, stat_types },
                )
            }
            pub fn batch_update_asset_stats(
                &self,
                asset: polymesh_primitives::statistics::AssetScope,
                stat_type: polymesh_primitives::statistics::StatType,
                values: Vec<polymesh_primitives::statistics::StatUpdate>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Statistics(
                    types::pallet_statistics::Call::batch_update_asset_stats {
                        asset,
                        stat_type,
                        values,
                    },
                )
            }
            pub fn set_asset_transfer_compliance(
                &self,
                asset: polymesh_primitives::statistics::AssetScope,
                transfer_conditions: Vec<
                    polymesh_primitives::transfer_compliance::TransferCondition,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Statistics(
                    types::pallet_statistics::Call::set_asset_transfer_compliance {
                        asset,
                        transfer_conditions,
                    },
                )
            }
            pub fn set_entities_exempt(
                &self,
                is_exempt: bool,
                exempt_key: polymesh_primitives::transfer_compliance::TransferConditionExemptKey,
                entities: Vec<polymesh_primitives::identity_id::IdentityId>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Statistics(
                    types::pallet_statistics::Call::set_entities_exempt {
                        is_exempt,
                        exempt_key,
                        entities,
                    },
                )
            }
        }
    }
    pub mod sto {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn create_fundraiser(
                &self,
                offering_portfolio: polymesh_primitives::identity_id::PortfolioId,
                offering_asset: polymesh_primitives::ticker::Ticker,
                raising_portfolio: polymesh_primitives::identity_id::PortfolioId,
                raising_asset: polymesh_primitives::ticker::Ticker,
                tiers: Vec<pallet_sto::PriceTier>,
                venue_id: pallet_settlement::VenueId,
                start: Option<u64>,
                end: Option<u64>,
                minimum_investment: u128,
                fundraiser_name: pallet_sto::FundraiserName,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sto(
                    types::pallet_sto::Call::create_fundraiser {
                        offering_portfolio,
                        offering_asset,
                        raising_portfolio,
                        raising_asset,
                        tiers,
                        venue_id,
                        start,
                        end,
                        minimum_investment,
                        fundraiser_name,
                    },
                )
            }
            pub fn invest(
                &self,
                investment_portfolio: polymesh_primitives::identity_id::PortfolioId,
                funding_portfolio: polymesh_primitives::identity_id::PortfolioId,
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
                purchase_amount: u128,
                max_price: Option<u128>,
                receipt: Option<
                    pallet_settlement::ReceiptDetails<
                        sp_core::crypto::AccountId32,
                        sp_runtime::MultiSignature,
                    >,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sto(
                    types::pallet_sto::Call::invest {
                        investment_portfolio,
                        funding_portfolio,
                        offering_asset,
                        id,
                        purchase_amount,
                        max_price,
                        receipt,
                    },
                )
            }
            pub fn freeze_fundraiser(
                &self,
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sto(
                    types::pallet_sto::Call::freeze_fundraiser { offering_asset, id },
                )
            }
            pub fn unfreeze_fundraiser(
                &self,
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sto(
                    types::pallet_sto::Call::unfreeze_fundraiser { offering_asset, id },
                )
            }
            pub fn modify_fundraiser_window(
                &self,
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
                start: u64,
                end: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sto(
                    types::pallet_sto::Call::modify_fundraiser_window {
                        offering_asset,
                        id,
                        start,
                        end,
                    },
                )
            }
            pub fn stop(
                &self,
                offering_asset: polymesh_primitives::ticker::Ticker,
                id: pallet_sto::FundraiserId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Sto(types::pallet_sto::Call::stop {
                    offering_asset,
                    id,
                })
            }
        }
    }
    pub mod treasury {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn disbursement(
                &self,
                beneficiaries: Vec<polymesh_primitives::Beneficiary<u128>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Treasury(
                    types::pallet_treasury::Call::disbursement { beneficiaries },
                )
            }
            pub fn reimbursement(&self, amount: u128) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Treasury(
                    types::pallet_treasury::Call::reimbursement { amount },
                )
            }
        }
    }
    pub mod utility {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn batch(
                &self,
                calls: Vec<polymesh_runtime_develop::runtime::Call>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Utility(
                    types::pallet_utility::Call::batch { calls },
                )
            }
            pub fn batch_atomic(
                &self,
                calls: Vec<polymesh_runtime_develop::runtime::Call>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Utility(
                    types::pallet_utility::Call::batch_atomic { calls },
                )
            }
            pub fn batch_optimistic(
                &self,
                calls: Vec<polymesh_runtime_develop::runtime::Call>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Utility(
                    types::pallet_utility::Call::batch_optimistic { calls },
                )
            }
            pub fn relay_tx(
                &self,
                target: sp_core::crypto::AccountId32,
                signature: sp_runtime::MultiSignature,
                call: pallet_utility::UniqueCall<polymesh_runtime_develop::runtime::Call>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Utility(
                    types::pallet_utility::Call::relay_tx {
                        target,
                        signature,
                        call,
                    },
                )
            }
        }
    }
    pub mod base {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {}
    }
    pub mod external_agents {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn create_group(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::create_group { ticker, perms },
                )
            }
            pub fn set_group_permissions(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                id: polymesh_primitives::agent::AGId,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::set_group_permissions {
                        ticker,
                        id,
                        perms,
                    },
                )
            }
            pub fn remove_agent(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                agent: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::remove_agent { ticker, agent },
                )
            }
            pub fn abdicate(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::abdicate { ticker },
                )
            }
            pub fn change_group(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                agent: polymesh_primitives::identity_id::IdentityId,
                group: polymesh_primitives::agent::AgentGroup,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::change_group {
                        ticker,
                        agent,
                        group,
                    },
                )
            }
            pub fn accept_become_agent(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::accept_become_agent { auth_id },
                )
            }
            pub fn create_group_and_add_auth(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
                target: polymesh_primitives::identity_id::IdentityId,
                expiry: Option<u64>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::create_group_and_add_auth {
                        ticker,
                        perms,
                        target,
                        expiry,
                    },
                )
            }
            pub fn create_and_change_custom_group(
                &self,
                ticker: polymesh_primitives::ticker::Ticker,
                perms: polymesh_primitives::subset::SubsetRestriction<
                    polymesh_primitives::secondary_key::PalletPermissions,
                >,
                agent: polymesh_primitives::identity_id::IdentityId,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::ExternalAgents(
                    types::pallet_external_agents::Call::create_and_change_custom_group {
                        ticker,
                        perms,
                        agent,
                    },
                )
            }
        }
    }
    pub mod relayer {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn set_paying_key(
                &self,
                user_key: sp_core::crypto::AccountId32,
                polyx_limit: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Relayer(
                    types::pallet_relayer::Call::set_paying_key {
                        user_key,
                        polyx_limit,
                    },
                )
            }
            pub fn accept_paying_key(
                &self,
                auth_id: u64,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Relayer(
                    types::pallet_relayer::Call::accept_paying_key { auth_id },
                )
            }
            pub fn remove_paying_key(
                &self,
                user_key: sp_core::crypto::AccountId32,
                paying_key: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Relayer(
                    types::pallet_relayer::Call::remove_paying_key {
                        user_key,
                        paying_key,
                    },
                )
            }
            pub fn update_polyx_limit(
                &self,
                user_key: sp_core::crypto::AccountId32,
                polyx_limit: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Relayer(
                    types::pallet_relayer::Call::update_polyx_limit {
                        user_key,
                        polyx_limit,
                    },
                )
            }
            pub fn increase_polyx_limit(
                &self,
                user_key: sp_core::crypto::AccountId32,
                amount: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Relayer(
                    types::pallet_relayer::Call::increase_polyx_limit { user_key, amount },
                )
            }
            pub fn decrease_polyx_limit(
                &self,
                user_key: sp_core::crypto::AccountId32,
                amount: u128,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Relayer(
                    types::pallet_relayer::Call::decrease_polyx_limit { user_key, amount },
                )
            }
        }
    }
    pub mod rewards {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn claim_itn_reward(
                &self,
                reward_address: sp_core::crypto::AccountId32,
                itn_address: sp_core::crypto::AccountId32,
                signature: sp_runtime::MultiSignature,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Rewards(
                    types::pallet_rewards::Call::claim_itn_reward {
                        reward_address,
                        itn_address,
                        signature,
                    },
                )
            }
            pub fn set_itn_reward_status(
                &self,
                itn_address: sp_core::crypto::AccountId32,
                status: pallet_rewards::ItnRewardStatus,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Rewards(
                    types::pallet_rewards::Call::set_itn_reward_status {
                        itn_address,
                        status,
                    },
                )
            }
        }
    }
    pub mod contracts {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn call(
                &self,
                dest: sp_runtime::MultiAddress<sp_core::crypto::AccountId32, u32>,
                value: ::codec::Compact<u128>,
                gas_limit: ::codec::Compact<u64>,
                storage_deposit_limit: Option<::codec::Compact<u128>>,
                data: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Contracts(
                    types::pallet_contracts::pallet::Call::call {
                        dest,
                        value,
                        gas_limit,
                        storage_deposit_limit,
                        data,
                    },
                )
            }
            pub fn instantiate_with_code(
                &self,
                value: ::codec::Compact<u128>,
                gas_limit: ::codec::Compact<u64>,
                storage_deposit_limit: Option<::codec::Compact<u128>>,
                code: Vec<u8>,
                data: Vec<u8>,
                salt: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Contracts(
                    types::pallet_contracts::pallet::Call::instantiate_with_code {
                        value,
                        gas_limit,
                        storage_deposit_limit,
                        code,
                        data,
                        salt,
                    },
                )
            }
            pub fn instantiate(
                &self,
                value: ::codec::Compact<u128>,
                gas_limit: ::codec::Compact<u64>,
                storage_deposit_limit: Option<::codec::Compact<u128>>,
                code_hash: primitive_types::H256,
                data: Vec<u8>,
                salt: Vec<u8>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Contracts(
                    types::pallet_contracts::pallet::Call::instantiate {
                        value,
                        gas_limit,
                        storage_deposit_limit,
                        code_hash,
                        data,
                        salt,
                    },
                )
            }
            pub fn upload_code(
                &self,
                code: Vec<u8>,
                storage_deposit_limit: Option<::codec::Compact<u128>>,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Contracts(
                    types::pallet_contracts::pallet::Call::upload_code {
                        code,
                        storage_deposit_limit,
                    },
                )
            }
            pub fn remove_code(
                &self,
                code_hash: primitive_types::H256,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Contracts(
                    types::pallet_contracts::pallet::Call::remove_code { code_hash },
                )
            }
        }
    }
    pub mod polymesh_contracts {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn instantiate_with_code_perms(
                &self,
                endowment: u128,
                gas_limit: u64,
                storage_deposit_limit: Option<u128>,
                code: Vec<u8>,
                data: Vec<u8>,
                salt: Vec<u8>,
                perms: polymesh_primitives::secondary_key::Permissions,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshContracts(
                    types::polymesh_contracts::Call::instantiate_with_code_perms {
                        endowment,
                        gas_limit,
                        storage_deposit_limit,
                        code,
                        data,
                        salt,
                        perms,
                    },
                )
            }
            pub fn instantiate_with_hash_perms(
                &self,
                endowment: u128,
                gas_limit: u64,
                storage_deposit_limit: Option<u128>,
                code_hash: primitive_types::H256,
                data: Vec<u8>,
                salt: Vec<u8>,
                perms: polymesh_primitives::secondary_key::Permissions,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::PolymeshContracts(
                    types::polymesh_contracts::Call::instantiate_with_hash_perms {
                        endowment,
                        gas_limit,
                        storage_deposit_limit,
                        code_hash,
                        data,
                        salt,
                        perms,
                    },
                )
            }
        }
    }
    pub mod preimage {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn note_preimage(&self, bytes: Vec<u8>) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Preimage(
                    types::pallet_preimage::pallet::Call::note_preimage { bytes },
                )
            }
            pub fn unnote_preimage(
                &self,
                hash: primitive_types::H256,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Preimage(
                    types::pallet_preimage::pallet::Call::unnote_preimage { hash },
                )
            }
            pub fn request_preimage(
                &self,
                hash: primitive_types::H256,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Preimage(
                    types::pallet_preimage::pallet::Call::request_preimage { hash },
                )
            }
            pub fn unrequest_preimage(
                &self,
                hash: primitive_types::H256,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::Preimage(
                    types::pallet_preimage::pallet::Call::unrequest_preimage { hash },
                )
            }
        }
    }
    pub mod test_utils {
        use super::*;
        #[derive(Clone, Default)]
        pub struct CallApi;
        impl CallApi {
            pub fn register_did(
                &self,
                uid: polymesh_primitives::cdd_id::InvestorUid,
                secondary_keys: Vec<
                    polymesh_primitives::secondary_key::SecondaryKey<sp_core::crypto::AccountId32>,
                >,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TestUtils(
                    types::pallet_test_utils::Call::register_did {
                        uid,
                        secondary_keys,
                    },
                )
            }
            pub fn mock_cdd_register_did(
                &self,
                target_account: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TestUtils(
                    types::pallet_test_utils::Call::mock_cdd_register_did { target_account },
                )
            }
            pub fn get_my_did(&self) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TestUtils(
                    types::pallet_test_utils::Call::get_my_did,
                )
            }
            pub fn get_cdd_of(
                &self,
                of: sp_core::crypto::AccountId32,
            ) -> polymesh_runtime_develop::runtime::Call {
                types::polymesh_runtime_develop::runtime::Call::TestUtils(
                    types::pallet_test_utils::Call::get_cdd_of { of },
                )
            }
        }
    }
}

