use anchor_lang::prelude::*;

#[error_code]
pub enum GossipError {
    #[msg("Gossip already revealed")]
    GossipAlreadyRevealed,
    #[msg("Gossip not found")]
    GossipNotFound,
}