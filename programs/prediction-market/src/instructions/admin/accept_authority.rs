use constants::CONFIG;
use errors::PredictionMarketError;

use crate::*;

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    //  Pending admin
    #[account(
        mut,
        constraint = global_config.pending_authority == new_admin.key() @PredictionMarketError::IncorrectAuthority
    )]
    pub new_admin: Signer<'info>,

    //  Stores admin address
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,
}

impl AcceptAuthority<'_> {
    pub fn process(&mut self) -> Result<()> {
        self.global_config.authority = self.new_admin.key();
        self.global_config.pending_authority = Pubkey::default();

        Ok(())
    }
}
