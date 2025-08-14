use anchor_lang::prelude::*;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{
    state::Gossip,
};

#[derive(Accounts)]
#[instruction(text: String, mention: Pubkey)]
pub struct CreateGossip<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user, 
        space = 8 + Gossip::INIT_SPACE,
        seeds = [b"gossip", user.key().as_ref()],
        bump
    )]
    pub gossip: Account<'info, Gossip>,

    pub system_program: Program<'info, System>
}

pub fn create_gossip(ctx: Context<CreateGossip>, text: String, mention: Pubkey) -> Result<()> {
    let gossip = &mut ctx.accounts.gossip;
    let user_key = &ctx.accounts.user;
    let price = (3 * LAMPORTS_PER_SOL) as u64;
    // Initialize gossip
    gossip.maker = user_key.key();
    gossip.text = text;
    gossip.mention = mention;
    gossip.is_revealed = false;
    gossip.price = price;
    
    msg!("Gossip created with price: {} lamports", price);
    Ok(())
}