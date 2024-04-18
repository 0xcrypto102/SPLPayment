use anchor_lang::prelude::*;

#[event]
pub struct DepositEvent {
    #[index]
    pub user: Pubkey,
    pub amount: u64,
    pub user_total_staked: u64,
    pub total_in_vault: u64,
    pub timestamp: i64,
}

#[event]
pub struct WithdrawEvent {
    #[index]
    pub user: Pubkey,
    pub amount: u64,
    pub user_total_staked: u64,
    pub total_in_vault: u64,
    pub timestamp: i64,
}
