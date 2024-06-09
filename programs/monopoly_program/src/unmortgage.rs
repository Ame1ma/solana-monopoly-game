use anchor_lang::prelude::*;

use super::*;

pub fn unmortgage(ctx: Context<Play>, square_position: u8) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let current_player = ctx.accounts.game.current_player;
    require!(call_from == current_player, GameError::NotCurrentPlayer);

    require!(square_position < 16, GameError::PositionOutOfBound);
    if let SquareStatus::Mortgaged { by } = ctx.accounts.game.board_status[square_position as usize]
    {
        require!(by == current_player, GameError::NotOwned);
    } else {
        return err!(GameError::NotMortgaged);
    };

    let mortgage_value = Game::BOARD_INFO[square_position as usize].house_price / 2;
    let new_balance = ctx.accounts.game.players[current_player.as_index()]
        .balance
        .checked_sub(mortgage_value + (mortgage_value / 10))
        .ok_or(GameError::BalanceNotEnough)?;

    ctx.accounts.game.players[current_player.as_index()].balance = new_balance;
    ctx.accounts.game.board_status[square_position as usize] = SquareStatus::Owned {
        by: current_player,
        level: 0,
    };
    Ok(())
}
