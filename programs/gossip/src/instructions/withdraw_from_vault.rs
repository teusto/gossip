use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    pub system_program: Program<'info, System>,
}

pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>) -> Result<()> {
    Ok(())
}