use std::str::FromStr;

use anyhow::{Error, Result};
use dioxus::prelude::*;
use program::Players;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use super::{Route, CLIENT, ERROR, PLAYERS};

#[component]
pub fn Join() -> Element {
    let mut inputed_game_pubkey = use_signal(|| "".to_owned());
    rsx! {
        div { class: "flex flex-col h-screen items-center justify-center",
            p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                "请输入房间公钥"
            }
            input {
                class: "my-3 items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-gray-900 transition-all duration-200 bg-gray-100 border-2 border-gray-900 sm:w-auto rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                value: "{inputed_game_pubkey}",
                oninput: move |event| inputed_game_pubkey.set(event.value())
            }
            button {
                class: "my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-gray-900 border-2 border-transparent sm:w-auto rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                onclick: move |_| {
                    spawn(async move {
                        let game_pubkey = inputed_game_pubkey();
                        if let Err(err) = set_players(&game_pubkey).await {
                            *ERROR.write() = Some(err.to_string());
                            return;
                        }
                        navigator()
                            .push(Route::Game {
                                game_pubkey,
                            });
                    });
                },
                "加入"
            }
        }
    }
}

async fn set_players(game_pubkey: &str) -> Result<()> {
    let client_optional = CLIENT.read();
    let client = client_optional
        .as_ref()
        .ok_or_else(|| Error::msg("client not inited"))?;
    let game_pubkey = Pubkey::from_str(game_pubkey)?;
    let game_status = client.fetch_game_status(&game_pubkey).await?;
    *PLAYERS.write() = Some(Players {
        player_one: game_status.players[0].pubkey,
        player_two: game_status.players[1].pubkey,
    });
    Ok(())
}
