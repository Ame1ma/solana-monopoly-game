use anchor_lang::prelude::*;

use super::*;

pub fn accept_sell_bid(ctx: Context<Play>) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let (position, bid_value) =
        if let PlayerStatus::Sell { position, bid } = &ctx.accounts.game.player_status {
            require!(call_from != bid.from, GameError::WaitAnotherPlayerBid);
            (*position, bid.value)
        } else {
            return err!(GameError::NotInSell);
        };

    let seller = ctx.accounts.game.current_player;
    let buyer = match seller {
        WhichPlayer::PlayerOne => WhichPlayer::PlayerTwo,
        WhichPlayer::PlayerTwo => WhichPlayer::PlayerOne,
    };
    let buyer_new_balance = ctx.accounts.game.players[buyer.as_index()]
        .balance
        .checked_sub(bid_value)
        .ok_or(GameError::BalanceNotEnough)?;
    let seller_new_balance = ctx.accounts.game.players[seller.as_index()]
        .balance
        .saturating_add(bid_value);

    ctx.accounts.game.players[buyer.as_index()].balance = buyer_new_balance;
    ctx.accounts.game.players[seller.as_index()].balance = seller_new_balance;

    let new_square_status = match ctx.accounts.game.board_status[position as usize] {
        SquareStatus::Mortgaged { .. } => SquareStatus::Mortgaged { by: buyer },
        _ => SquareStatus::Owned {
            by: buyer,
            level: 0,
        },
    };
    ctx.accounts.game.board_status[position as usize] = new_square_status;
    Ok(())
}
