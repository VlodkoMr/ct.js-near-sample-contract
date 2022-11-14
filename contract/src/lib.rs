use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_sdk::serde::{Deserialize, Serialize};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::collections::{LookupMap, UnorderedMap, LazyOption};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, Timestamp, Balance};

const IPFS_HASH: &str = "bafybeiabr7npvllaoncqmsi5rl6mlaubzvjt2r6sygjzzfec4b6dohtx7e";

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ShipSeries {
    id: u8,
    title: String,
    media: String,
    max_supply: u32,
    minted_total: u32,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ship {
    id: TokenId,
    health: u8,
    attack: u8,
    weapons: u8,
    speed: u8,
    level: u8,
    max_energy: u8,
    current_energy: u32,
    ship_series: u8,
    last_flight: Timestamp,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    total_series: u8,
    total_ships: u128,
    user_balance: LookupMap<AccountId, u128>,
    ships: LookupMap<u128, Ship>,
    user_ships: LookupMap<AccountId, Vec<u128>>,
    ship_series: UnorderedMap<u8, ShipSeries>,
}

const DATA_NEAR_ICON: &str = "data:image/svg+xml,%3C%3Fxml version='1.0' %3F%3E%3Csvg enable-background='new 0 0 64 64' height='64px' version='1.1' viewBox='0 0 64 64' width='64px' xml:space='preserve' xmlns='http://www.w3.org/2000/svg'%3E%3Cg id='Layer_1'%3E%3Cg%3E%3Ccircle cx='32' cy='32' fill='%234F5D73' r='32'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cpath d='M32.096,39.682c-1.556,1.556-6.087,12.54-6.087,12.54c-0.665,1.208,0.33,2.085,2.593,0.936l8.131-4.435 c4.381-2.435,5.228-3.071,5.17-8.028l-0.815-5.853c-0.279-2.005-1.78-2.373-3.336-0.817L32.096,39.682z' fill='%23231F20'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cpath d='M27.072,34.658c-1.556,1.556-12.54,6.087-12.54,6.087c-1.208,0.665-2.085-0.33-0.936-2.593l4.435-8.131 c2.435-4.381,3.071-5.228,8.028-5.17l5.853,0.815c2.005,0.279,2.373,1.78,0.817,3.336L27.072,34.658z' fill='%23231F20'/%3E%3C/g%3E%3Cg%3E%3Cpath d='M32.096,37.682c-1.556,1.556-6.087,12.54-6.087,12.54c-0.665,1.208,0.33,2.085,2.593,0.936l8.131-4.435 c4.381-2.435,5.228-3.071,5.17-8.028l-0.815-5.853c-0.279-2.005-1.78-2.373-3.336-0.817L32.096,37.682z' fill='%23C75C5C'/%3E%3C/g%3E%3Cg%3E%3Cpath d='M27.072,32.658c-1.556,1.556-12.54,6.087-12.54,6.087c-1.208,0.665-2.085-0.33-0.936-2.593l4.435-8.131 c2.435-4.381,3.071-5.228,8.028-5.17l5.853,0.815c2.005,0.279,2.373,1.78,0.817,3.336L27.072,32.658z' fill='%23C75C5C'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cpath d='M21.247,39.545c5.074-5.074,9.337-3.526,9.337-3.526s1.549,4.263-3.526,9.337s-13.291,8.76-13.931,8.12 C12.507,52.855,16.172,44.619,21.247,39.545z' fill='%23231F20'/%3E%3C/g%3E%3Cg%3E%3Cpath d='M21.247,37.545c5.074-5.074,9.337-3.526,9.337-3.526s1.549,4.263-3.526,9.337s-13.291,8.76-13.931,8.12 C12.507,50.855,16.172,42.619,21.247,37.545z' fill='%23E0995E'/%3E%3C/g%3E%3Cg%3E%3Cpath d='M25.489,33.302c5.074-5.074,9.337-3.526,9.337-3.526s1.549,4.263-3.526,9.337s-13.291,8.76-13.931,8.12 C16.749,46.613,20.415,38.376,25.489,33.302z' fill='%23F5CF87'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cpath d='M43.634,37.078c-7.81,7.81-19.783,5.673-19.783,5.673s-2.137-11.972,5.673-19.783 s19.643-8.641,21.197-7.087C52.228,17.388,51.445,29.268,43.634,37.078z' fill='%23231F20'/%3E%3C/g%3E%3Cg%3E%3Cpath d='M43.634,35.078c-7.81,7.81-19.783,5.673-19.783,5.673s-2.137-11.972,5.673-19.783 s19.643-8.641,21.197-7.087C52.228,15.388,51.445,27.268,43.634,35.078z' fill='%23E0E0D1'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cpath d='M21.023,46.994c-0.781,0.781-1.34,1.488-2.121,0.707l0,0c-0.781-0.781-0.074-1.34,0.707-2.121 l9.899-12.728c0.781-0.781,4.169-1.488,4.95-0.707l0,0c0.781,0.781,0.074,4.169-0.707,4.95L21.023,46.994z' fill='%23231F20'/%3E%3C/g%3E%3Cg%3E%3Cpath d='M21.023,44.994c-0.781,0.781-1.34,1.488-2.121,0.707l0,0c-0.781-0.781-0.074-1.34,0.707-2.121 l9.899-12.728c0.781-0.781,4.169-1.488,4.95-0.707l0,0c0.781,0.781,0.074,4.169-0.707,4.95L21.023,44.994z' fill='%23C75C5C'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cellipse cx='39.604' cy='27.15' fill='%23231F20' rx='4' ry='4' transform='matrix(0.7071 -0.7071 0.7071 0.7071 -7.598 35.9566)'/%3E%3C/g%3E%3Cg opacity='0.2'%3E%3Cellipse cx='45.261' cy='21.493' fill='%23231F20' rx='2' ry='2' transform='matrix(0.7071 -0.7071 0.7071 0.7071 -1.9411 38.2997)'/%3E%3C/g%3E%3Cg%3E%3Cellipse cx='39.604' cy='25.15' fill='%234F5D73' rx='4' ry='4' transform='matrix(0.7071 -0.7071 0.7071 0.7071 -6.1838 35.3708)'/%3E%3C/g%3E%3Cg%3E%3Cellipse cx='45.261' cy='19.493' fill='%234F5D73' rx='2' ry='2' transform='matrix(0.7071 -0.7071 0.7071 0.7071 -0.5269 37.7139)'/%3E%3C/g%3E%3C/g%3E%3Cg id='Layer_2'/%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    UserBalance,
    Ships,
    UserShips,
    ShipSeries,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Space Ranger Game".to_string(),
                symbol: "SPG".to_string(),
                icon: Some(DATA_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");

        metadata.assert_valid();
        Self {
            owner_id: env::predecessor_account_id(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            total_series: 0,
            total_ships: 0,
            user_balance: LookupMap::new(StorageKey::UserBalance),
            ships: LookupMap::new(StorageKey::Ships),
            user_ships: LookupMap::new(StorageKey::UserShips),
            ship_series: UnorderedMap::new(StorageKey::ShipSeries),
        }
    }

    pub fn add_ship_series(&mut self, title: String, media: String, max_supply: u32) -> u8 {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
        if title.len() < 1 || media.len() < 1 {
            env::panic_str("All fields is required");
        }

        self.total_series += 1;
        self.ship_series.insert(&self.total_series, &ShipSeries {
            id: self.total_series.clone(),
            title,
            media,
            max_supply,
            minted_total: 0,
        });
        self.total_series
    }

    pub fn get_user_ships(&self, account_id: AccountId) -> Vec<Ship> {
        let id_list = self.user_ships.get(&account_id).unwrap_or(vec![]);
        id_list.into_iter().flat_map(|token_id| self.ships.get(&token_id)).collect()
    }

    pub fn get_user_scores(&self, account_id: AccountId) -> Balance {
        self.user_balance.get(&account_id).unwrap_or(0)
    }

    // For demo only - not safe method!
    pub fn add_user_scores(&mut self, account_id: AccountId, scores: u128) {
        let mut balance = self.user_balance.get(&account_id).unwrap_or(0);
        balance += scores;
        self.user_balance.insert(&account_id, &balance);
    }

    #[payable]
    pub fn nft_mint(&mut self, series_id: u8) -> Token {
        let token_owner_id = env::predecessor_account_id();
        let series = self.ship_series.get(&series_id).expect("Series not found");

        if series.minted_total + 1 > series.max_supply {
            env::panic_str("No ships left, max supply reached");
        }
        if self.user_ships.get(&token_owner_id).unwrap_or(vec![]).len() > 0 {
            env::panic_str("You already have spaceship");
        }
        if env::attached_deposit() < self.convert_to_yocto("0.1") {
            env::panic_str("Wrong payment deposit");
        }

        self.total_ships += 1;
        let token_id = self.total_ships.to_string();

        let token_metadata = TokenMetadata {
            title: Some(series.title.to_string()),
            description: None,
            media: Some(IPFS_HASH.to_string() + "/" + &series.media),
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        self.ships.insert(&self.total_ships, &Ship {
            id: self.total_ships.to_string(),
            health: 10,
            attack: 10,
            weapons: 10,
            speed: 5,
            level: 1,
            max_energy: 10,
            current_energy: 10,
            ship_series: series_id,
            last_flight: 0,
        });
        self.user_ships.insert(&token_owner_id, &vec![self.total_ships]);

        self.tokens.internal_mint(token_id, token_owner_id, Some(token_metadata))
    }

    pub(crate) fn convert_to_yocto(&self, value: &str) -> Balance {
        let values: Vec<_> = value.split('.').collect();
        let part1 = values[0].parse::<u128>().unwrap() * 10u128.pow(24);
        if values.len() > 1 {
            let power = values[1].len() as u32;
            let part2 = values[1].parse::<u128>().unwrap() * 10u128.pow(24 - power);
            part1 + part2
        } else {
            part1
        }
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;

    use super::*;

    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        let token = contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0));
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1));
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    #[test]
    fn test_approve() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }

    #[test]
    fn test_revoke() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke(token_id.clone(), accounts(1));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
    }

    #[test]
    fn test_revoke_all() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke_all(token_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }
}