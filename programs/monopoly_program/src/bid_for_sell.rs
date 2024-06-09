use anchor_lang::prelude::*;

use super::*;

pub fn bid_for_sell(ctx: Context<Play>, bid_value: u16) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let seller = ctx.accounts.game.current_player;
    let buyer_balance = ctx.accounts.game.players[seller.either_index()].balance;
    let position = if let PlayerStatus::Sell { position, bid } = &ctx.accounts.game.player_status {
        require!(call_from != bid.from, GameError::WaitAnotherPlayerBid);
        require!(bid.value <= buyer_balance, GameError::BalanceNotEnough);
        *position
    } else {
        return err!(GameError::NotInSell);
    };

    ctx.accounts.game.player_status = PlayerStatus::Sell {
        position,
        bid: Bid {
            from: call_from,
            value: bid_value,
        },
    };
    Ok(())
}
