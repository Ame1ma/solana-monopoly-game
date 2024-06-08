use anchor_lang::InstructionData;
use anyhow::Result;
use dioxus::prelude::*;
use program::{Bid, Game, WhichPlayer};
use solana_client_wasm::solana_sdk::signature::Signature;

use super::{ERROR, TX_IN_PROGRESS, WALLET};

#[component]
pub fn Action(game: Game, bid: Bid, self_player: WhichPlayer) -> Element {
    let mut inputed_bid_value = use_signal(|| "".to_owned());

    let value = bid.value;

    let current_player = game.current_player;
    let current_player_status = &game.players[current_player as usize];

    let position = current_player_status.position;
    let position_str = format!("正在拍卖地产{position}");

    let value_str = if bid.from == self_player {
        format!("已出价{value}")
    } else {
        format!("对方出价{value}")
    };

    let operate_rsx = if bid.from == self_player {
        rsx! {
            div { class: "col-span-2 w-full flex flex-col items-center justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-2xl font-bold",
                    "等待对方确认"
                }
            }
        }
    } else {
        rsx! {
            div { class: "col-span-2 w-full flex flex-col items-center justify-center",
                input {
                    class: "w-9/12 shadow-2xl drop-shadow-2xl items-center justify-center px-8 py-2 text-lg font-bold text-gray-900 transition-all duration-200 bg-gray-100 border-2 border-cyan-900 rounded-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                    value: "{inputed_bid_value}",
                    oninput: move |event| inputed_bid_value.set(event.value())
                }
            }
            div { class: "w-full flex flex-col items-center justify-center",
                button {
                    class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-r-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                    onclick: move |_| {
                        spawn(async move {
                            let bid_value = inputed_bid_value();
                            match bind_for_action(&bid_value).await {
                                Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                                Err(err) => *ERROR.write() = Some(err.to_string()),
                            };
                        });
                    },
                    "出价"
                }
            }
            div { class: "w-full flex flex-col items-center justify-center",
                button {
                    class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-l-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                    onclick: move |_| {
                        spawn(async move {
                            match accept_action_bid().await {
                                Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                                Err(err) => *ERROR.write() = Some(err.to_string()),
                            };
                        });
                    },

                    "放弃"
                }
            }
        }
    };

    rsx! {
        div { class: "w-full aspect-square flex flex-col items-center justify-start py-7 md:py-0 md:justify-center",
            div { class: "grid grid-cols-2 w-full gap-y-7 gap-x-9",
                div { class: "col-span-2 w-full flex flex-col items-center justify-center",
                    p { class: "my-3 w-9/12 text-center font-mono text-2xl font-bold",
                        "{position_str}"
                    }
                    p { class: "my-3 w-9/12 text-center font-mono text-2xl font-bold",
                        "{value_str}"
                    }
                }
                {operate_rsx}
            }
        }
    }
}

async fn accept_action_bid() -> Result<Signature> {
    let ix = program::instruction::AcceptActionBid;
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}

async fn bind_for_action(bid_value: &str) -> Result<Signature> {
    let bid_value = bid_value.parse::<u16>()?;
    let ix = program::instruction::BidForAction { bid_value };
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}
