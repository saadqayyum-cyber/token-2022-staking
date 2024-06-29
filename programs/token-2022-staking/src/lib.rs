use anchor_lang::prelude::*;

declare_id!("GrpKuGPVTNjUCTqfJRMKpph7XZdfuBcc1cTLfRdSm3Xv");

#[program]
pub mod token_2022_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
