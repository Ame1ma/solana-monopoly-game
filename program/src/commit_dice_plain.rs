use anchor_lang::{prelude::*, solana_program::hash::hash as calculate_hash};

use super::*;

pub fn commit_dice_plain(ctx: Context<Play>, random_num: u16, salt: [u8; 32]) -> Result<()> {
    let call_from = call_from(&ctx)?;

    let DiceStatus::Rolling {
        hash_from_each,
        plain_from_either,
    } = &ctx.accounts.game.dice_status
    else {
        return err!(GameError::NotRolling);
    };

    let hash_commited_before = if let Some((player_one_hash, player_two_hash)) =
        hash_from_each[0].zip(hash_from_each[1])
    {
        match call_from {
            WhichPlayer::PlayerOne => player_one_hash,
            WhichPlayer::PlayerTwo => player_two_hash,
        }
    } else {
        return err!(GameError::HashCollectNotFinish);
    };

    let hash_from_plain = calculate_hash(
        &random_num
            .to_be_bytes()
            .iter()
            .chain(salt.iter())
            .copied()
            .collect::<Vec<u8>>(),
    )
    .to_bytes();

    require!(
        hash_from_plain == hash_commited_before,
        GameError::HashVerifyFailed
    );

    match plain_from_either {
        Some(DicePlain {
            from,
            random_num: random_num_commited_before,
            salt: _,
        }) if *from != call_from => {
            let dice_num = ((random_num + *random_num_commited_before) % 6 + 1) as u8;
            ctx.accounts.game.dice_status = DiceStatus::Rolled(dice_num);

            let current_player_index = ctx.accounts.game.current_player.as_index();
            let old_position = ctx.accounts.game.players[current_player_index].position;
            let new_position = (old_position + dice_num) % 16;
            ctx.accounts.game.players[current_player_index].position = new_position;

            if old_position + dice_num >= 16 {
                ctx.accounts.game.players[current_player_index].balance += Game::SALARY;
            }
        }
        _ => {
            ctx.accounts.game.dice_status = DiceStatus::Rolling {
                hash_from_each: *hash_from_each,
                plain_from_either: Some(DicePlain {
                    from: call_from,
                    random_num,
                    salt,
                }),
            }
        }
    }

    Ok(())
}
