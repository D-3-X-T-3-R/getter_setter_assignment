use anchor_lang::prelude::*;

declare_id!("434s3aws1dnq4bq4UBzYiVU3qUWKr7B92zXFMtkbXZBS");

#[program]
mod getter_setter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, allowed_program: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.allowed_program = allowed_program;
        base_account.is_access_granted = false;
        Ok(())
    }

    pub fn grant_access(ctx: Context<GrantAccess>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.is_access_granted = true;
        Ok(())
    }

    pub fn revoke_access(ctx: Context<RevokeAccess>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.is_access_granted = false;
        Ok(())
    }

    pub fn set_value(ctx: Context<SetValue>, value: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        require!(
            ctx.accounts.caller_program.key() == base_account.allowed_program,
            MyError::Unauthorized
        );
        require!(
            base_account.is_access_granted,
            MyError::AccessNotGranted
        );
        base_account.value = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1 + 8)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GrantAccess<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeAccess<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub caller_program: Program<'info, System>,
}

#[account]
pub struct BaseAccount {
    pub allowed_program: Pubkey,
    pub is_access_granted: bool,
    pub value: u64,
}

#[error_code]
pub enum MyError {
    #[msg("Unauthorized program")]
    Unauthorized,
    #[msg("Access not granted")]
    AccessNotGranted,
}
