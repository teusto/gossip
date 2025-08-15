use anchor_lang::prelude::*;

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

/// Calculate dynamic price based on text length and mention status
fn calculate_gossip_price(text: &str, has_mention: bool) -> u64 {
    // Base price: 0.01 SOL (10M lamports)
    const BASE_PRICE: u64 = 10_000_000;
    
    // Mention bonus: +0.005 SOL (5M lamports)
    const MENTION_BONUS: u64 = 5_000_000;
    
    // Text tier pricing: +0.002 SOL (2M lamports) per tier
    const CHAR_TIER_PRICE: u64 = 2_000_000;
    const CHARS_PER_TIER: usize = 10;
    
    let mut price = BASE_PRICE;
    
    // Add mention bonus
    if has_mention {
        price += MENTION_BONUS;
    }
    
    // Calculate text tier (0-based)
    // 1-10 chars = tier 0 (no extra cost)
    // 11-20 chars = tier 1 (+0.002 SOL)
    // 21-30 chars = tier 2 (+0.004 SOL)
    let text_length = text.len();
    if text_length > CHARS_PER_TIER {
        let tier = (text_length - 1) / CHARS_PER_TIER; // -1 to make it 0-based
        price += (tier as u64) * CHAR_TIER_PRICE;
    }
    
    price
}

pub fn create_gossip(ctx: Context<CreateGossip>, text: String, mention: Pubkey) -> Result<()> {
    let gossip = &mut ctx.accounts.gossip;
    let user_key = &ctx.accounts.user;
    
    // Check if mentioning someone (not the default/zero pubkey)
    let has_mention = mention != Pubkey::default();
    
    // Calculate dynamic price
    let price = calculate_gossip_price(&text, has_mention);

    gossip.maker = user_key.key();
    gossip.text = text.clone();
    gossip.mention = mention;
    gossip.is_revealed = false;
    gossip.price = price;
    gossip.total_collected = 0;
    
    msg!("Gossip created: '{}' chars, mention: {}, price: {} lamports ({} SOL)", 
         text.len(), has_mention, price, price as f64 / 1_000_000_000.0);
    Ok(())
}