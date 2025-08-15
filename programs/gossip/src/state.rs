use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Gossip {
    pub maker: Pubkey, // The creator of the gossip
    #[max_len(20)]
    pub text: String, // The text of the gossip
    pub mention: Pubkey, // The person mentioned in the gossip
    pub is_revealed: bool, // Whether the gossip has been revealed
    pub price: u64, // The price of the gossip
    pub bump: u8, // Bump for the PDA
    pub total_collected: u64 // Total amount collected
}

#[account]
#[derive(InitSpace)]
pub struct GossipVault {
    pub owner: Pubkey,
    pub amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct SharedGossip {
    pub original_gossip: Pubkey,  // Reference to original gossip
    pub sharer: Pubkey,           // User who shared this gossip
    pub original_creator: Pubkey, // Original gossip creator (cached)
    pub is_revealed: bool,        // Whether this share has been revealed
    pub share_price: u64,         // Price to reveal this shared gossip
    pub total_collected: u64,     // Total earnings from this share
    pub bump: u8,                 // PDA bump seed
}