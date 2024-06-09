use anchor_lang::prelude::*;

use super::*;

pub fn commit_dice_hash(ctx: Context<Play>, hash: [u8; 32]) -> Result<()> {
    let call_from = call_from(&ctx)?;

    if let DiceStatus::Rolling {
        hash_from_each,
        plain_from_either,
    } = &mut ctx.accounts.game.dice_status
    {
        hash_from_each[call_from.as_index()] = Some(hash);
        if plain_from_either.is_some() {
            hash_from_each[call_from.either_index()] = None;
        }
        *plain_from_either = None;
    } else {
        return err!(GameError::NotRolling);
    }

    Ok(())
}
