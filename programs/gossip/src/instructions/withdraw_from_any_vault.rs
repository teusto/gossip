use anchor_lang::prelude::*;

use crate::{
    state::GossipVault,
    errors::GossipError,
};

#[derive(Accounts)]
pub struct WithdrawFromAnyVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        constraint = vault.owner == owner.key() @ GossipError::UnauthorizedWithdraw,
        close = destination
    )]
    pub vault: Account<'info, GossipVault>,

    /// CHECK: This is the destination account for withdrawal
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_from_any_vault(ctx: Context<WithdrawFromAnyVault>) -> Result<()> {
    let vault = &ctx.accounts.vault;
    
    // Get total lamports in the vault (including rent)
    let total_lamports = vault.to_account_info().lamports();
    
    // The close constraint will automatically transfer all lamports to destination
    // and close the account, so we don't need manual transfer
    
    msg!("Withdrawn {} lamports and closed vault for owner: {}", 
         total_lamports, vault.owner);
    Ok(())
}
