use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    accept_authority::*, add_liquidity::*, configure::*, create_market::*, mint_no_token::*,
    nominate_authority::*, resolution::*, swap::*, withdraw_liquidity::*,
};

use state::config::*;
use state::market::*;

declare_id!("5q1C8N47AYvLu7w6LKngwXhLjrZCZ5izMB8nbziZhYEV");

#[program]
pub mod prediction_market {
    // use crate::{instruction::AddLiquidity, instructions::resolution::Resolution};

    use super::*;

    //  called by admin to set global config
    //  need to check the signer is authority
    pub fn configure(ctx: Context<Configure>, new_config: Config) -> Result<()> {
        msg!("configure: {:#?}", new_config);
        ctx.accounts.handler(new_config, ctx.bumps.config)
    }

    //  Admin can hand over admin role
    pub fn nominate_authority(ctx: Context<NominateAuthority>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.process(new_admin)
    }

    //  Pending admin should accept the admin role
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn mint_no_token(
        ctx: Context<MintNoToken>,
        // metadata
        no_symbol: String,
        no_uri: String,
    ) -> Result<()> {
        ctx.accounts
            .handler(no_symbol, no_uri, ctx.bumps.global_vault)
    }

    pub fn create_market(ctx: Context<CreateMarket>, params: CreateMarketParams) -> Result<()> {
        ctx.accounts.handler(params, ctx.bumps.global_vault)
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,
    ) -> Result<()> {
        ctx.accounts.handler(
            amount,
            direction,
            token_type,
            minimum_receive_amount,
            ctx.bumps.global_vault,
        )
    }

    pub fn resolution(
        ctx: Context<Resolution>,
        yes_amount: u64,
        no_amount: u64,
        token_type: u8,
        is_completed: bool,
    ) -> Result<()> {
        ctx.accounts.handler(
            yes_amount,
            no_amount,
            token_type,
            is_completed,
            ctx.bumps.global_vault,
        )
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount)
    }

    pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount, ctx.bumps.global_vault)
    }
}
