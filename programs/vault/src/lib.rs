use anchor_lang::prelude::*;

declare_id!("FTmr8VWWi79hQBVVeJWseYcv42YgNGVAhTSgW8qdw63N");

#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);

        vault.balance -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub vault: Account<'info, VaultData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultData>,
    pub user: Signer<'info>,
}

#[account]
pub struct VaultData {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
