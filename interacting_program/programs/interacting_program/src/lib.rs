use anchor_lang::prelude::*;
use getter_setter::cpi::accounts::SetValue;
use getter_setter::program::GetterSetter;
use getter_setter::{self, BaseAccount};
use program::InteractingProgram;

declare_id!("2q1iXZ2fHMMimgppsFfghRMdBGk3iT9qxDo84Ltk1fwt");

#[program]
mod interacting_program {
    use super::*;

    pub fn set_value(ctx: Context<SetValueInteraction>, value: u64) -> Result<()> {
        let cpi_program = ctx.accounts.getter_setter_program.to_account_info();
        let cpi_accounts = SetValue {
            base_account: ctx.accounts.base_account.to_account_info(),
            caller_program: ctx.accounts.interacting_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        getter_setter::cpi::set_value(cpi_ctx, value)
    }

}

#[derive(Accounts)]
pub struct SetValueInteraction<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub getter_setter_program: Program<'info, GetterSetter>,
    #[account(executable)]
    pub interacting_program: Program<'info, InteractingProgram>,
}
