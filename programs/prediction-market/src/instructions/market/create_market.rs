use crate::{
    constants::{CONFIG, GLOBAL, MARKET, METADATA, YES_NAME},
    errors::*,
    events::CreateEvent,
    state::{config::*, market::*},
};
use anchor_lang::{prelude::*, solana_program::sysvar::SysvarId, system_program};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    metadata::{self, mpl_token_metadata::types::DataV2, Metadata},
    token::{self, spl_token::instruction::AuthorityType, Mint, Token},
};

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        mint::decimals = global_config.token_decimals_config,
        mint::authority = global_vault.key(),
    )]
    yes_token: Box<Account<'info, Mint>>,

    pub no_token: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<Market>(),
        seeds = [MARKET.as_bytes(), &yes_token.key().to_bytes(), &no_token.key().to_bytes()],
        bump
    )]
    market: Box<Account<'info, Market>>,

    /// CHECK: passed to token metadata program
    #[account(mut,
        seeds = [
            METADATA.as_bytes(),
            metadata::ID.as_ref(),
            yes_token.key().as_ref(),
        ],
        bump,
        seeds::program = metadata::ID
    )]
    yes_token_metadata_account: UncheckedAccount<'info>,

    /// CHECK: passed to token metadata program
    #[account(
        mut,
        seeds = [
            METADATA.as_bytes(),
            metadata::ID.as_ref(),
            no_token.key().as_ref(),
        ],
        bump,
        seeds::program = metadata::ID
    )]
    no_token_metadata_account: UncheckedAccount<'info>,

    /// CHECK: created in instruction
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            token::spl_token::ID.as_ref(),
            yes_token.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token::ID
    )]
    global_yes_token_account: UncheckedAccount<'info>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
    #[account(address = Rent::id())]
    rent: Sysvar<'info, Rent>,
    #[account(address = token::ID)]
    token_program: Program<'info, Token>,
    #[account(address = associated_token::ID)]
    associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = metadata::ID)]
    mpl_token_metadata_program: Program<'info, Metadata>,

    //  team wallet
    /// CHECK: should be same with the address in the global_config
    #[account(
        mut,
        constraint = global_config.team_wallet == team_wallet.key() @PredictionMarketError::IncorrectAuthority
    )]
    pub team_wallet: UncheckedAccount<'info>,
}

impl<'info> CreateMarket<'info> {
    pub fn handler(&mut self, params: CreateMarketParams, global_vault_bump: u8) -> Result<()> {
        msg!("CreateMarket start");

        //A decentralized prediction market platform built on Solana blockchain, inspired by Polymarket. This project enables users to create markets, trade positions, and resolve outcomes based on real-world events.    
        

        Ok(())
    }
}
