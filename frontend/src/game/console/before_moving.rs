use anchor_lang::{solana_program::hash::hash as calculate_hash, InstructionData};
use anyhow::Result;
use dioxus::prelude::*;
use program::{DicePlain, DiceStatus, Game, WhichPlayer};
use solana_client_wasm::solana_sdk::signature::Signature;

use super::{ERROR, TX_IN_PROGRESS, WALLET};

#[component]
pub fn BeforeMoving(game: Game, self_player: WhichPlayer) -> Element {
    let mut random_signal = use_signal(|| None::<u16>);
    let mut salt_signal = use_signal(|| None::<[u8; 32]>);
    let mut dice_plain_signal = use_signal(|| None::<DicePlain>);

    let _send_hash = use_resource(move || async move {
        let random_num = *random_signal.read();
        let salt = *salt_signal.read();
        if let Some((random_num, salt)) = random_num.zip(salt) {
            let hash_from_plain = calculate_hash(
                &random_num
                    .to_be_bytes()
                    .iter()
                    .chain(salt.iter())
                    .copied()
                    .collect::<Vec<u8>>(),
            )
            .to_bytes();
            match commit_dice_hash(hash_from_plain).await {
                Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                Err(err) => *ERROR.write() = Some(err.to_string()),
            };
        }
    });

    let _send_plain = use_resource(move || async move {
        if let Some(dice_plain) = dice_plain_signal.read().clone() {
            match commit_dice_plain(dice_plain.random_num, dice_plain.salt).await {
                Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                Err(err) => *ERROR.write() = Some(err.to_string()),
            };
        }
    });

    if let DiceStatus::Rolling {
        hash_from_each,
        plain_from_either: _,
    } = game.dice_status
    {
        if hash_from_each[self_player as usize].is_none() {
            let random = rand::random::<u16>();
            let salt = rand::random::<[u8; 32]>();
            *random_signal.write() = Some(random);
            *salt_signal.write() = Some(salt);
        } else if hash_from_each.iter().all(Option::is_some) {
            let random_num = *random_signal.read();
            let salt = *salt_signal.read();
            if let Some((random_num, salt)) = random_num.zip(salt) {
                let plain = DicePlain {
                    from: self_player,
                    random_num,
                    salt,
                };
                *dice_plain_signal.write() = Some(plain);
            }
        }

        let maybe_opponet_str = if self_player != game.current_player {
            "对方"
        } else {
            ""
        };

        return rsx! {
            div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "{maybe_opponet_str}投掷骰子中..."
                }
            }
        };
    };

    if self_player != game.current_player {
        return rsx! {
            div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "等待对方投掷骰子..."
                }
            }
        };
    }

    rsx! {
        div { class: "w-full aspect-square flex flex-col items-center justify-center",
            button {
                class: "drop-shadow-2xl my-3 inline-flex items-center justify-center w-3/5 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-lg hover:bg-cyan-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                onclick: move |_| {
                    spawn(async move {
                        match roll_dice().await {
                            Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                            Err(err) => *ERROR.write() = Some(err.to_string()),
                        };
                    });
                },
                "投掷骰子"
            }
        }
    }
}

async fn roll_dice() -> Result<Signature> {
    let ix = program::instruction::RollDice;
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}

async fn commit_dice_hash(hash: [u8; 32]) -> Result<Signature> {
    let ix = program::instruction::CommitDiceHash { hash };
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}

async fn commit_dice_plain(random_num: u16, salt: [u8; 32]) -> Result<Signature> {
    let ix = program::instruction::CommitDicePlain { random_num, salt };
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}
