use dioxus::prelude::*;
use program::{Game, PlayerStatus, WhichPlayer};

use super::{WaitTxConfirmed, ERROR, TX_IN_PROGRESS, WALLET, WALLET_PUBKEY};

mod action;
mod after_moving;
mod before_moving;
mod sell;

use self::{action::Action, after_moving::AfterMoving, before_moving::BeforeMoving, sell::Sell};

#[component]
pub fn GameConsole(game: Game) -> Element {
    if matches!(game.player_status, PlayerStatus::Lose) {
        let current_player = game.current_player;
        let winer = match current_player {
            WhichPlayer::PlayerOne => "2P",
            WhichPlayer::PlayerTwo => "1P",
        };
        return rsx! {
            div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "游戏结束"
                }
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "{winer}获胜"
                }
            }
        };
    }

    let _latest_tx_result = use_resource(move || async move {
        match *TX_IN_PROGRESS.read() {
            Some(signature) => signature.wait_tx_confirmed().await,
            None => Ok(()),
        }
    });

    let self_pubkey = (*WALLET_PUBKEY.read())?;
    let self_player = if self_pubkey == game.players[0].pubkey {
        WhichPlayer::PlayerOne
    } else if self_pubkey == game.players[1].pubkey {
        WhichPlayer::PlayerTwo
    } else {
        return rsx! {
            div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "观战模式"
                }
            }
        };
    };

    match game.clone().player_status {
        PlayerStatus::BeforeMoving => {
            rsx! { BeforeMoving { game: game, self_player: self_player } }
        }
        PlayerStatus::AfterMoving => rsx! { AfterMoving { game: game, self_player: self_player } },
        PlayerStatus::Sell { position, bid } => {
            rsx! { Sell { game: game, position: position, bid: bid, self_player: self_player } }
        }
        PlayerStatus::Action(bid) => {
            rsx! { Action { game: game, bid: bid, self_player: self_player } }
        }
        PlayerStatus::Lose => rsx! {"游戏结束"},
    }
}
