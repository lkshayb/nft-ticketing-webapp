use anchor_lang::prelude::*;

declare_id!("23pYtn2tDzjCNe97eZm3Aerp8FDdXM9qupPxua6ASwvA");

#[program]
pub mod nft_webapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
