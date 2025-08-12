use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use errors::*;

declare_id!("8afyMAB2tiA8a6M9KMYgWfcrLK5nKcbp7NBuqdYxW8kR");

#[program]
pub mod gossip {
    use super::*;

    pub fn create_gossip(_ctx: Context<CreateGossip>, text: String, mention: Pubkey, gossip_index: u64) -> Result<()>{
        instructions::create_gossip(_ctx, text, mention, gossip_index);
        Ok(())
    }

    pub fn reveal_gossip(_ctx: Context<RevealGossip>, gossip_id: u64) -> Result<()> {
        instructions::reveal_gossip(_ctx, gossip_id);
        Ok(())
    }
}
