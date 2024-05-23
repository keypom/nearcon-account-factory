use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

/// The image URL for the default icon
pub const DATA_IMAGE_SVG_GT_ICON: &str = "https://assets-global.website-files.com/6509ee7744afed1c907f8f97/655050623557b10a706dae04_ETHDEN_logo_full_purple-p-500.png";

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FungibleTokenMetadata {
    pub spec: String, // Should be ft-1.0.0 to indicate that a Fungible Token contract adheres to the current versions of this Metadata and the Fungible Token Core specs. This will allow consumers of the Fungible Token to know if they support the features of a given contract.
    pub name: String, // The human-readable name of the token.
    pub symbol: String, // The abbreviation, like wETH or AMPL.
    pub icon: Option<String>, // Icon of the fungible token.
    pub reference: Option<String>, // A link to a valid JSON file containing various keys offering supplementary details on the token
    pub reference_hash: Option<Base64VecU8>, // The base64-encoded sha256 hash of the JSON file contained in the reference field. This is to guard against off-chain tampering.
    pub decimals: u8, // used in frontends to show the proper significant digits of a token. This concept is explained well in this OpenZeppelin post. https://docs.openzeppelin.com/contracts/3.x/erc20#a-note-on-decimals
    pub minted_per_claim: Option<U128>, // The number of tokens that will be minted per claim (depends on drop ID passed in)
}

#[near_bindgen]
impl Contract {
    pub fn ft_metadata(&self, drop_id: Option<String>) -> FungibleTokenMetadata {
        let mut metadata = self.metadata.clone();
        if let Some(id) = drop_id {
            metadata.minted_per_claim = Some(U128(
                self.starting_token_balance
                    .get(&id)
                    .expect("no drop id found"),
            ));
        }
        metadata
    }

    #[private]
    pub fn update_ft_metadata(&mut self, metadata: FungibleTokenMetadata) {
        self.metadata = metadata;
    }
}
