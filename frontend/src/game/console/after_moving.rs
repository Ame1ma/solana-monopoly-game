use anchor_lang::InstructionData;
use anyhow::Result;
use dioxus::prelude::*;
use program::{Game, WhichPlayer};
use solana_client_wasm::solana_sdk::signature::Signature;

use super::{ERROR, TX_IN_PROGRESS, WALLET};

mod buy_house;
mod mortgage;
mod sell_house;
mod sell_square;
mod unmortgage;

use self::{
    buy_house::BuyHouse, mortgage::Mortgage, sell_house::SellHouse, sell_square::SellSquare,
    unmortgage::Unmortgage,
};

#[component]
pub fn AfterMoving(game: Game, self_player: WhichPlayer) -> Element {
    let mut current_operate_signal = use_signal(|| CurrentOperate::None);
    let current_operate = current_operate_signal.read();

    if self_player != game.current_player {
        return rsx! {
            div { class: "w-full aspect-square flex flex-col items-center py-7 justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "等待对方操作..."
                }
            }
        };
    }

    match current_operate.clone() {
        CurrentOperate::None => rsx! {
            div { class: "w-full aspect-square flex flex-col items-center justify-start py-7 md:py-0 md:justify-center",
                div { class: "grid grid-cols-2 w-full gap-y-7 gap-x-9",
                    div { class: "w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-r-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                spawn(async move {
                                    match buy_square().await {
                                        Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                                        Err(err) => *ERROR.write() = Some(err.to_string()),
                                    };
                                });
                            },
                            "购买"
                        }
                    }
                    div { class: "w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-l-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                *current_operate_signal.write() = CurrentOperate::SellSquare;
                            },
                            "出售"
                        }
                    }
                    div { class: "w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-r-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                *current_operate_signal.write() = CurrentOperate::BuyHouse;
                            },
                            "升级"
                        }
                    }
                    div { class: "w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-l-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                *current_operate_signal.write() = CurrentOperate::SellHouse;
                            },
                            "降级"
                        }
                    }
                    div { class: "w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-r-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                *current_operate_signal.write() = CurrentOperate::Mortgage;
                            },
                            "抵押"
                        }
                    }
                    div { class: "w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-l-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                *current_operate_signal.write() = CurrentOperate::Unmortgage;
                            },
                            "赎回"
                        }
                    }
                    div { class: "col-span-2 w-full flex flex-col items-center justify-center",
                        button {
                            class: "w-1/2 shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                            onclick: move |_| {
                                spawn(async move {
                                    match end_turn().await {
                                        Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                                        Err(err) => *ERROR.write() = Some(err.to_string()),
                                    };
                                });
                            },
                            "结束回合"
                        }
                    }
                }
            }
        },
        CurrentOperate::SellSquare => {
            rsx! { SellSquare { current_operate_signal: current_operate_signal } }
        }
        CurrentOperate::BuyHouse => {
            rsx! { BuyHouse { current_operate_signal: current_operate_signal } }
        }
        CurrentOperate::SellHouse => {
            rsx! { SellHouse { current_operate_signal: current_operate_signal } }
        }
        CurrentOperate::Mortgage => {
            rsx! { Mortgage { current_operate_signal: current_operate_signal } }
        }
        CurrentOperate::Unmortgage => {
            rsx! { Unmortgage { current_operate_signal: current_operate_signal } }
        }
    }
}

#[derive(Clone)]
enum CurrentOperate {
    SellSquare,
    BuyHouse,
    SellHouse,
    Mortgage,
    Unmortgage,
    None,
}

async fn buy_square() -> Result<Signature> {
    let ix = program::instruction::BuySquare;
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}

async fn end_turn() -> Result<Signature> {
    let ix = program::instruction::EndTurn;
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}
