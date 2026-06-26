//! Configuration of the supporting FRAME pallets (assets, NFTs, fractionalization,
//! assets holder/freezer) and the Xcavate custom pallets.
//!
//! This is adapted from the Xcavate parachain runtime. Parachain-specific concerns
//! (XCM, cumulus, treasury pallet, KILT) are intentionally omitted; the treasury is
//! represented by a plain `PalletId` constant, matching the parachain's placeholder.

use frame_support::{
	instances::{Instance1, Instance2, Instance3},
	parameter_types,
	traits::{
		AsEnsureOriginWithArg, EnsureOriginWithArg, MapSuccess, OriginTrait,
	},
	BoundedVec, PalletId,
};
use frame_system::{EnsureRoot, EnsureRootWithSuccess, EnsureSigned};
use pallet_nfts::PalletFeatures;
use sp_runtime::{
	traits::{Morph, Verify},
	Perbill, Percent, Permill,
};

use primitives::{AssetMetadataProvider, MarketplaceFreezeReason, MarketplaceHoldReason};

use crate::{
	deposit, AccountId, Assets, AssetsFreezer, AssetsHolder, Balance, Balances, BlockNumber,
	EducationAssets, EducationNfts, EducationRegions, PropertyManagement, RealEstateAssets,
	RealEstateNfts, RealWorldAsset, Regions, Runtime, RuntimeEvent, RuntimeHoldReason, Signature,
	System, XcavateWhitelist, DAYS, EXISTENTIAL_DEPOSIT, MICROXCAV, MILLIXCAV, XCAV,
};

// =====================================================================================
// Assets (three instances)
// =====================================================================================

parameter_types! {
	pub const AssetDeposit: Balance = 10 * XCAV;
	pub const AssetAccountDeposit: Balance = deposit(1, 16);
	pub const ApprovalDeposit: Balance = EXISTENTIAL_DEPOSIT;
	pub const StringLimit: u32 = 5000;
	pub const MetadataDepositBase: Balance = deposit(1, 68);
	pub const MetadataDepositPerByte: Balance = deposit(0, 1);
	pub const RemoveItemsLimit: u32 = 1000;
	pub const ZeroDeposit: Balance = 0;
	pub RootAccountId: AccountId = AccountId::from([0xffu8; 32]);
}

impl pallet_assets::Config<Instance1> for Runtime {
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type AssetDeposit = ZeroDeposit;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Balance = Balance;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type CallbackHandle = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureRootWithSuccess<AccountId, RootAccountId>>;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type Freezer = AssetsFreezer;
	type Holder = ();
	type MetadataDepositBase = ZeroDeposit;
	type MetadataDepositPerByte = ZeroDeposit;
	type RemoveItemsLimit = RemoveItemsLimit;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}

impl pallet_assets::Config<Instance2> for Runtime {
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type AssetDeposit = AssetDeposit;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Balance = Balance;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type CallbackHandle = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureRootWithSuccess<AccountId, RootAccountId>>;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type Freezer = ();
	type Holder = AssetsHolder;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type RemoveItemsLimit = RemoveItemsLimit;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}

impl pallet_assets::Config<Instance3> for Runtime {
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type AssetDeposit = AssetDeposit;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Balance = Balance;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type CallbackHandle = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureRootWithSuccess<AccountId, RootAccountId>>;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type Freezer = ();
	type Holder = ();
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type RemoveItemsLimit = RemoveItemsLimit;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}

// =====================================================================================
// NFTs (two instances) + fractionalization
// =====================================================================================

parameter_types! {
	pub Features: PalletFeatures = PalletFeatures::all_enabled();
	pub const MaxAttributesPerCall: u32 = 10;
	pub const CollectionDeposit: Balance = 0;
	pub const ItemDeposit: Balance = 0;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
	pub const ApprovalsLimit: u32 = 20;
	pub const ItemAttributesApprovalsLimit: u32 = 20;
	pub const MaxTips: u32 = 10;
	pub const MaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
}

impl pallet_nfts::Config<Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = <Signature as Verify>::Signer;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureRootWithSuccess<AccountId, RootAccountId>>;
	type Locker = ();
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}

impl pallet_nfts::Config<Instance2> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = ZeroDeposit;
	type AttributeDepositBase = ZeroDeposit;
	type DepositPerByte = ZeroDeposit;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = <Signature as Verify>::Signer;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureRootWithSuccess<AccountId, RootAccountId>>;
	type Locker = ();
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}

parameter_types! {
	pub const NftFractionalizationPalletId: PalletId = PalletId(*b"fraction");
	pub NewAssetSymbol: BoundedVec<u8, StringLimit> = (*b"BRIX").to_vec().try_into().unwrap();
	pub NewAssetName: BoundedVec<u8, StringLimit> = (*b"Brix").to_vec().try_into().unwrap();
	pub const Deposit: Balance = XCAV;
}

impl pallet_nft_fractionalization::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Deposit = Deposit;
	type Currency = Balances;
	type NewAssetSymbol = NewAssetSymbol;
	type NewAssetName = NewAssetName;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type AssetBalance = <Self as pallet_balances::Config>::Balance;
	type AssetId = <Self as pallet_assets::Config<Instance1>>::AssetId;
	type Assets = RealEstateAssets;
	type Nfts = RealEstateNfts;
	type PalletId = NftFractionalizationPalletId;
	type WeightInfo = ();
	type StringLimit = StringLimit;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type RuntimeHoldReason = RuntimeHoldReason;
}

impl pallet_assets_holder::Config<Instance2> for Runtime {
	type RuntimeHoldReason = MarketplaceHoldReason;
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_assets_freezer::Config<Instance1> for Runtime {
	type RuntimeFreezeReason = MarketplaceFreezeReason;
	type RuntimeEvent = RuntimeEvent;
}

// =====================================================================================
// Pallet ids shared across the custom pallets
// =====================================================================================

parameter_types! {
	pub const MarketplacePalletId: PalletId = PalletId(*b"py/nftxc");
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const PropertyFundingAmount: Balance = 10 * XCAV;
	pub const MaxPropertyShares: u32 = 250;
}

// =====================================================================================
// Whitelist
// =====================================================================================

parameter_types! {
	pub const WhitelistAirdropNativeAmount: Balance = 10 * XCAV; // 10 XCAV
	pub const WhitelistAirdropAssetId: u32 = 10; // tGBP
	pub const WhitelistAirdropAssetAmount: Balance = 10_000_000_000_000_000_000_000; // 10,000 tGBP (18 decimals)
}

#[cfg(feature = "runtime-benchmarks")]
pub struct WhitelistBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcavate_whitelist::BenchmarkHelper<Runtime> for WhitelistBenchmarkHelper {
	fn setup_airdrop_asset() {
		use frame_support::traits::fungibles::Create;
		let admin = AccountId::from([0xffu8; 32]);
		let _ = <Assets as Create<AccountId>>::create(WhitelistAirdropAssetId::get(), admin, true, 1);
	}
}

impl pallet_xcavate_whitelist::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_xcavate_whitelist::weights::SubstrateWeight<Runtime>;
	type WhitelistOrigin = EnsureRoot<Self::AccountId>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type ForeignCurrency = Assets;
	type AirdropNativeAmount = WhitelistAirdropNativeAmount;
	type AirdropAssetId = WhitelistAirdropAssetId;
	type AirdropAssetAmount = WhitelistAirdropAssetAmount;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = WhitelistBenchmarkHelper;
}

use pallet_xcavate_whitelist::{self as whitelist, RolePermission};

pub struct EnsureHasRole<T>(core::marker::PhantomData<T>);

impl<T: whitelist::Config> EnsureOriginWithArg<T::RuntimeOrigin, whitelist::Role> for EnsureHasRole<T> {
	type Success = T::AccountId;

	fn try_origin(
		origin: T::RuntimeOrigin,
		role: &whitelist::Role,
	) -> Result<Self::Success, T::RuntimeOrigin> {
		let Some(who) = origin.clone().into_signer() else {
			return Err(origin);
		};
		if whitelist::Pallet::<T>::has_role(&who, role.clone()) {
			Ok(who)
		} else {
			Err(origin)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(_role: &whitelist::Role) -> Result<T::RuntimeOrigin, ()> {
		let account = frame_benchmarking::whitelisted_caller();
		Ok(frame_system::RawOrigin::Signed(account).into())
	}
}

pub struct EnsureCompliant<T>(core::marker::PhantomData<T>);

impl<T: whitelist::Config> EnsureOriginWithArg<T::RuntimeOrigin, whitelist::Role> for EnsureCompliant<T> {
	type Success = T::AccountId;

	fn try_origin(
		origin: T::RuntimeOrigin,
		role: &whitelist::Role,
	) -> Result<Self::Success, T::RuntimeOrigin> {
		let Some(who) = origin.clone().into_signer() else {
			return Err(origin);
		};
		if whitelist::Pallet::<T>::is_compliant(&who, role.clone()) {
			Ok(who)
		} else {
			Err(origin)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(_role: &whitelist::Role) -> Result<T::RuntimeOrigin, ()> {
		let account = frame_benchmarking::whitelisted_caller();
		Ok(frame_system::RawOrigin::Signed(account).into())
	}
}

// =====================================================================================
// Regions + Education regions
// =====================================================================================

parameter_types! {
	pub const Postcode: u32 = 10;
	pub const LocationDepositAmount: Balance = 10_000 * XCAV;
	pub const MaximumListingDuration: BlockNumber = 30 * DAYS;
	pub const RegionVotingTime: BlockNumber = 30;
	pub const RegionAuctionTime: BlockNumber = 30;
	pub const RegionOperatorVotingTime: BlockNumber = 20;
	pub const RegionThreshold: Percent = Percent::from_percent(75);
	pub const MaxProposalForBlock: u32 = 100;
	pub const RegionSlashingAmount: Balance = 10 * XCAV;
	pub const RegionOwnerChangeTime: BlockNumber = 400;
	pub const RegionOwnerNoticeTime: BlockNumber = 50;
	pub const RegionOwnerDisputeDepositAmount: Balance = 1_000 * XCAV;
	pub const MinimumRegionDepositAmount: Balance = 100_000 * XCAV;
	pub const RegionProposalDepositAmount: Balance = 5_000 * XCAV;
	pub const MinimumVotingPower: Balance = 100 * XCAV;
	pub const LawyerDepositAmount: Balance = 10_000 * XCAV;
	pub const MaximumTaxPercent: Permill = Permill::from_percent(10);
	pub const MaxAllowedStrikes: u8 = 3;
	pub const RegionVotingQuorum: Permill = Permill::from_percent(1);
}

impl pallet_regions::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_regions::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Nfts = RealEstateNfts;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type MarketplacePalletId = MarketplacePalletId;
	type MaxListingDuration = MaximumListingDuration;
	type PostcodeLimit = Postcode;
	type LocationDeposit = LocationDepositAmount;
	type RegionVotingTime = RegionVotingTime;
	type RegionAuctionTime = RegionAuctionTime;
	type RegionThreshold = RegionThreshold;
	type RegionOperatorVotingTime = RegionOperatorVotingTime;
	type MaxProposalsForBlock = MaxProposalForBlock;
	type RegionSlashingAmount = RegionSlashingAmount;
	type TreasuryId = TreasuryPalletId;
	type RegionOwnerChangePeriod = RegionOwnerChangeTime;
	type Slash = ();
	type RegionOwnerNoticePeriod = RegionOwnerNoticeTime;
	type RegionOwnerDisputeDeposit = RegionOwnerDisputeDepositAmount;
	type MinimumRegionDeposit = MinimumRegionDepositAmount;
	type RegionProposalDeposit = RegionProposalDepositAmount;
	type MinimumVotingAmount = MinimumVotingPower;
	type PermissionOrigin = EnsureHasRole<Self>;
	type LawyerDeposit = LawyerDepositAmount;
	type BlockNumberProvider = System;
	type MaxTaxPercent = MaximumTaxPercent;
	type AllowedStrikes = MaxAllowedStrikes;
	type MinVotingQuorum = RegionVotingQuorum;
}

impl pallet_education_regions::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_education_regions::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RegionVotingTime = RegionVotingTime;
	type RegionAuctionTime = RegionAuctionTime;
	type RegionThreshold = RegionThreshold;
	type RegionOperatorVotingTime = RegionOperatorVotingTime;
	type MaxProposalsForBlock = MaxProposalForBlock;
	type RegionSlashingAmount = RegionSlashingAmount;
	type TreasuryId = TreasuryPalletId;
	type RegionOwnerChangePeriod = RegionOwnerChangeTime;
	type Slash = ();
	type RegionOwnerNoticePeriod = RegionOwnerNoticeTime;
	type RegionOwnerDisputeDeposit = RegionOwnerDisputeDepositAmount;
	type MinimumRegionDeposit = MinimumRegionDepositAmount;
	type RegionProposalDeposit = RegionProposalDepositAmount;
	type MinimumVotingAmount = MinimumVotingPower;
	type PermissionOrigin = EnsureHasRole<Self>;
	type BlockNumberProvider = System;
	type AllowedStrikes = MaxAllowedStrikes;
	type MinVotingQuorum = RegionVotingQuorum;
}

// =====================================================================================
// RealX education
// =====================================================================================

parameter_types! {
	pub const XEducationPalletId: PalletId = PalletId(*b"py/xeduc");
	pub const MaximumModuleToken: u32 = 1000;
	pub const ModulePriceLimit: Balance = 100;
	pub const ContentCreatorPercentage: Perbill = Perbill::from_parts(83_000_000);
	pub const RegionalOperatorPercentage: Perbill = Perbill::from_parts(83_000_000);
	pub const ProtocolPercentage: Perbill = Perbill::from_parts(50_000_000);
	pub const DBSPercentage: Perbill = Perbill::from_parts(34_000_000);
	pub const BookingDepositAmount: Balance = 10 * XCAV;
	pub const ModuleDepositAmount: Balance = 100 * XCAV;
	pub const MaxCancellationAmount: u32 = 5;
	pub const CancellationWindow: BlockNumber = 100;
	pub const SponsorshipWindow: BlockNumber = 200;
	pub const ModuleDelivererDepositAmount: Balance = 500 * XCAV;
	pub const MaxAllowedStrikesAmount: u8 = 3;
	pub const StrikeSlashPercentage: Perbill = Perbill::from_parts(100_000_000);
	pub const MaxCleanupPerCallAmount: u32 = 50;
	pub const MinimumImpactScore: Permill = Permill::from_percent(50);
	pub const SuccessfulDeliveriesForStrikeReduction: u32 = 5;
	pub const AcceptedPaymentAssets: [u32; 3] = [10, 1337, 1984];
}

pub struct AssetsMetadataWrapper;

impl AssetMetadataProvider for AssetsMetadataWrapper {
	type AssetId = u32;

	fn get_decimals(asset_id: Self::AssetId) -> Option<u8> {
		Some(pallet_assets::Metadata::<Runtime, Instance2>::get(asset_id).decimals)
	}
}

impl pallet_real_x_education::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_real_x_education::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Nfts = EducationNfts;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance2>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance2>>::ItemId;
	type MaxModuleToken = MaximumModuleToken;
	type LocalCurrency = EducationAssets;
	type ForeignCurrency = Assets;
	type ForeignAssetsHolder = AssetsHolder;
	type StringLimit = StringLimit;
	type ModulePrice = ModulePriceLimit;
	type BlockNumberProvider = System;
	type ContentCreatorPercentage = ContentCreatorPercentage;
	type RegionalOperatorPercentage = RegionalOperatorPercentage;
	type ProtocolPercentage = ProtocolPercentage;
	type DBSPercentage = DBSPercentage;
	type PalletId = XEducationPalletId;
	type TreasuryId = TreasuryPalletId;
	type PermissionOrigin = EnsureHasRole<Self>;
	type AcceptedAssets = AcceptedPaymentAssets;
	type BookingDeposit = BookingDepositAmount;
	type ModuleDeposit = ModuleDepositAmount;
	type RegionProvider = EducationRegions;
	type NewAssetSymbol = NewAssetSymbol;
	type NewAssetName = NewAssetName;
	type Slash = ();
	type MaxCancellations = MaxCancellationAmount;
	type CancellationWindow = CancellationWindow;
	type SponsorshipWindow = SponsorshipWindow;
	type ModuleDelivererDeposit = ModuleDelivererDepositAmount;
	type MaxAllowedStrikes = MaxAllowedStrikesAmount;
	type StrikeSlashPercentage = StrikeSlashPercentage;
	type MaxCleanupPerCall = MaxCleanupPerCallAmount;
	type MinImpactScore = MinimumImpactScore;
	type SuccessfulDeliveriesForStrikeReduction = SuccessfulDeliveriesForStrikeReduction;
	type RoleProvider = XcavateWhitelist;
	type AssetMetadata = AssetsMetadataWrapper;
}

// =====================================================================================
// Real world asset
// =====================================================================================

pub struct BucketNamespaceManager;

impl pallet_real_world_asset::NamespaceManager<AccountId> for BucketNamespaceManager {
	fn create_namespace_for_property(
		manager: &AccountId,
		real_world_asset_id: u32,
	) -> Result<u128, frame_support::pallet_prelude::DispatchError> {
		let mut properties = frame_support::storage::bounded_btree_map::BoundedBTreeMap::default();
		properties
			.try_insert(
				BoundedVec::truncate_from(b"propertyId".to_vec()),
				BoundedVec::truncate_from(real_world_asset_id.to_le_bytes().to_vec()),
			)
			.map_err(|_| {
				frame_support::pallet_prelude::DispatchError::Other(
					"Namespace metadata properties full",
				)
			})?;

		let namespace_id = pallet_bucket::NextNamespaceId::<Runtime>::get();
		let metadata_input = pallet_bucket::types::NamespaceMetadataInput::<Runtime> {
			name: BoundedVec::truncate_from(b"Property namespace".to_vec()),
			schema_uri: None,
			properties,
		};

		<pallet_bucket::Pallet<Runtime> as pallet_bucket::traits::Create<Runtime>>::namespace(
			metadata_input.into(),
			Some(manager.clone()),
		)?;

		Ok(namespace_id)
	}
}

impl pallet_real_world_asset::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type Nfts = RealEstateNfts;
	type MarketplacePalletId = MarketplacePalletId;
	type LocalCurrency = RealEstateAssets;
	type FractionalizeCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type FractionalizeItemId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type AssetId = <Self as pallet_assets::Config<Instance1>>::AssetId;
	type PropertyAccountFundingAmount = PropertyFundingAmount;
	type MaxPropertyShares = MaxPropertyShares;
	type StringLimit = StringLimit;
	type RegionProvider = Regions;
	type PostcodeLimit = Postcode;
	type NamespaceManager = BucketNamespaceManager;
}

// =====================================================================================
// Marketplace
// =====================================================================================

parameter_types! {
	pub const MinPropertyShares: u32 = 100;
	pub const ListingDepositAmount: Balance = 10 * MICROXCAV;
	pub const MarketplaceFeePercent: Perbill = Perbill::from_percent(1);
	pub const MaximumAcceptedAssets: u32 = 2;
	pub const LawyerVotingDuration: BlockNumber = 30;
	pub const LegalProcessDuration: BlockNumber = 80;
	pub const MinimumVotingQuorum: Percent = Percent::from_percent(50);
	pub const ClaimWindowTime: BlockNumber = 100;
	pub const MaximumRelistAttempts: u8 = 1;
	pub const MaxOwnershipPercentage: Perbill = Perbill::from_percent(50);
	pub const AcceptedMarketplacePaymentAssets: [u32; 2] = [10, 1];
}

impl pallet_marketplace::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_marketplace::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type LocalCurrency = RealEstateAssets;
	type ForeignCurrency = Assets;
	type ForeignAssetsHolder = AssetsHolder;
	type AssetsFreezer = AssetsFreezer;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type PalletId = MarketplacePalletId;
	type MinPropertyShares = MinPropertyShares;
	type MaxPropertyShares = MaxPropertyShares;
	type TreasuryId = TreasuryPalletId;
	type AssetId = <Self as pallet_assets::Config<Instance1>>::AssetId;
	type ListingDeposit = ListingDepositAmount;
	type MarketplaceFeePercentage = MarketplaceFeePercent;
	type AcceptedAssets = AcceptedMarketplacePaymentAssets;
	type MaxAcceptedAssets = MaximumAcceptedAssets;
	type PropertyShares = RealWorldAsset;
	type LawyerVotingTime = LawyerVotingDuration;
	type LegalProcessTime = LegalProcessDuration;
	type Whitelist = XcavateWhitelist;
	type PermissionOrigin = EnsureHasRole<Self>;
	type CompliantOrigin = EnsureCompliant<Self>;
	type MinVotingQuorum = MinimumVotingQuorum;
	type ClaimWindow = ClaimWindowTime;
	type MaxRelistAttempts = MaximumRelistAttempts;
	type BlockNumberProvider = System;
	type IncomeSettlement = PropertyManagement;
	type RegionProvider = Regions;
	type StringLimit = StringLimit;
	type PostcodeLimit = Postcode;
	type MaxOwnershipPercentage = MaxOwnershipPercentage;
}

// =====================================================================================
// Property management
// =====================================================================================

parameter_types! {
	pub const MinimumStakingAmount: Balance = 1000 * XCAV;
	pub const MaxProperty: u32 = 1000;
	pub const MaxLocation: u32 = 50;
	pub const LettingAgentVotingDuration: BlockNumber = 20;
	pub const LettingAgentNoticeTime: BlockNumber = 30;
	pub const MaximumNoticesPerBlock: u32 = 10;
}

impl pallet_property_management::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_property_management::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type RuntimeHoldReason = RuntimeHoldReason;
	type NativeCurrency = Balances;
	type ForeignCurrency = Assets;
	type AssetsFreezer = AssetsFreezer;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type MarketplacePalletId = MarketplacePalletId;
	type LettingAgentDeposit = MinimumStakingAmount;
	type MaxProperties = MaxProperty;
	type MaxLocations = MaxLocation;
	type AcceptedAssets = AcceptedMarketplacePaymentAssets;
	type PropertyShares = RealWorldAsset;
	type LettingAgentVotingTime = LettingAgentVotingDuration;
	type PermissionOrigin = EnsureHasRole<Self>;
	type MinVotingQuorum = MinimumVotingQuorum;
	type LettingAgentNoticePeriod = LettingAgentNoticeTime;
	type MaxNoticesPerBlock = MaximumNoticesPerBlock;
	type BlockNumberProvider = System;
	type RegionProvider = Regions;
	type PostcodeLimit = Postcode;
}

// =====================================================================================
// Property governance
// =====================================================================================

parameter_types! {
	pub const PropertyVotingTime: BlockNumber = 20;
	pub const MaxVoteForBlock: u32 = 100;
	pub const MinimumSlashingAmount: Balance = 10 * XCAV;
	pub const VotingThreshold: Percent = Percent::from_percent(51);
	pub const HighVotingThreshold: Percent = Percent::from_percent(67);
	pub const LowProposal: Balance = 500 * XCAV;
	pub const HighProposal: Balance = 10_000 * XCAV;
	pub const ChallengeDepositAmount: Balance = 500 * XCAV;
	pub const AutoExecutionCooldown: BlockNumber = 28;
}

impl pallet_property_governance::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_property_governance::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type AssetsFreezer = AssetsFreezer;
	type NftCollectionId = <Self as pallet_nfts::Config<Instance1>>::CollectionId;
	type NftId = <Self as pallet_nfts::Config<Instance1>>::ItemId;
	type VotingTime = PropertyVotingTime;
	type MaxVotesForBlock = MaxVoteForBlock;
	type MinSlashingAmount = MinimumSlashingAmount;
	type HighThreshold = HighVotingThreshold;
	type LowProposal = LowProposal;
	type HighProposal = HighProposal;
	type MarketplacePalletId = MarketplacePalletId;
	type Slash = ();
	type PropertyShares = RealWorldAsset;
	type PermissionOrigin = EnsureHasRole<Self>;
	type MinVotingQuorum = MinimumVotingQuorum;
	type BlockNumberProvider = System;
	type ChallengeDeposit = ChallengeDepositAmount;
	type StringLimit = StringLimit;
	type PostcodeLimit = Postcode;
	type AutoExecutionCooldown = AutoExecutionCooldown;
}

// =====================================================================================
// Buckets
// =====================================================================================

use pallet_bucket::{traits::CallSources, AccountIdOf};

pub struct SuccessOrigin {
	sender: AccountIdOf<Runtime>,
}
impl SuccessOrigin {
	fn new(sender: AccountIdOf<Runtime>) -> Self {
		SuccessOrigin { sender }
	}
}

impl CallSources<AccountIdOf<Runtime>, AccountIdOf<Runtime>> for SuccessOrigin {
	fn sender(&self) -> AccountIdOf<Runtime> {
		self.sender.clone()
	}

	fn subject(&self) -> AccountIdOf<Runtime> {
		self.sender.clone()
	}
}
impl Morph<AccountIdOf<Runtime>> for SuccessOrigin {
	type Outcome = Self;

	fn morph(a: AccountIdOf<Runtime>) -> Self::Outcome {
		Self::new(a)
	}
}

type BucketsOrigin = MapSuccess<EnsureSigned<AccountIdOf<Runtime>>, SuccessOrigin>;

parameter_types! {
	pub const NamespaceStorageFee: Balance = deposit(1, 2856);
	pub const BucketStorageFee: Balance = deposit(1, 2650);
	pub const MessageStorageFee: Balance = deposit(1, 2682);
	pub const StorageFee: Balance = MILLIXCAV;
	pub const MaxStringLength: u32 = 200;
	#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
	pub const MaxNameLen: u32 = 100;
	#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
	pub const MaxUriLen: u32 = 256;
	#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
	pub const MaxCategoryLen: u32 = 50;
	#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
	pub const MaxProperties: u32 = 10;
	#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
	pub const MaxPropertyKeyLen: u32 = 50;
	#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
	pub const MaxPropertyValueLen: u32 = 200;
}

impl pallet_bucket::Config for Runtime {
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type BucketId = u128;
	type Currency = Balances;
	type FeeBucket = BucketStorageFee;
	type FeeCollector = ();
	type FeeMessage = MessageStorageFee;
	type FeeNamespace = NamespaceStorageFee;
	type FeeTag = StorageFee;
	type ForceOriginCheck = EnsureRoot<AccountId>;
	type KeyId = pallet_bucket::types::BucketPublicKey;
	type MaxStringInputLengthTag = MaxStringLength;
	type MessageId = u128;
	type NamespaceId = u128;
	type NamespaceMetadataInput = pallet_bucket::types::NamespaceMetadataInput<Self>;
	type BucketMetadataInput = pallet_bucket::types::BucketMetadataInput<Self>;
	type MessageMetadataInput = pallet_bucket::types::MessageMetadataInput<Self>;
	type NamespaceMetadata = pallet_bucket::types::NamespaceMetadata<Self>;
	type BucketMetadata = pallet_bucket::types::BucketMetadata<Self>;
	type MessageMetadata = pallet_bucket::types::MessageMetadata<Self>;
	type OnCallHooks = ();
	type OriginCheck = BucketsOrigin;
	type OriginSuccess = SuccessOrigin;
	type Reference = BoundedVec<u8, MaxStringLength>;
	type RuntimeEvent = RuntimeEvent;
	type SubjectId = AccountIdOf<Runtime>;
	type WeightInfo = pallet_bucket::weights::SubstrateWeight<Runtime>;
	type MaxNameLen = MaxNameLen;
	type MaxUriLen = MaxUriLen;
	type MaxCategoryLen = MaxCategoryLen;
	type MaxProperties = MaxProperties;
	type MaxPropertyKeyLen = MaxPropertyKeyLen;
	type MaxPropertyValueLen = MaxPropertyValueLen;
}

// =====================================================================================
// Faucet
// =====================================================================================

parameter_types! {
	pub const FaucetDripAssetId: u32 = 10;
	pub const FaucetDripAmount: Balance = 1_000_000_000_000_000_000_000; // 1000 tGBP (18 decimals)
	pub const FaucetMinXcavBalance: Balance = XCAV;
	pub const FaucetCooldownPeriod: BlockNumber = 7 * DAYS;
}

impl pallet_faucet::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_faucet::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type ForeignCurrency = Assets;
	type DripAssetId = FaucetDripAssetId;
	type DripAmount = FaucetDripAmount;
	type MinXcavBalance = FaucetMinXcavBalance;
	type CooldownPeriod = FaucetCooldownPeriod;
	type BlockNumberProvider = System;
}
