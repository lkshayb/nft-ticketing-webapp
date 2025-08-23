use anchor_lang::prelude::*;

declare_id!("9imnRE1qA6BhgrBdcWMRRSgSeQAcb5L1jyc6gJK6ciWN");

#[program]
pub mod backend_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
