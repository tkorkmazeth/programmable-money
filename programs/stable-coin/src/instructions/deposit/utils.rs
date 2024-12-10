use crate::SEED_MINT_ACCOUNT;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::{
    token_2022::{mint_to, MintTo},
    token_interface::{Mint, Token2022, TokenAccount},
};

pub fn mint_tokens_internal<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    token_accoınt: &InterfaceAccount<'info, TokenAccount>,
    token_program: &InterfaceAccount<'info, Token2022>,
    bump: u8,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint_account.to_account_info(),
                to: token_accoınt.to_account_info(),
                authority: mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}
