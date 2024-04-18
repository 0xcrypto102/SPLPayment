use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub owner: Pubkey, // the pubkey of owner
    pub token_mint: Pubkey, // the apl_token
    pub vault: Pubkey, // the address of SPL token
}

#[account]
#[derive(Default)]
pub struct UserInfo {
   pub address: Pubkey, // the wallet address
   pub amount: u64, // the amount staked
   pub updated_time: i64, // the last updated time
}

