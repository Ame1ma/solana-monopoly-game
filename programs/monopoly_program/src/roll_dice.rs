use anchor_lang::prelude::*;

use super::*;

pub fn roll_dice(ctx: Context<Play>) -> Result<()> {
    let call_from = call_from(&ctx)?;
    need_call_from_current_player(&ctx, call_from)?;

    ctx.accounts.game.dice_status = DiceStatus::Rolling {
        hash_from_each: [None, None],
        plain_from_either: None,
    };
    Ok(())
}
