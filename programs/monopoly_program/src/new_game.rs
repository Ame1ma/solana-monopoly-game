use anchor_lang::prelude::*;

use super::*;

pub fn new_game(ctx: Context<NewGame>, player_two: Pubkey) -> Result<()> {
    let new_game = Game::new(ctx.accounts.player_one.key(), player_two);
    ctx.accounts.game.set_inner(new_game);
    Ok(())
}

#[derive(Accounts)]
#[instruction(player_two: Pubkey)]
pub struct NewGame<'info> {
    #[account(
        init,
        payer = player_one,
        space = 8 + Game::INIT_SPACE,
        seeds = [b"game", player_one.key.as_ref(), &player_two.to_bytes()],
        bump
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}
