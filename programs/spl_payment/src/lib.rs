mod errors;
mod events;
mod instructions;
mod state;
mod constants;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("EL7MGw7uNeAbgU5kEp2WnKQ2BtJb2Coopnr5U8HAZM9N");

#[program]
pub mod spl_payment {
    use super::*;
    // owner functions
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn update_owner(ctx: Context<SetData>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }

    //  user function
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
