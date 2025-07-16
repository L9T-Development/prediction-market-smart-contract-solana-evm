use anchor_lang::prelude::*;

#[event]
pub struct GlobalUpdateEvent {
    pub global_authority: Pubkey,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub mint_decimals: u8,
}

#[event]
pub struct CreateEvent {
    pub creator: Pubkey,
    pub market: Pubkey,

    pub token_yes: Pubkey,
    pub metadata_yes: Pubkey,
    pub token_yes_total_supply: u64,
    pub real_yes_sol_reserves: u64,

    pub token_no: Pubkey,
    pub metadata_no: Pubkey,
    pub token_no_total_supply: u64,
    pub real_no_sol_reserves: u64,

    pub start_slot: u64,
    pub ending_slot: u64,
}

#[event]
pub struct WithdrawEvent {
    pub withdraw_authority: Pubkey,
    pub mint: Pubkey,
    pub fee_vault: Pubkey,

    pub withdrawn: u64,
    pub total_withdrawn: u64,

    pub withdraw_time: i64,
}

#[event]
pub struct TradeEvent {
    pub user: Pubkey,
    pub token_yes: Pubkey,
    pub token_no: Pubkey,
    pub market_info: Pubkey,

    pub sol_amount: u64,
    pub token_amount: u64,
    pub fee_lamports: u64,
    pub is_buy: bool,
    pub is_yes_no: bool,

    pub real_sol_reserves: u64,
    pub real_token_yes_reserves: u64,
    pub real_token_no_reserves: u64,

    pub timestamp: i64,
}

#[event]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub timestamp: i64,
}

pub trait IntoEvent<T: anchor_lang::Event> {
    fn into_event(&self) -> T;
}
