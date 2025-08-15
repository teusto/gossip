use anchor_lang::prelude::*;

use crate::{
    state::{Gossip, SharedGossip},
    errors::GossipError,
};

#[derive(Accounts)]
pub struct ShareGossip<'info> {
    #[account(mut)]
    pub sharer: Signer<'info>,
    
    #[account(
        constraint = original_gossip.is_revealed @ GossipError::GossipNotRevealed
    )]
    pub original_gossip: Account<'info, Gossip>,
    
    #[account(
        init,
        payer = sharer,
        space = 8 + SharedGossip::INIT_SPACE,
        seeds = [b"shared_gossip", original_gossip.key().as_ref(), sharer.key().as_ref()],
        bump
    )]
    pub shared_gossip: Account<'info, SharedGossip>,
    
    pub system_program: Program<'info, System>,
}

/// Calculate share price (80% of original price to incentivize sharing)
fn calculate_share_price(original_price: u64) -> u64 {
    (original_price * 80) / 100
}

pub fn share_gossip(ctx: Context<ShareGossip>) -> Result<()> {
    let shared_gossip = &mut ctx.accounts.shared_gossip;
    let original_gossip = &ctx.accounts.original_gossip;
    let sharer = &ctx.accounts.sharer;
    
    let share_price = calculate_share_price(original_gossip.price);
    
    shared_gossip.original_gossip = original_gossip.key();
    shared_gossip.sharer = sharer.key();
    shared_gossip.original_creator = original_gossip.maker;
    shared_gossip.is_revealed = false;
    shared_gossip.share_price = share_price;
    shared_gossip.total_collected = 0;
    shared_gossip.bump = ctx.bumps.shared_gossip;
    
    msg!("Gossip shared! Original price: {} lamports, Share price: {} lamports", 
         original_gossip.price, share_price);
    
    Ok(())
}
