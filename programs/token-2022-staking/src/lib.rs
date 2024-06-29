use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022_extensions::transfer_fee::{transfer_checked_with_fee, TransferCheckedWithFee},
    token_interface::{Mint, Token2022, TokenAccount},
};

declare_id!("GrpKuGPVTNjUCTqfJRMKpph7XZdfuBcc1cTLfRdSm3Xv");

#[program]
pub mod token_2022_staking {
    use super::*;

    /**
     *! 1. INITIALIZE
     */
    pub fn initialize(ctx: Context<Initialize>, min_stake_period: i64) -> Result<()> {
        let config = &mut ctx.accounts.config;

        config.authority = *ctx.accounts.authority.key;
        config.min_stake_period = min_stake_period;
        config.reward_balance = 0;

        Ok(())
    }

    /**
     *! 2. UPDATE MINIMUM STAKE PERIOD (Admin Only)
     */
    pub fn update_min_stake_period(
        ctx: Context<UpdateMinStakePeriod>,
        min_stake_period: i64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;

        require!(
            *ctx.accounts.authority.key == config.authority,
            ErrorCode::Unauthorized
        );

        config.min_stake_period = min_stake_period;

        Ok(())
    }

    /**
     *! 3. DEPOSIT TOKENS INTO CONTRACT
     */

    pub fn deposit_rewards(
        ctx: Context<DepositRewards>,
        amount: u64,
        token_tax_percentage: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;

        // Transfer tokens from the depositor to the contract's reward account
        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            source: ctx.accounts.depositor_ata.to_account_info(),
            destination: ctx.accounts.config_ata.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let fee = (amount as f64 * (token_tax_percentage as f64 / 100.0)) as u64;

        transfer_checked_with_fee(cpi_ctx, amount, 9, fee)?;

        // Update the reward balance
        config.reward_balance += amount;

        Ok(())
    }

    /**
     *! 4. WITHDRAW TOKENS FROM CONTRACT (Admin Only)
     */

    pub fn withdraw(
        ctx: Context<Withdraw>,
        token_tax_percentage: u64,
        config_pda_bump: u8,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;

        require!(
            *ctx.accounts.authority.key == config.authority,
            ErrorCode::Unauthorized
        );

        // Transfer Tokens from contract account to user account
        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            source: ctx.accounts.config_ata.to_account_info(),
            destination: ctx.accounts.authority_ata.to_account_info(),
            authority: config.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let seeds: &[&[&[u8]]] = &[&[CONFIG_PDA_SEED, &[config_pda_bump]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);

        let total_reward_balance = config.reward_balance;
        let fee = (total_reward_balance as f64 * (token_tax_percentage as f64 / 100.0)) as u64;

        transfer_checked_with_fee(cpi_ctx, total_reward_balance, 9, fee)?;

        Ok(())
    }

    /**
     *! 5. STAKE
     */
    pub fn stake(ctx: Context<Stake>, amount: u64, token_tax_percentage: u64) -> Result<()> {
        let user_stake_account = &mut ctx.accounts.user_stake_account;

        // Transfer Tokens from user account to contract account
        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            source: ctx.accounts.user_ata.to_account_info(),
            destination: ctx.accounts.config_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let fee = (amount as f64 * (token_tax_percentage as f64 / 100.0)) as u64;

        transfer_checked_with_fee(cpi_ctx, amount, 9, fee)?;

        user_stake_account.stakes.push(StakeRecord {
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     *! 6. UNSTAKE
     */

    pub fn unstake(
        ctx: Context<Unstake>,
        token_tax_percentage: u64,
        config_pda_bump: u8,
    ) -> Result<()> {
        let user_stake_account = &mut ctx.accounts.user_stake_account;
        let config = &mut ctx.accounts.config;

        let current_time = Clock::get()?.unix_timestamp;
        let annual_rate = 0.15; // APY of 15%
        let seconds_in_year = 365.0 * 24.0 * 3600.0;
        let second_rate = annual_rate / seconds_in_year; // Rate per second

        let mut total_reward = 0.0;

        for stake in user_stake_account.stakes.iter_mut() {
            let duration = current_time - stake.timestamp;
            if duration >= config.min_stake_period {
                let reward_duration = duration as f64; // Duration in seconds
                total_reward += stake.amount as f64 * second_rate * reward_duration;
            }
        }

        // Convert total_reward to u64
        let token_decimals = 9;
        let total_reward_u64 = (total_reward * 10u64.pow(token_decimals as u32) as f64) as u64;

        // Ensure there are enough rewards in the contract
        if config.reward_balance < total_reward_u64 {
            return Err(ErrorCode::InsufficientRewards.into());
        }

        // Update reward balance in state
        config.reward_balance -= total_reward_u64;

        // Transfer Tokens from contract account to user account
        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            source: ctx.accounts.config_ata.to_account_info(),
            destination: ctx.accounts.user_ata.to_account_info(),
            authority: ctx.accounts.config.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let seeds: &[&[&[u8]]] = &[&[CONFIG_PDA_SEED, &[config_pda_bump]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);

        let fee = (total_reward_u64 as f64 * (token_tax_percentage as f64 / 100.0)) as u64;

        transfer_checked_with_fee(cpi_ctx, total_reward_u64, 9, fee)?;

        // Remove all stakes
        user_stake_account.stakes.clear();

        Ok(())
    }

    /**
     *! 7. CLAIM REWARDS
     */

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        token_tax_percentage: u64,
        config_pda_bump: u8,
    ) -> Result<()> {
        let user_stake_account = &mut ctx.accounts.user_stake_account;
        let config = &mut ctx.accounts.config;

        let current_time = Clock::get()?.unix_timestamp;
        let annual_rate = 0.15; // APY of 15%
        let seconds_in_year = 365.0 * 24.0 * 3600.0;
        let second_rate = annual_rate / seconds_in_year; // Rate per second

        let mut total_reward = 0.0;

        for stake in user_stake_account.stakes.iter_mut() {
            let duration = current_time - stake.timestamp;
            if duration >= config.min_stake_period {
                let reward_duration = duration as f64; // Duration in seconds
                total_reward += stake.amount as f64 * second_rate * reward_duration;
                stake.timestamp = current_time;
            }
        }

        // Convert total_reward to u64
        let token_decimals = 9;
        let total_reward_u64 = (total_reward * 10u64.pow(token_decimals as u32) as f64) as u64;

        // Ensure there are enough rewards in the contract
        if config.reward_balance < total_reward_u64 {
            return Err(ErrorCode::InsufficientRewards.into());
        }

        // Update reward balance in state
        config.reward_balance -= total_reward_u64;

        // Transfer Tokens from contract account to user account
        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            source: ctx.accounts.config_ata.to_account_info(),
            destination: ctx.accounts.user_ata.to_account_info(),
            authority: ctx.accounts.config.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let seeds: &[&[&[u8]]] = &[&[CONFIG_PDA_SEED, &[config_pda_bump]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);

        let fee = (total_reward_u64 as f64 * (token_tax_percentage as f64 / 100.0)) as u64;

        transfer_checked_with_fee(cpi_ctx, total_reward_u64, 9, fee)?;

        Ok(())
    }
}

// ----------------------------------------------------------------------------------------------
//                                  Instruction Structs
// ----------------------------------------------------------------------------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        payer = authority,
        space = Config::LEN
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

#[derive(Accounts)]
pub struct UpdateMinStakePeriod<'info> {
    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        mut
    )]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DepositRewards<'info> {
    #[account(
        seeds = [CONFIG_ATA_SEED.as_ref()],
        bump,
        mut
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        mut,
        token::mint = token_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub config_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = token_mint,
        // token::authority = depositor,
        token::token_program = token_program,
    )]
    pub depositor_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub depositor: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        seeds = [CONFIG_ATA_SEED.as_ref()],
        bump,
        mut
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        mut,
        token::mint = token_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub config_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        seeds = [user.key().as_ref()],
        bump,
        payer = user,
        space = UserStakeAccount::LEN,
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        seeds = [CONFIG_ATA_SEED.as_ref()],
        bump,
        mut
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        mut,
        token::mint = token_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub config_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [user.key().as_ref()],
        bump,
        mut
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        seeds = [CONFIG_ATA_SEED.as_ref()],
        bump,
        mut
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        mut,
        token::mint = token_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub config_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [user.key().as_ref()],
        bump,
        mut
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [CONFIG_ATA_SEED.as_ref()],
        bump,
        mut
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        mut,
        token::mint = token_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub config_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = authority,
        token::token_program = token_program,
    )]
    pub authority_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

// ----------------------------------------------------------------------------------------------
//                                  PDAs
// ----------------------------------------------------------------------------------------------

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub min_stake_period: i64,
    pub reward_balance: u64,
}

impl Config {
    const DISCRIMINATOR: usize = 8;
    pub const LEN: usize = Self::DISCRIMINATOR + 32 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct StakeRecord {
    pub amount: u64,
    pub timestamp: i64,
}

impl StakeRecord {
    pub const LEN: usize = 8 + 8;
}

#[account]
pub struct UserStakeAccount {
    pub authority: Pubkey,
    pub stakes: Vec<StakeRecord>,
}

impl UserStakeAccount {
    const DISCRIMINATOR: usize = 8;
    const VECTOR_LENGTH_PREFIX: usize = 4;
    const STAKE_RECORD_COUNT: usize = 1; // For more than 1 stake record at a time, user will pay

    pub const LEN: usize = Self::DISCRIMINATOR
        + 32
        + (Self::VECTOR_LENGTH_PREFIX + (StakeRecord::LEN * Self::STAKE_RECORD_COUNT));
}

// ----------------------------------------------------------------------------------------------
//                                  ERRORS
// ----------------------------------------------------------------------------------------------

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized, // 6000
    #[msg("Insufficient rewards in the contract account")]
    InsufficientRewards, // 6001
}

// ----------------------------------------------------------------------------------------------
//                                  SEEDS
// ----------------------------------------------------------------------------------------------

pub const CONFIG_PDA_SEED: &[u8] = b"config-pda-1";
pub const CONFIG_ATA_SEED: &[u8] = b"config-ata-1";
