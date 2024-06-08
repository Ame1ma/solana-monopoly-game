use anchor_lang::InstructionData;
use anyhow::Result;
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::signature::Signature;

use super::{CurrentOperate, ERROR, TX_IN_PROGRESS, WALLET};

#[component]
pub fn SellSquare(current_operate_signal: Signal<CurrentOperate>) -> Element {
    let mut inputed_bid_value = use_signal(|| "".to_owned());
    let mut inputed_position = use_signal(|| "".to_owned());
    rsx! {
        div { class: "col-span-2 w-full flex flex-col items-center justify-center",
            input {
                class: "w-9/12 shadow-2xl drop-shadow-2xl items-center justify-center px-8 py-2 text-lg font-bold text-gray-900 transition-all duration-200 bg-gray-100 border-2 border-cyan-900 rounded-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                value: "{inputed_position}",
                oninput: move |event| inputed_position.set(event.value())
            }
        }
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
                        let square_position = inputed_position();
                        match sell_square(&square_position, &bid_value).await {
                            Ok(signature) => *TX_IN_PROGRESS.write() = Some(signature),
                            Err(err) => *ERROR.write() = Some(err.to_string()),
                        };
                    });
                },
                "出售"
            }
        }
        div { class: "w-full flex flex-col items-center justify-center",
            button {
                class: "w-full shadow-2xl drop-shadow-2xl inline-flex items-center justify-center px-8 py-2 text-base font-bold text-white transition-all duration-200 bg-cyan-900 border-2 border-transparent rounded-l-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-900",
                onclick: move |_| {
                    *current_operate_signal.write() = CurrentOperate::None;
                },

                "返回"
            }
        }
    }
}

async fn sell_square(square_position: &str, bid_value: &str) -> Result<Signature> {
    let square_position = square_position.parse::<u8>()?;
    let bid_value = bid_value.parse::<u16>()?;
    let ix = program::instruction::SellSquare {
        square_position,
        bid_value,
    };
    let signature = WALLET.send_play_ix(ix.data()).await?;
    Ok(signature)
}
