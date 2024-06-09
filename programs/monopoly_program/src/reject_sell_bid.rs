use anchor_lang::prelude::*;

use super::*;

pub fn reject_sell_bid(ctx: Context<Play>) -> Result<()> {
    let call_from = call_from(&ctx)?;

    if let PlayerStatus::Sell { position: _, bid } = &ctx.accounts.game.player_status {
        require!(call_from != bid.from, GameError::WaitAnotherPlayerBid);
    } else {
        return err!(GameError::NotInSell);
    };

    ctx.accounts.game.player_status = PlayerStatus::AfterMoving;
    Ok(())
}
