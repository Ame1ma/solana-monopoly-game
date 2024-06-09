use anchor_lang::prelude::*;

use super::*;

pub fn call_from(ctx: &Context<Play>) -> Result<WhichPlayer> {
    let player_one_pubkey = ctx.accounts.game.players[WhichPlayer::PlayerOne as usize].pubkey;
    let player_two_pubkey = ctx.accounts.game.players[WhichPlayer::PlayerTwo as usize].pubkey;
    let call_from = match ctx.accounts.player.key() {
        caller if caller == player_one_pubkey => WhichPlayer::PlayerOne,
        caller if caller == player_two_pubkey => WhichPlayer::PlayerTwo,
        _ => return err!(GameError::NotPlayer),
    };

    Ok(call_from)
}

pub fn need_call_from_current_player(ctx: &Context<Play>, call_from: WhichPlayer) -> Result<()> {
    let current_player = ctx.accounts.game.current_player;
    require!(call_from == current_player, GameError::NotCurrentPlayer);

    Ok(())
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(
        mut,
        seeds = [
            b"game",
            game.players[0].pubkey.as_ref(),
            game.players[1].pubkey.as_ref(),
        ],
        bump
    )]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}
