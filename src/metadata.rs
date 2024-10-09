use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen};

use crate::*;

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

pub const DATA_IMAGE_SVG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAABDCAMAAABjnP3jAAAAIVBMVEVMaXFJq49Kq49LqY5Jq49HpopJq45Jqo5JqY1Kq49Ns5XD8x5oAAAACXRSTlMA/e0wxBWZdFI2obyjAAAACXBIWXMAAAsTAAALEwEAmpwYAAAC8UlEQVR4nMVZ6baEIAgeUEB5/we+x6wm16zmJj/nTPCxL34+j4jkM5fEm6nyCdVOBcAK6ibKN+hBLc0DYBU8KE+T7xS8B4+z4pCCAfxEE8giP9CcODTgIwCYlIq8GwB0Rj10m/gAACekot0NMCcO5SDf+yepaJpE1K+BB/kncUjUFvJhjy2ylsXVUTiFFEArCowTthYRqhIATdClQRoI0ErNvE6TGLC1/5BjiwubhoQYO6zQofA9WCn1c7ghgHpDcoyL6B5zMKU7c5CBVJELFc2aBwDKhXwSCxo16/Bek+dbUdooVKGAEHoBLPJL7a2q78leyURVuibYIHjFwhHBex6KMmjsovwpzx05n5og+kIx7zpS+Y0Y0hRpMfwWDzNggtURubddEf7GDhk/LZ88YoI13k9KnsAor2P1pDETLAgKkyfEOsgp6x8yCDt8VwZdlheDfJLaSXtVOf+y3f3XGW0QQMpFLnzaQnBJvs+aB41GQUTgHsqvDFFy5etq++dL8ovuSdfwlwiuaFCdIuUag3wAccP5V4uAi4lQUeHZ13dMEFv5rQBozU9k7zshm9BOv63nsVwBkHK5iL01QNqLJqC70F1v3b5hgifOSyn0kmRo7DPCUQOkXDtHHadH2j48M0HPAN/R+ku9HYbtgRAijhaEzZZNz22iAY9s7eg1gYiMkwVGAwPEqbZeA5ZpXjEsV4Z6W94ZkQsjfpDWGGqqjXTRHbm6VN0gI+grpSaGodTkh3WqsVbeIwqrRg4h1uPCA8sOUW5Sj8kFCLmiUuYAgEK5q/2CSDAzQsgDk4UH6Ono/gACp10/VPU0CU/n9qfkkr4fZrMkBBqb+i/JJC4HdUcA4N842HFyGpEDoP82f7KUbyIZu6PqfyMAtUf9Xzub834e8vg1xptne94dvwF4+9mA8/Ln4d2bPWUF+P2DuUma8OxrNcy+18OURxuzPdlMf7OBWQ93tMbh7Hc7qC37b5oAJr7dcng7nvd0GxPhaQr8AV7dYxaOrxXUAAAAAElFTkSuQmCC";

pub const NAME: &str = "OPENEDU101";
pub const SYMBOL: &str = "EDU101";
pub const DECIMALS: u8 = 18;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct FungibleTokenMetadata {
    pub spec: String, // Should be ft-1.0.0 to indicate that a Fungible Token contract adheres to the current versions of this Metadata and the Fungible Token Core specs. This will allow consumers of the Fungible Token to know if they support the features of a given contract.
    pub name: String, // The human-readable name of the token.
    pub symbol: String, // The abbreviation, like wETH or AMPL.
    pub icon: Option<String>, // Icon of the fungible token.
    pub reference: Option<String>, // A link to a valid JSON file containing various keys offering supplementary details on the token
    pub reference_hash: Option<Base64VecU8>, // The base64-encoded sha256 hash of the JSON file contained in the reference field. This is to guard against off-chain tampering.
    pub decimals: u8, // used in frontends to show the proper significant digits of a token. This concept is explained well in this OpenZeppelin post. https://docs.openzeppelin.com/contracts/3.x/erc20#a-note-on-decimals
}

pub trait FungibleTokenMetadataProvider {
    // View call for returning the contract metadata
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}