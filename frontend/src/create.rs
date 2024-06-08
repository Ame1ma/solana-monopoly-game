use std::str::FromStr;

use anchor_lang::InstructionData;
use anyhow::{Error, Result};
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Signature};
use tap::prelude::*;

use super::{Route, WaitTxConfirmed, ERROR, PLAYERS, TX_IN_PROGRESS, WALLET, WALLET_PUBKEY};

#[component]
pub fn Create() -> Element {
    rsx! {
        match *TX_IN_PROGRESS.read() {
            None => rsx! {
                BeforeGameCreate {}
            },
            Some(signature) => rsx! {
                AfterGameCreate { signature: signature }
            },
        }
    }
}

#[component]
fn BeforeGameCreate() -> Element {
    let mut inputed_opponet_pubkey = use_signal(|| "".to_owned());
    rsx! {
        div { class: "flex flex-col h-screen items-center justify-center",
            p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                "请输入朋友公钥"
            }
            input {
                class: "my-3 items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-gray-900 transition-all duration-200 bg-gray-100 border-2 border-gray-900 sm:w-auto rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                value: "{inputed_opponet_pubkey}",
                oninput: move |event| inputed_opponet_pubkey.set(event.value())
            }
            button {
                class: "my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-gray-900 border-2 border-transparent sm:w-auto rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                onclick: move |_| {
                    spawn(async move {
                        let opponet_pubkey = match Pubkey::from_str(&inputed_opponet_pubkey()) {
                            Ok(opponet_pubkey) => opponet_pubkey,
                            Err(err) => {
                                *ERROR.write() = Some(err.to_string());
                                return;
                            }
                        };
                        match create_game(&opponet_pubkey.to_string()).await {
                            Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                            Err(err) => *ERROR.write() = Some(err.to_string()),
                        };
                    });
                },
                "创建"
            }
        }
    }
}

async fn create_game(opponet_pubkey: &str) -> Result<Signature> {
    let player_one = WALLET_PUBKEY
        .read()
        .ok_or_else(|| Error::msg("wallet not connected"))?;
    let player_two = Pubkey::from_str(opponet_pubkey)?;
    let players = program::Players {
        player_one,
        player_two,
    };
    *PLAYERS.write() = Some(players);
    let ix = program::instruction::NewGame { player_two };
    let signature = WALLET.send_create_ix(ix.data()).await?;
    Ok(signature)
}

#[component]
fn AfterGameCreate(signature: Signature) -> Element {
    let game_pubkey = use_resource(move || async move {
        signature.wait_tx_confirmed().await?;
        PLAYERS
            .read()
            .ok_or_else(|| "players not inited".to_owned())?
            .to_pda()
            .pipe(Ok::<Pubkey, String>)
    });

    match game_pubkey.value().read().clone() {
        Some(Ok(game_pubkey)) => rsx! {
            div { class: "flex flex-col h-screen items-center justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "房间公钥为"
                }
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "{game_pubkey}"
                }
                button {
                    class: "my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-gray-900 border-2 border-transparent sm:w-auto rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                    onclick: move |_| {
                        navigator()
                            .push(Route::Game {
                                game_pubkey: game_pubkey.to_string(),
                            });
                    },
                    "进入房间"
                }
            }
        },
        Some(Err(err)) => rsx! {
            div { class: "flex flex-col h-screen items-center justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "创建失败"
                }
                p { class: "my-3 w-9/12 text-center font-mono text-2xl font-bold",
                    "{err}"
                }
            }
        },
        None => rsx! {
            div { class: "flex flex-col h-screen items-center justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "创建中..."
                }
            }
        },
    }
}
