use anchor_lang::prelude::*;

use super::*;

pub fn sell_house(ctx: Context<Play>, house_position: u8) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let current_player = ctx.accounts.game.current_player;
    require!(call_from == current_player, GameError::NotCurrentPlayer);

    require!(house_position < 16, GameError::PositionOutOfBound);
    let target_square_info = &Game::BOARD_INFO[house_position as usize];

    let new_square_status = if let SquareStatus::Owned { by, level } =
        ctx.accounts.game.board_status[house_position as usize]
    {
        require!(by == current_player, GameError::NotOwned);
        let allowed_level = Game::BOARD_INFO
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
            .max()
            .unwrap_or(0)
            .saturating_sub(1)
            .max(0);
        let new_level = level.checked_sub(1).ok_or(GameError::LevelNotAllowed)?;
        require!(new_level >= allowed_level, GameError::LevelNotAllowed);
        SquareStatus::Owned {
            by,
            level: new_level,
        }
    } else {
        return err!(GameError::NotOwned);
    };

    let sell_price = Game::BOARD_INFO[house_position as usize].house_price / 2;
    let new_balance = ctx.accounts.game.players[current_player.as_index()]
        .balance
        .saturating_add(sell_price);

    ctx.accounts.game.players[current_player.as_index()].balance = new_balance;
    ctx.accounts.game.board_status[house_position as usize] = new_square_status;

    Ok(())
}
