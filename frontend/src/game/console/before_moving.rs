use anchor_lang::{solana_program::hash::hash as calculate_hash, InstructionData};
use anyhow::Result;
use dioxus::prelude::*;
use program::{DicePlain, DiceStatus, Game, WhichPlayer};
use solana_client_wasm::solana_sdk::signature::Signature;

use super::{ERROR, TX_IN_PROGRESS, WALLET};

#[component]
pub fn BeforeMoving(game: Game, self_player: WhichPlayer) -> Element {
    let is_self_current_player = self_player == game.current_player;

    if let DiceStatus::Rolling {
        hash_from_each,
        plain_from_either,
    } = game.dice_status
    {
        return rsx! {
            Rolling {
                hash_from_each: hash_from_each,
                plain_from_either: plain_from_either,
                is_self_current_player: is_self_current_player
            }
        };
    };

    if !is_self_current_player {
        rsx! {
            div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "等待对方投掷骰子..."
                }
            }
        }
    } else {
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
}

#[component]
pub fn Rolling(
    hash_from_each: [Option<[u8; 32]>; 2],
    plain_from_either: Option<DicePlain>,
    is_self_current_player: bool,
) -> Element {
    let mut dice_local_status_signal = use_signal(|| DiceLocalState::Init);
    let _tx_sender = use_resource(move || async move {
        let dice_local_status = *dice_local_status_signal.read();
        match dice_local_status {
            DiceLocalState::Init => (),
            DiceLocalState::HashSent(DiceLocalData { random_num, salt }) => {
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
            DiceLocalState::PlainSent(DiceLocalData { random_num, salt }) => {
                match commit_dice_plain(random_num, salt).await {
                    Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                    Err(err) => *ERROR.write() = Some(err.to_string()),
                };
            }
        };
    });

    let dice_local_status = *dice_local_status_signal.read();
    match dice_local_status {
        DiceLocalState::Init => {
            let random_num = rand::random::<u16>();
            let salt = rand::random::<[u8; 32]>();
            let data = DiceLocalData { random_num, salt };
            *dice_local_status_signal.write() = DiceLocalState::HashSent(data);
        }
        DiceLocalState::HashSent(data) => {
            if hash_from_each.iter().all(Option::is_some) {
                *dice_local_status_signal.write() = DiceLocalState::PlainSent(data);
            }
        }
        DiceLocalState::PlainSent(_) => {
            if hash_from_each.iter().any(Option::is_none) {
                let random_num = rand::random::<u16>();
                let salt = rand::random::<[u8; 32]>();
                let data = DiceLocalData { random_num, salt };
                *dice_local_status_signal.write() = DiceLocalState::HashSent(data);
            }
        }
    };

    let maybe_opponet_str = if !is_self_current_player {
        "对方"
    } else {
        ""
    };

    let dice_status_str = format!(
        "({p1_hash}/{p2_hash}/{either_plain})",
        p1_hash = hash_from_each[0].is_some() as usize,
        p2_hash = hash_from_each[1].is_some() as usize,
        either_plain = plain_from_either.is_some() as usize,
    );

    rsx! {
        div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
            p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                "{maybe_opponet_str}投掷骰子中{dice_status_str}..."
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

#[derive(Clone, Copy)]
enum DiceLocalState {
    Init,
    HashSent(DiceLocalData),
    PlainSent(DiceLocalData),
}

#[derive(Clone, Copy)]
struct DiceLocalData {
    random_num: u16,
    salt: [u8; 32],
}
