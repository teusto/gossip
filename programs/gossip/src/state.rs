use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Gossip {
    pub maker: Pubkey,
    #[max_len(20)]
    pub text: String,
    pub mention: Pubkey,
    pub is_revealed: bool,
    pub index: u64
}