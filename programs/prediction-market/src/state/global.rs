use crate::{
    errors::PredictionMarketError,
    events::{GlobalUpdateEvent, IntoEvent},
};
use anchor_lang::{prelude::*, solana_program::last_restart_slot::LastRestartSlot};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GlobalAuthorityInput {
    pub global_authority: Option<Pubkey>,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Global {
    pub initialized: bool,
    pub global_authority: Pubkey, // can update settings

    pub team_wallet: Pubkey,

    pub platform_buy_fee: f64, //  platform fee percentage
    pub platform_sell_fee: f64,

    // Prediction Market initial values
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub mint_decimals: u8,

    pub last_updated_slot: u64,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            initialized: true,
            global_authority: Pubkey::default(),
            team_wallet: Pubkey::default(),
            platform_buy_fee: 1.0,
            platform_sell_fee: 1.0,

            // prediction-market initial values
            initial_real_token_reserves: 1000000000000000,
            token_total_supply: 1000000000000000,
            mint_decimals: 6,
            last_updated_slot: 0,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct GlobalSettingsInput {
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub mint_decimals: u8,
    pub team_wallet: Pubkey,
}

impl Global {
    pub const SEED_PREFIX: &'static str = "globalconfig";

    pub fn get_signer<'a>(bump: &'a u8) -> [&'a [u8]; 2] {
        let prefix_bytes = Self::SEED_PREFIX.as_bytes();
        let bump_slice: &'a [u8] = std::slice::from_ref(bump);
        [prefix_bytes, bump_slice]
    }

    pub fn validate_settings(&self, params: &GlobalSettingsInput) -> Result<()> {
        require!(params.mint_decimals <= 9, PredictionMarketError::InvalidParameter);
        require!(
            params.token_total_supply <= u64::MAX / 2,
            PredictionMarketError::InvalidParameter
        );
        require!(
            params.team_wallet != Pubkey::default(),
            PredictionMarketError::InvalidParameter
        );
        require!(
            params.initial_real_token_reserves > 0,
            PredictionMarketError::InvalidParameter
        );

        // Making sure that there is a token amount left for the real token reserves
        require!(
            params.token_total_supply > params.initial_real_token_reserves,
            PredictionMarketError::InvalidParameter
        );
        Ok(())
    }

    pub fn update_settings(&mut self, params: GlobalSettingsInput, slot: u64) {
        self.mint_decimals = params.mint_decimals;
        self.initial_real_token_reserves = params.initial_real_token_reserves;
        self.token_total_supply = params.token_total_supply;
        self.team_wallet = params.team_wallet;

        // Set last updated slot to the slot of the update
        self.last_updated_slot = slot;
    }

    pub fn update_authority(&mut self, params: GlobalAuthorityInput) {
        if let Some(global_authority) = params.global_authority {
            self.global_authority = global_authority;
        }
    }

    pub fn is_config_outdated(&self) -> Result<bool> {
        let last_restart_slot = LastRestartSlot::get()?;
        Ok(self.last_updated_slot <= last_restart_slot.last_restart_slot)
    }
}

impl IntoEvent<GlobalUpdateEvent> for Global {
    fn into_event(&self) -> GlobalUpdateEvent {
        GlobalUpdateEvent {
            global_authority: self.global_authority,
            initial_real_token_reserves: self.initial_real_token_reserves,
            token_total_supply: self.token_total_supply,
            mint_decimals: self.mint_decimals,
        }
    }
}
