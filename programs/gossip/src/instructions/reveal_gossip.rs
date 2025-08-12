use anchor_lang::prelude::*;

use crate::{
    state::Gossip,
    errors::GossipError,
};

#[derive(Accounts)]
pub struct RevealGossip<'info> {
    #[account(mut)]
    pub gossip: Account<'info, Gossip>,

    pub system_program: Program<'info, System>,
}

/*
* @description: Reveal a gossip by paying the creator a fee
* @param ctx: Context of the instruction
* @param gossip_id: Id of the gossip to reveal
* @return: Result of the instruction
*/
pub fn reveal_gossip(ctx: Context<RevealGossip>, gossip_id: u64) -> Result<()> {
    let gossip = &mut ctx.accounts.gossip;
    
    if gossip.is_revealed {
        return Err(GossipError::GossipAlreadyRevealed.into());
    }
    
    if gossip.index != gossip_id {
        return Err(GossipError::GossipNotFound.into());
    }
    
    gossip.is_revealed = true;
    msg!("Gossip revealed");
    Ok(())
}