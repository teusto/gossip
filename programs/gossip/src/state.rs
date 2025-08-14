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