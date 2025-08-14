use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::{
    state::{Gossip, GossipVault},
    errors::GossipError,
};

#[derive(Accounts)]
pub struct RevealGossip<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub gossip: Account<'info, Gossip>,

    #[account(
        init,
        payer = buyer,
        space = 8 + GossipVault::INIT_SPACE,
        seeds = [b"gossip_vault", gossip.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, GossipVault>,

    pub system_program: Program<'info, System>,
}

/*
* @description: Reveal a gossip by paying the creator a fee
* @param ctx: Context of the instruction
* @return: Result of the instruction
*/
pub fn reveal_gossip(ctx: Context<RevealGossip>) -> Result<()> {
    let buyer = &ctx.accounts.buyer;
    let gossip = &mut ctx.accounts.gossip;
    let vault = &mut ctx.accounts.vault;

    if gossip.is_revealed {
        return Err(GossipError::GossipAlreadyRevealed.into());
    }

    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: buyer.to_account_info(),
        to: vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    system_program::transfer(cpi_ctx, gossip.price)?;
        

    gossip.is_revealed = true;
    vault.owner = gossip.maker;
    vault.amount = gossip.price;
    gossip.total_collected += ctx.accounts.vault.amount;

    msg!("Gossip revealed!");
    Ok(())
}