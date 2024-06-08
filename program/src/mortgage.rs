use anchor_lang::prelude::*;

use super::*;

pub fn mortgage(ctx: Context<Play>, square_position: u8) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let current_player = ctx.accounts.game.current_player;
    require!(call_from == current_player, GameError::NotCurrentPlayer);

    require!(square_position < 16, GameError::PositionOutOfBound);
    let target_square_info = &Game::BOARD_INFO[square_position as usize];

    if let SquareStatus::Owned { by, level: _ } =
        ctx.accounts.game.board_status[square_position as usize]
    {
        require!(by == current_player, GameError::NotOwned);
        Game::BOARD_INFO
            .iter()
            .enumerate()
            .filter_map(|(position, square)| {
                square
                    .color
                    .eq(&target_square_info.color)
                    .then_some(position)
            })
            .map(|position| match ctx.accounts.game.board_status[position] {
                SquareStatus::Owned { by, level } if by == current_player => level,
                _ => 0,
            })
            .all(|level| level == 0)
            .then_some(())
            .ok_or(GameError::LevelNotAllowed)?;
    } else {
        return err!(GameError::NotOwned);
    };

    let mortgage_value = Game::BOARD_INFO[square_position as usize].house_price / 2;
    let new_balance = ctx.accounts.game.players[current_player.as_index()]
        .balance
        .saturating_add(mortgage_value);

    ctx.accounts.game.players[current_player.as_index()].balance = new_balance;
    ctx.accounts.game.board_status[square_position as usize] =
        SquareStatus::Mortgaged { by: current_player };
    Ok(())
}
