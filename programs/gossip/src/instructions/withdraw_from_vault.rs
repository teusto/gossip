use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::{
    state::{Gossip, GossipVault},
    errors::GossipError,
};

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"gossip_vault", gossip.key().as_ref()],
        bump,
        constraint = vault.owner == owner.key() @ GossipError::UnauthorizedWithdraw,
        close = destination
    )]
    pub vault: Account<'info, GossipVault>,

    #[account(mut)]
    pub gossip: Account<'info, Gossip>,

    /// CHECK: This is the destination account for withdrawal
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>) -> Result<()> {
    let vault = &ctx.accounts.vault;
    
    // Get total lamports in the vault (including rent)
    let total_lamports = vault.to_account_info().lamports();
    
    // The close constraint will automatically transfer all lamports to destination
    // and close the account, so we don't need manual transfer
    
    msg!("Withdrawn {} lamports and closed vault", total_lamports);
    Ok(())
}