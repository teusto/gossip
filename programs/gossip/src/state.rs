use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Gossip {
    pub maker: Pubkey, // The creator of the gossip
    #[max_len(20)]
    pub text: String, // The text of the gossip
    pub mention: Pubkey, // The person mentioned in the gossip
    pub is_revealed: bool, // Whether the gossip has been revealed
    pub index: u64, // The index of the gossip
    pub vault: Pubkey, // The vault where the funds are stored
    pub price: u64, // The price of the gossip
    pub total_collected: u64, // The total amount collected
    pub bump: u8, // Bump for the PDA
    pub vault_bump: u8, // Bump for the vault PDA
}