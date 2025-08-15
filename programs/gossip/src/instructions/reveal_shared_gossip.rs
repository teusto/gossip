use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::{
    state::{SharedGossip, GossipVault},
    errors::GossipError,
};

#[derive(Accounts)]
pub struct RevealSharedGossip<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        constraint = !shared_gossip.is_revealed @ GossipError::GossipAlreadyRevealed
    )]
    pub shared_gossip: Account<'info, SharedGossip>,

    #[account(
        init,
        payer = buyer,
        space = 8 + GossipVault::INIT_SPACE,
        seeds = [b"creator_vault", shared_gossip.key().as_ref()],
        bump
    )]
    pub creator_vault: Account<'info, GossipVault>,

    #[account(
        init,
        payer = buyer,
        space = 8 + GossipVault::INIT_SPACE,
        seeds = [b"sharer_vault", shared_gossip.key().as_ref()],
        bump
    )]
    pub sharer_vault: Account<'info, GossipVault>,

    pub system_program: Program<'info, System>,
}

/// Revenue split percentages
const ORIGINAL_CREATOR_SHARE: u64 = 60; // 60%
const SHARER_SHARE: u64 = 40;          // 40%

/// Calculate revenue split amounts
fn calculate_revenue_split(total_amount: u64) -> (u64, u64) {
    let creator_amount = (total_amount * ORIGINAL_CREATOR_SHARE) / 100;
    let sharer_amount = (total_amount * SHARER_SHARE) / 100;
    (creator_amount, sharer_amount)
}

pub fn reveal_shared_gossip(ctx: Context<RevealSharedGossip>) -> Result<()> {
    let buyer = &ctx.accounts.buyer;
    let shared_gossip = &mut ctx.accounts.shared_gossip;
    let creator_vault = &mut ctx.accounts.creator_vault;
    let sharer_vault = &mut ctx.accounts.sharer_vault;

    let total_payment = shared_gossip.share_price;
    let (creator_amount, sharer_amount) = calculate_revenue_split(total_payment);

    // Transfer to creator vault
    let creator_cpi_accounts = system_program::Transfer {
        from: buyer.to_account_info(),
        to: creator_vault.to_account_info(),
    };
    let creator_cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        creator_cpi_accounts
    );
    system_program::transfer(creator_cpi_ctx, creator_amount)?;

    // Transfer to sharer vault
    let sharer_cpi_accounts = system_program::Transfer {
        from: buyer.to_account_info(),
        to: sharer_vault.to_account_info(),
    };
    let sharer_cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        sharer_cpi_accounts
    );
    system_program::transfer(sharer_cpi_ctx, sharer_amount)?;

    // Initialize vaults
    creator_vault.owner = shared_gossip.original_creator;
    creator_vault.amount = creator_amount;

    sharer_vault.owner = shared_gossip.sharer;
    sharer_vault.amount = sharer_amount;

    // Update shared gossip state
    shared_gossip.is_revealed = true;
    shared_gossip.total_collected += total_payment;

    msg!("Shared gossip revealed! Creator gets: {} lamports ({}%), Sharer gets: {} lamports ({}%)", 
         creator_amount, ORIGINAL_CREATOR_SHARE, sharer_amount, SHARER_SHARE);

    Ok(())
}
