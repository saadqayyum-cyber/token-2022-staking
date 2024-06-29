use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};

declare_id!("GrpKuGPVTNjUCTqfJRMKpph7XZdfuBcc1cTLfRdSm3Xv");

#[program]
pub mod token_2022_staking {
    use super::*;

    // 1. Initialize
    pub fn initialize(ctx: Context<Initialize>, min_stake_period: i64) -> Result<()> {
        let config = &mut ctx.accounts.config;

        config.authority = *ctx.accounts.authority.key;
        config.min_stake_period = min_stake_period;
        config.reward_balance = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 8 + 8
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [CONFIG_ATA_SEED.as_ref()],
        bump,
        payer = authority,
        token::mint = token_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub config_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// PDAs
#[account]
pub struct Config {
    pub authority: Pubkey,     // 32
    pub min_stake_period: i64, // 8
    pub reward_balance: u64,   // 8
}

#[account]
pub struct User {
    pub authority: Pubkey,
    pub stakes: Vec<Stake>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Stake {
    pub amount: u64,
    pub timestamp: i64,
}

pub const CONFIG_PDA_SEED: &[u8] = b"config-pda-1";
pub const CONFIG_ATA_SEED: &[u8] = b"config-ata-1";
