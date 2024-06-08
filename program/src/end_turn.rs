use anchor_lang::prelude::*;

use super::*;

pub fn end_turn(ctx: Context<Play>) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let current_player = ctx.accounts.game.current_player;
    require!(call_from == current_player, GameError::NotCurrentPlayer);

    let next_player = match current_player {
        WhichPlayer::PlayerOne => WhichPlayer::PlayerTwo,
        WhichPlayer::PlayerTwo => WhichPlayer::PlayerOne,
    };

    let position = ctx.accounts.game.players[current_player.as_index()].position;
    let square_info = &Game::BOARD_INFO[position as usize];
    match ctx.accounts.game.board_status[position as usize] {
        SquareStatus::Unowned => {
            ctx.accounts.game.player_status = PlayerStatus::Action(Bid {
                from: next_player,
                value: 0,
            });
            return Ok(());
        }
        SquareStatus::Owned { by, level } if by != current_player => {
            let rent = if level == 0 {
                let is_monopoly = Game::BOARD_INFO
                    .iter()
                    .enumerate()
                    .filter_map(|(position, square)| {
                        square.color.eq(&square_info.color).then_some(position)
                    })
                    .all(|position| match ctx.accounts.game.board_status[position] {
                        SquareStatus::Owned { by, level: _ } if by == next_player => true,
                        SquareStatus::Mortgaged { by } if by == next_player => true,
                        _ => false,
                    });
                is_monopoly
                    .then(|| square_info.rent[0] * 2)
                    .unwrap_or_else(|| square_info.rent[0])
            } else {
                square_info.rent[level as usize]
            };
            let Some(new_balance) = ctx.accounts.game.players[current_player.as_index()]
                .balance
                .checked_sub(rent)
            else {
                ctx.accounts.game.player_status = PlayerStatus::Lose;
                return Ok(());
            };
            ctx.accounts.game.players[current_player.as_index()].balance = new_balance;
        }
        _ => (),
    }

    ctx.accounts.game.current_player = next_player;
    ctx.accounts.game.player_status = PlayerStatus::BeforeMoving;
    Ok(())
}
