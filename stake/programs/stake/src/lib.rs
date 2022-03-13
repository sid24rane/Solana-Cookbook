use anchor_lang::prelude::*;
use anchor_lang::Id;
use anchor_spl::token::{Mint, TokenAccount, Token};
use anchor_spl::{associated_token::get_associated_token_address, token::Transfer, token::mint_to};

declare_id!("Xyq5eax9FevQWUNzwmq22CidMExNpR3V2KJD76Rxkqqq");

#[program]
pub mod stake {

    use anchor_spl::token::{transfer, MintTo};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let ass_key =
            get_associated_token_address(&ctx.accounts.stake_pda.key(), &ctx.accounts.mint.key());
        ctx.accounts.state.mint = ctx.accounts.mint.mint_authority.unwrap();
        ctx.accounts.state.associated_account = ctx.accounts.associated_acc.key();
        Ok(())
    }
    
    pub fn stake(ctx: Context<Stake>, amt: u64) -> Result<()> {
        ctx.accounts.stake_info.owner = ctx.accounts.owner.key();
        ctx.accounts.stake_info.amt = amt;
        let t = Transfer{
           from:ctx.accounts.user_associated_acc.to_account_info(),
           to:ctx.accounts.program_associated_acc.to_account_info(),
           authority:ctx.accounts.stake_pda.clone(),
        };
        let c = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        transfer(c, amt)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let owner = ctx.accounts.stake_info.owner;
        let amount = ctx.accounts.stake_info.amt;
        let t = Transfer{
           from:ctx.accounts.program_associated_acc.to_account_info(),
           to:ctx.accounts.user_associated_acc.to_account_info(),
           authority:ctx.accounts.stake_pda.clone(),
        };
        let c = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        transfer(c, amount)?;
        
        let m = MintTo {
            mint:ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_associated_acc.to_account_info(),
            authority: ctx.accounts.stake_pda.clone(),
        };
        let stake_bump = ctx.bumps["stake_pda"];
        let seeds: &[&[&[u8]]] = &[&[b"stake".as_ref(),&[stake_bump]]];
        let c = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), m, seeds);
        mint_to(c, (amount as f64 *0.20) as u64)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, payer = signer, seeds=[b"state"], bump)]
    state: Account<'info, State>,
    mint: Account<'info, Mint>,
    stake_pda: AccountInfo<'info>,
    associated_acc: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(seeds=[b"state"], bump)]
    state: Account<'info, State>,
    #[account(init, payer = owner, seeds=[owner.key().as_ref()], bump)]
    stake_info: Account<'info, StakeInfo>,
    stake_pda: AccountInfo<'info>,
    #[account(mut)]
    user_associated_acc: Account<'info, TokenAccount>,
    #[account(mut)]
    program_associated_acc: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    mint: Account<'info, Mint>,
    #[account(seeds=[b"state"], bump)]
    state: Account<'info, State>,
    #[account(seeds=[owner.key().as_ref()], bump)]
    stake_info: Account<'info, StakeInfo>,
    #[account(seeds=[b"stake"], bump, mut)]
    stake_pda: AccountInfo<'info>,
    #[account(mut)]
    user_associated_acc: Account<'info, TokenAccount>,
    #[account(mut)]
    program_associated_acc: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>
}

#[account]
#[derive(Default)]
pub struct State {
    mint: Pubkey,
    associated_account: Pubkey,
}

#[account]
#[derive(Default)]
pub struct StakeInfo {
    owner: Pubkey,
    amt: u64,
}