use crate::*;
use near_sdk::CryptoHash;

pub type DropId = String;
pub type ScavengerId = String;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    DataByVendor,
    AccountIdByPubKey,
    VendorItems { vendor_id_hash: CryptoHash },
    DropsClaimedByAccount,
    DropsClaimedByAccountInner { account_id_hash: CryptoHash },
    DropById,
    AccontStatusById,
    BalanceByAccount,
    TicketDataById,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AccountStatus {
    Basic,
    Vendor,
    Sponsor,
    Admin
}

impl AccountStatus {
    pub fn is_admin(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_sponsor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => true,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_vendor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => true,
            AccountStatus::Sponsor => false,
            AccountStatus::Admin => true,
        }
    }
}

/// For each vendor, there's a store-front and list of items for sale
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketType {
    pub starting_near_balance: U128,
    pub starting_token_balance: U128,
    pub account_type: AccountStatus
}

/// For each vendor, there's a store-front and list of items for sale
#[derive(BorshSerialize, BorshDeserialize)]
pub struct VendorInformation {
    /// Info to render on the store-front
    pub metadata: VendorMetadata,
    /// List of items for sale
    pub item_by_id: UnorderedMap<u64, InternalVendorItem>,
}

/// Represents an asset that is purchasable.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InternalVendorItem {
    pub id: String,
    pub name: String,
    pub image: String,
    /// Price in $TOKENS
    pub price: U128,
    /// Is the item currently purchasable?
    pub in_stock: bool,
}

/// Represents an asset that is purchasable.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtVendorItem {
    pub name: String,
    pub image: String,
    /// Price in $TOKENS
    pub price: U128,
    /// Is the item currently purchasable?
    pub in_stock: bool,
}

/// Store-front information for a vendor
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VendorMetadata {
    pub name: String,
    pub description: String,
    /// Must be IPFS CID
    pub cover_image: String,
}

// Outlines the different types of drops that can be created and claimed
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum InternalDropData {
    token(TokenDropData),
    nft(NFTDropData),
}

impl InternalDropData {
    pub fn get_id(&self) -> String {
        match self {
            InternalDropData::token(data) => data.id.clone(),
            InternalDropData::nft(data) => data.id.clone(),
        }
    }

    pub fn get_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            InternalDropData::token(data) => data.scavenger_ids.clone(),
            InternalDropData::nft(data) => data.scavenger_ids.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            InternalDropData::token(data) => data.name.clone(),
            InternalDropData::nft(data) => data.name.clone(),
        }
    }

    pub fn get_image(&self) -> String {
        match self {
            InternalDropData::token(data) => data.image.clone(),
            InternalDropData::nft(data) => data.image.clone(),
        }
    }
}

// Allows users to claim a set of tokens. If scavenger_ids are set, all the ids need to be claimed
// before the user gets the `amount`
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenDropData {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
    pub amount: U128,

    pub name: String,
    pub image: String,
}

// Allows users to claim NFTs. If scavenger_ids are set, all the ids need to be claimed
// before the user gets the NFT
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTDropData {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
    pub name: String,
    pub image: String,

    pub contract_id: String,
    pub method: String,
    pub args: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTWithOwnership {
    pub nft: NFTDropData,
    pub is_owned: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ScavengerHuntWithOwnership {
    pub id: String,
    pub scavenger_ids: Vec<String>,
    pub found: Vec<String>,
    pub name: String,
    pub image: String,
}
