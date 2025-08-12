use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::{
    state::Gossip,
    errors::GossipError,
};

#[derive(Accounts)]
pub struct RevealGossip<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"gossip", gossip.maker.as_ref(), &gossip.index.to_le_bytes()],
        bump = gossip.bump,
    )]
    pub gossip: Account<'info, Gossip>,

    /// CHECK: This is a PDA vault that receives SOL payments. Seeds and bump are validated.
    #[account(
        mut,
        seeds = [b"vault", gossip.key().as_ref()],
        bump = gossip.vault_bump,
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

/*
* @description: Reveal a gossip by paying the creator a fee
* @param ctx: Context of the instruction
* @return: Result of the instruction
*/
pub fn reveal_gossip(ctx: Context<RevealGossip>) -> Result<()> {
    let gossip = &mut ctx.accounts.gossip;
    let buyer = &ctx.accounts.buyer;
    let vault = &ctx.accounts.vault;
    
    if gossip.is_revealed {
        return Err(GossipError::GossipAlreadyRevealed.into());
    }

    let price = gossip.price;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        price,
    )?;
    
    gossip.is_revealed = true;
    
    msg!("Gossip revealed!");
    Ok(())
}