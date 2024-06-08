use anchor_lang::prelude::*;

use super::*;

pub fn buy_square(ctx: Context<Play>) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let current_player = ctx.accounts.game.current_player;
    require!(call_from == current_player, GameError::NotCurrentPlayer);

    let player_position = ctx.accounts.game.players[current_player.as_index()].position;
    let current_square = &ctx.accounts.game.board_status[player_position as usize];
    require!(
        matches!(current_square, SquareStatus::Unowned),
        GameError::AlreadyOwned
    );

    let square_price = Game::BOARD_INFO[player_position as usize].price;
    let new_balance = ctx.accounts.game.players[current_player.as_index()]
        .balance
        .checked_sub(square_price)
        .ok_or(GameError::BalanceNotEnough)?;

    ctx.accounts.game.players[current_player.as_index()].balance = new_balance;
    Ok(())
}
