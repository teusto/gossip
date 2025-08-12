use anchor_lang::prelude::*;

use crate::{
    state::Gossip,
};

#[derive(Accounts)]
#[instruction(text: String, mention: Pubkey, gossip_index: u64, price: u64)]
pub struct CreateGossip<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user, 
        space = 8 + Gossip::INIT_SPACE,
        seeds = [b"gossip", user.key().as_ref(), gossip_index.to_le_bytes().as_ref()],
        bump
    )]
    pub gossip: Account<'info, Gossip>,

    /// CHECK: This is a PDA vault for holding SOL payments
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_gossip(ctx: Context<CreateGossip>, text: String, mention: Pubkey, gossip_index: u64, price: u64) -> Result<()> {
    let gossip = &mut ctx.accounts.gossip;
    let vault_key = ctx.accounts.vault.key(); // Get the key before mutable borrow
    
    // Initialize gossip
    gossip.maker = ctx.accounts.user.key();
    gossip.text = text;
    gossip.mention = mention;
    gossip.is_revealed = false;
    gossip.index = gossip_index;
    gossip.vault = vault_key;
    gossip.price = price;
    gossip.total_collected = 0;
    
    msg!("Gossip created with price: {} lamports", price);
    Ok(())
}