use anchor_lang::prelude::*;

use crate::errors::*;
use crate::events::{DepositEvent, WithdrawEvent};
use crate::state::{GlobalState, UserInfo};
use crate::constants::{ GLOBAL_STATE_SEED, USER_INFO_SEED, VAULT_SEED };
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use std::mem::size_of;


pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;

    accts.global_state.owner = accts.owner.key();
    accts.global_state.token_mint = accts.token_mint.key();
    accts.global_state.vault = accts.token_vault_account.key();

    Ok(())
}

pub fn update_owner(ctx: Context<SetData>, new_owner: Pubkey) -> Result<()> {
    let accts = ctx.accounts;

    if accts.global_state.owner != accts.owner.key() {
        return Err(SplPaymentError::NotAllowedOwner.into());
    }

    accts.global_state.owner = new_owner;

    Ok(())
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    if accts.token_mint.key() != accts.global_state.token_mint {
        return Err(SplPaymentError::InvalidTokenAddress.into());
    }

    if amount == 0 {
        return Err(SplPaymentError::ZeroAmount.into());
    }

    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_owner_account.to_account_info().clone(),
            to: accts.token_vault_account.to_account_info().clone(),
            authority: accts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, amount)?;

    accts.user_info.address = accts.user.key();
    accts.user_info.amount += amount;
    accts.user_info.updated_time = accts.clock.unix_timestamp;

    // Update user_info and get new total staked
    let user_total_staked = accts.user_info.amount;

    // Get current total in vault
    let total_in_vault = accts.token_vault_account.amount;

    // Emitting the enhanced deposit event
    emit!(DepositEvent {
        user: accts.user.key(),
        amount: amount,
        user_total_staked: user_total_staked,
        total_in_vault: total_in_vault,
        timestamp: accts.clock.unix_timestamp,
    });

    Ok(())
}



pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    if accts.token_mint.key() != accts.global_state.token_mint {
        return Err(SplPaymentError::InvalidTokenAddress.into());
    }

    if accts.user_info.amount < amount {
        return Err(SplPaymentError::InvalidAmount.into());
    }

    if amount == 0 {
        return Err(SplPaymentError::ZeroAmount.into());
    }

    let (_, bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], ctx.program_id);
    let vault_seeds = &[GLOBAL_STATE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];

    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_vault_account.to_account_info().clone(),
            to: accts.token_owner_account.to_account_info().clone(),
            authority: accts.global_state.to_account_info().clone(),
        },
    );
    transfer(
        cpi_ctx.with_signer(signer),
        amount,
    )?;

    accts.user_info.amount -= amount;
    accts.user_info.updated_time = accts.clock.unix_timestamp;

    // Update user_info and get new total staked
    let user_total_staked = accts.user_info.amount;

    // Get current total in vault
    let total_in_vault = accts.token_vault_account.amount;

    // Emitting the enhanced withdraw event
    emit!(WithdrawEvent {
        user: accts.user.key(),
        amount: amount,
        user_total_staked: user_total_staked,
        total_in_vault: total_in_vault,
        timestamp: accts.clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<GlobalState>()
    )]
    pub global_state: Account<'info, GlobalState>,

    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = owner,
        seeds = [VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = global_state,
    )]
    pub token_vault_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump,
        space = 8 + size_of::<UserInfo>()
    )]
    pub user_info: Account<'info, UserInfo>,

    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        address = global_state.vault
    )]
    token_vault_account: Account<'info, TokenAccount>,

    #[account(mut)]
    token_owner_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_info: Account<'info, UserInfo>,

    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        address = global_state.vault
    )]
    token_vault_account: Account<'info, TokenAccount>,

    #[account(mut)]
    token_owner_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}
