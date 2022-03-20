use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction::transfer;

declare_id!("q1f5cmwP3PKJiopHA5ADKD6tvMPkuioB4yobEwMftyPOR");

#[program]
pub mod walletsharing {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, friends: [Pubkey; 4]) -> Result<()> {
        ctx.accounts.state.friends = friends;
        ctx.accounts.state.pda_acc = ctx.accounts.spend.key();
        ctx.accounts.state.bump = ctx.bumps["spend"];
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> Result<()> {
        require!(ctx.accounts
                .state
                .friends
                .contains(&ctx.accounts.payer.key()),
            ConstraintOwner
        );
        let t_ix = transfer(
            &ctx.accounts.spend.key(),
            &ctx.accounts.payee.key(),
            amount,
        );
        invoke_signed(
            &t_ix,
            &[
                ctx.accounts.spend.to_account_info(),
                ctx.accounts.payee.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ],
            &[&[b"spend", &[ctx.accounts.state.bump]]],
        )?;
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct State {
    pub friends: [Pubkey; 4],
    pub pda_acc: Pubkey,
    pub bump: u8
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer=signer, seeds=[b"state"], bump)]
    pub state: Account<'info, State>,
    #[account(seeds=[b"spend"], bump)]
    pub spend: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"state"], bump)]
    pub state: Account<'info, State>,
    #[account(seeds=[b"spend"], bump, mut)]
    pub spend: UncheckedAccount<'info>,
    #[account(mut)]
    pub payee: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}


