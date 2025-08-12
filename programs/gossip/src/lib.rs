use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("8afyMAB2tiA8a6M9KMYgWfcrLK5nKcbp7NBuqdYxW8kR");

#[program]
pub mod gossip {
    use super::*;

    pub fn create_gossip(_ctx: Context<CreateGossip>, text: String, mention: Pubkey, gossip_index: u64) -> Result<()>{
        instructions::create_gossip(_ctx, text, mention, gossip_index);
        Ok(())
    }
}
