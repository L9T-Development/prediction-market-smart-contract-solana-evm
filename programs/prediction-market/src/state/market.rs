use crate::errors::PredictionMarketError;
use crate::state::config::*;
use crate::utils::*;

use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

// use anchor_spl::token::{self};

#[account]
pub struct UserInfo {
    pub user: Pubkey,     // User's public key
    pub yes_balance: u64, // Amount of YES tokens purchased
    pub no_balance: u64,  // Amount of NO tokens purchased
    pub is_lp: bool,
    pub is_initialized: bool,
}

#[account]
pub struct LpInfo {
    pub user: Pubkey,    // User's public key
    pub sol_amount: u64, // Amount of init lp
}

#[account]
pub struct Market {
    pub yes_token_mint: Pubkey,
    pub no_token_mint: Pubkey,

    pub creator: Pubkey,

    pub initial_yes_token_reserves: u64,
    pub real_yes_token_reserves: u64,
    pub real_yes_sol_reserves: u64,
    pub token_yes_total_supply: u64,

    pub initial_no_token_reserves: u64,
    pub real_no_token_reserves: u64,
    pub real_no_sol_reserves: u64,
    pub token_no_total_supply: u64,

    pub is_completed: bool,
    pub start_slot: Option<u64>,
    pub ending_slot: Option<u64>,

    pub lps: Vec<LpInfo>,
    pub total_lp_amount: u64,
}

#[derive(Debug, Clone)]
pub struct SellResult {
    pub token_amount: u64,
    pub change_amount: u64,
    pub current_yes_reserves: u64,
    pub current_no_reserves: u64,
    pub new_yes_reserves: u64,
    pub new_no_reserves: u64,
}

#[derive(Debug, Clone)]
pub struct BuyResult {
    pub token_amount: u64,
    pub change_amount: u64,
    pub current_yes_reserves: u64,
    pub current_no_reserves: u64,
    pub new_yes_reserves: u64,
    pub new_no_reserves: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateMarketParams {
    pub yes_symbol: String,
    pub yes_uri: String,

    pub start_slot: Option<u64>,
    pub ending_slot: Option<u64>,
}
pub trait MarketAccount<'info> {
    fn swap(
        &mut self,
        global_config: &Account<'info, Config>,

        yes_token_mint: &Account<'info, Mint>,
        global_yes_ata: &mut AccountInfo<'info>,
        user_yes_ata: &mut AccountInfo<'info>,

        no_token_mint: &Account<'info, Mint>,
        global_no_ata: &mut AccountInfo<'info>,
        user_no_ata: &mut AccountInfo<'info>,

        source: &mut AccountInfo<'info>,
        team_wallet: &mut AccountInfo<'info>,

        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,

        user: &Signer<'info>,
        signer: &[&[&[u8]]],

        user_info_pda: &mut Account<'info, UserInfo>,

        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn apply_buy(&mut self, sol_amount: u64, token_type: u8) -> Option<BuyResult>;

    fn apply_sell(&mut self, token_amount: u64, token_type: u8) -> Option<SellResult>;

    fn get_tokens_for_buy_sol(&self, sol_amount: u64, token_type: u8) -> Option<BuyResult>;

    fn get_tokens_for_sell_sol(&self, token_amount: u64, token_type: u8) -> Option<SellResult>;

    fn resolution(
        &mut self,

        source: &mut AccountInfo<'info>,

        user: &mut AccountInfo<'info>,
        signer: &[&[&[u8]]],
        user_info_pda: &mut Account<'info, UserInfo>,

        token_type: u8,

        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn add_liquidity(
        &mut self,

        source: &mut AccountInfo<'info>,

        user: &Signer<'info>,
        sol_amount: u64,
        user_info_pda: &mut Account<'info, UserInfo>,

        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn withdraw_liquidity(
        &mut self,

        source: &mut AccountInfo<'info>,

        user: &Signer<'info>,
        sol_amount: u64,

        signer: &[&[&[u8]]],
        user_info_pda: &mut Account<'info, UserInfo>,

        system_program: &Program<'info, System>,
    ) -> Result<()>;
}

impl<'info> MarketAccount<'info> for Account<'info, Market> {
    fn swap(
        &mut self,
        global_config: &Account<'info, Config>,

        yes_token_mint: &Account<'info, Mint>,
        global_yes_ata: &mut AccountInfo<'info>,
        user_yes_ata: &mut AccountInfo<'info>,

        no_token_mint: &Account<'info, Mint>,
        global_no_ata: &mut AccountInfo<'info>,
        user_no_ata: &mut AccountInfo<'info>,

        source: &mut AccountInfo<'info>,
        team_wallet: &mut AccountInfo<'info>,

        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,

        user: &Signer<'info>,
        signer: &[&[&[u8]]],

        user_info_pda: &mut Account<'info, UserInfo>,

        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        //A decentralized prediction market platform built on Solana blockchain, inspired by Polymarket. This project enables users to create markets, trade positions, and resolve outcomes based on real-world events.    
        

        Ok(())
    }

    fn get_tokens_for_buy_sol(&self, change_amount: u64, token_type: u8) -> Option<BuyResult> {
        Some(BuyResult {
            token_amount,
            change_amount,
            current_yes_reserves,
            current_no_reserves,
            new_yes_reserves,
            new_no_reserves,
        })
    }

    fn apply_buy(&mut self, change_amount: u64, token_type: u8) -> Option<BuyResult> {
        // Computing Token Amount out
        let result = self.get_tokens_for_buy_sol(change_amount, token_type)?;

        Some(result)
    }

    fn apply_sell(&mut self, change_amount: u64, token_type: u8) -> Option<SellResult> {
        // Computing Sol Amount out
        let result = self.get_tokens_for_sell_sol(change_amount, token_type)?;

        Some(result)
    }

    fn get_tokens_for_sell_sol(&self, change_amount: u64, token_type: u8) -> Option<SellResult> {
        Some(SellResult {
            token_amount,
            change_amount,
            current_yes_reserves,
            current_no_reserves,
            new_yes_reserves,
            new_no_reserves,
        })
    }

    fn resolution(
        &mut self,

        source: &mut AccountInfo<'info>,

        user: &mut AccountInfo<'info>,
        signer: &[&[&[u8]]],
        user_info_pda: &mut Account<'info, UserInfo>,

        token_type: u8,

        system_program: &Program<'info, System>,
    ) -> Result<()> {
        Ok(())
    }

    fn add_liquidity(
        &mut self,

        source: &mut AccountInfo<'info>,

        user: &Signer<'info>,
        sol_amount: u64,

        user_info_pda: &mut Account<'info, UserInfo>,

        system_program: &Program<'info, System>,
    ) -> Result<()> {
        Ok(())
    }

    fn withdraw_liquidity(
        &mut self,

        source: &mut AccountInfo<'info>,

        user: &Signer<'info>,
        sol_amount: u64,
        signer: &[&[&[u8]]],
        user_info_pda: &mut Account<'info, UserInfo>,

        system_program: &Program<'info, System>,
    ) -> Result<()> {
        Ok(())
    }
}
