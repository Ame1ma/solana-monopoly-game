use anchor_lang::prelude::*;

use super::*;

pub fn bid_for_action(ctx: Context<Play>, bid_value: u16) -> Result<()> {
    let call_from = call_from(&ctx)?;

    if let PlayerStatus::Action(bid) = &ctx.accounts.game.player_status {
        require!(call_from != bid.from, GameError::WaitAnotherPlayerBid);
        let balance = ctx.accounts.game.players[call_from.as_index()].balance;
        require!(bid_value <= balance, GameError::BalanceNotEnough);
        require!(bid_value > bid.value, GameError::BidValueNotHigher);
    } else {
        return err!(GameError::NotInAction);
    };

    ctx.accounts.game.player_status = PlayerStatus::Action(Bid {
        from: call_from,
        value: bid_value,
    });
    Ok(())
}
