use anchor_lang::prelude::*;
use anchor_spl::{
    assosiated_token::Ass,
    token_interface::{Mint, TokenAccount, TokenInterface}
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>

    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    
    // checks
    #[account(
        mut,
        assosiated_token::mint = token_mint_a,
        assosiated_token::authority = maker,
        assosiated_token::token_program = token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>

    // pda where we store about what we want in exchange for token
    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()];
        bump
    )]
    pub Offer: Accounts<'info, Offer>,

    #[account(
        init,
        payer = maker,
        assosiated_token::mint = token_mint_a,
        assosiated_token::authority = offer,  //program
        assosiated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info. TokenAccount>

    pub system_program: Program<'info, System>,
    pub token_program: InterfaceAccount<'info, TokenAccount>
    pub assosiated_token_program: Program<'info, AssociatedToken>
}

pub fn send_offered_token_to_vault(
    context: &Context<MakeOffer>, 
    token_a_offered_amount:u64
) -> Result<()> {
    transfer_token()
    Ok(())
}
