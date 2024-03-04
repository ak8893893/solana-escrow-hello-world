use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, TokenAccount};

declare_id!("5G1rfh76knLXQJhPuWerjmmoaoCqDmdC4xLHrxSRXYqk");

#[program]
pub mod escrow {
    use anchor_spl::token::{transfer_checked, TransferChecked};
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64) -> Result<()>{
        ctx.accounts.escrow.set_inner(Escrow{
            seed,
            maker: ctx.accounts.maker.key(),
            mint_a: ctx.accounts.mint_a.key(),
            mint_b: ctx.accounts.mint_b.key(),
            receive_amount,
            bump: ctx.bumps.escrow,
        });

        let ctx_accounts = TransferChecked{
            from: ctx.accounts.maker_ata_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            ctx_accounts
        );

        transfer_checked(transfer_ctx, deposit_amount, ctx.accounts.mint_a.decimals)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {

    }



    pub fn refund(ctx: Context<Refund>) -> Result<()>{
        let ctx_accounts = TransferChecked{
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to:  ctx.accounts.maker_ata_a.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };

        let bump:u8 = [ctx.accounts.esc]

        let signer_seeds = &[&[
            b"escrow",
            ctx.accounts.escrow.seed.to_le_bytes().as_ref(),
            escrow.seed.to_le_bytes().as_ref()

        )]]

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            ctx_accounts
        );

        let close_accounts = CloseAccount{

        };

        transfer_checked(transfer_ctx, deposit_amount, ctx.accounts.mint_a.decimals)
    }
}

#[derive(Accounts)]
#[instruction(seed:64)]
pub struct Refund<'info>{  // 跟maker一樣只是沒有mint_b
    #[account(mut)]
    maker: Signer<'info,>,
    mint_a: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    maker_ata_a: Account<'info, TokenAccount>,  // maker assosiate account

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    vault: Account<'info, TokenAccount>,

    #[account(
    seeds = [b"escrow",seed.to_le_bytes().as_ref()],
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        seeds= [
            b"escrow",
            maker.key().to_bytes().as_ref(),
            escrow.seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    escrow: Account<'info, Escrow>,

    assoicated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,

}


#[derive(Accounts)]
#[instruction(seed:64)]
pub struct Make<'info>{
    #[account(mut)]
    maker: Signer<'info,>,
    mint_a: Account<'info, Mint>,
    mint_b: Account<'info, Mint>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    maker_ata_a: Account<'info, TokenAccount>,  // maker assosiate account

    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    vault: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"escrow",seed.to_le_bytes().as_ref()],
        init,
        payer = maker,
        space = Escrow::INIT_SPACE,
        bump
    )]
    escrow: Account<'info, Escrow>
    assoicated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,

}

#[account]
pub struct Escrow{
    seed: u64,  // 用來算PDA program derive address
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    receive_amount: u64,
    bump: u8,  // 用來算一個沒有私鑰的公鑰  避免有人剛好有這個私鑰   如果不用這個算的畫就要給在簽名的時候跟solana說有人拿私鑰要存取的時候不要理他
}

impl Space for Escrow{
    const INIT_SPACE: usize = 8 + 8 + 32 * 3 + 8 + 1;
}