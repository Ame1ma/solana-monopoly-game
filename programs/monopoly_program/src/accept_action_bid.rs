use anchor_lang::prelude::*;

use super::*;

pub fn accept_action_bid(ctx: Context<Play>) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let (bid_from, bid_value) = if let PlayerStatus::Action(bid) = &ctx.accounts.game.player_status
    {
        (bid.from, bid.value)
    } else {
        return err!(GameError::NotInAction);
    };
    require!(call_from != bid_from, GameError::WaitAnotherPlayerBid);

    let Some(new_balance) = ctx.accounts.game.players[bid_from.as_index()]
        .balance
        .checked_sub(bid_value)
    else {
        ctx.accounts.game.current_player = bid_from;
        ctx.accounts.game.player_status = PlayerStatus::Lose;
        return Ok(());
    };

    let current_player = ctx.accounts.game.current_player;
    let position = ctx.accounts.game.players[current_player.as_index()].position;
    ctx.accounts.game.players[bid_from.as_index()].balance = new_balance;
    ctx.accounts.game.board_status[position as usize] = SquareStatus::Owned {
        by: bid_from,
        level: 0,
    };

    let next_player = match current_player {
        WhichPlayer::PlayerOne => WhichPlayer::PlayerTwo,
        WhichPlayer::PlayerTwo => WhichPlayer::PlayerOne,
    };
    ctx.accounts.game.current_player = next_player;
    ctx.accounts.game.player_status = PlayerStatus::BeforeMoving;
    Ok(())
}
