use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, StorageUsage, BorshStorageKey, IntoStorageKey};

pub mod nearderthal_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;


use crate::metadata::*;
use crate::events::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    pub accounts: LookupMap<AccountId, u128>,
    pub total_supply: u128,
    pub metadata: LazyOption<FungibleTokenMetadata>,
    /// The bytes for the largest possible account ID that can be registered on the contract
    pub bytes_for_longest_account_id: StorageUsage,
    pub points: UnorderedMap<AccountId, u64>,
    pub admin: AccountId,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKey {
    Accounts,
    Metadata,
    Points,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: NAME.to_string(),
                symbol: SYMBOL.to_string(),
                icon: Some(DATA_IMAGE_SVG.to_string()),
                reference: None,
                reference_hash: None,
                decimals: DECIMALS,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized.
        let mut this = Self {
            total_supply: total_supply.0,
            bytes_for_longest_account_id: 0,
            metadata: LazyOption::new(
                StorageKey::Metadata.into_storage_key(),
                Some(&metadata),
            ),
            accounts: LookupMap::new(StorageKey::Accounts.into_storage_key()),
            points: UnorderedMap::new(StorageKey::Points.into_storage_key()),
            admin: owner_id.clone(),
        };

        this.measure_bytes_for_longest_account_id();

        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());

        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
            .emit();

        this
    }

    #[payable]
    pub fn count_point(&mut self) {
        let signer = env::signer_account_id();
        let current_points = self.points.get(&signer).unwrap_or(0);
        self.points.insert(&signer, &(current_points + 5));
    }

    pub fn get_point_by_accountid(&self, account_id: AccountId) -> u64 {
        self.points.get(&account_id).unwrap_or(0)
    }

    #[payable]
    pub fn distribute_tokens(&mut self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.admin,
            "Only the admin can call this function"
        );
        assert_eq!(
            self.total_supply,
            self.accounts.get(&self.admin).unwrap_or(0),
            "Tokens have already been distributed"
        );
    
        let mut total_points = 0;
        let mut distribution: Vec<(AccountId, u128)> = Vec::new();
    
        // First pass: calculate total points and prepare distribution
        for (account, points) in self.points.iter() {
            total_points += points;
            distribution.push((account, points as u128));
        }
    
        // Second pass: perform transfers
        for (account, points) in distribution {
            let token_amount = (points as u128) * self.total_supply / (total_points as u128);
            let admin_balance = self.accounts.get(&self.admin).unwrap_or(0);
            let recipient_balance = self.accounts.get(&account).unwrap_or(0);
    
            assert!(admin_balance >= token_amount, "Insufficient balance for transfer");
    
            self.accounts.insert(&self.admin, &(admin_balance - token_amount));
            self.accounts.insert(&account, &(recipient_balance + token_amount));
    
            FtTransfer {
                old_owner_id: &self.admin,
                new_owner_id: &account,
                amount: &U128(token_amount),
                memo: Some("Token distribution based on points"),
            }
            .emit();
        }
    
        // Create a dummy AccountId for the final event
        let dummy_account_id: AccountId = "multiple_accounts.near".parse().unwrap();
    
        FtTransfer {
            old_owner_id: &self.admin,
            new_owner_id: &dummy_account_id,
            amount: &U128(self.total_supply),
            memo: Some("Total token distribution based on points"),
        }
        .emit();
    }
}