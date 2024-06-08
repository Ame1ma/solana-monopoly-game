use std::{str::FromStr, time::Duration};

use anyhow::{Error, Result};
use dioxus::prelude::*;
use gloo_timers::future::sleep;
use program::Game;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

mod console;
mod square;

use self::{console::GameConsole, square::GameSquare};
use super::{WaitTxConfirmed, CLIENT, ERROR, TX_IN_PROGRESS, WALLET, WALLET_PUBKEY};

#[component]
pub fn Game(game_pubkey: String) -> Element {
    let game_status_signal = use_signal::<Option<Game>>(|| None);
    let _game_status_fetch_loop = use_coroutine(|_rx: UnboundedReceiver<()>| async move {
        if let Err(err) = game_status_fetch_loop(game_status_signal, &game_pubkey).await {
            *ERROR.write() = Some(err.to_string());
        }
    });
    let game_status = game_status_signal.read();
    match game_status.clone() {
        None => rsx! {
            div { class: "flex flex-col h-screen items-center justify-center",
                p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold",
                    "加载中..."
                }
            }
        },
        Some(game_status) => rsx! {
            div { class: "flex flex-col md:flex-row h-screen items-center justify-center",
                div { class: "flex aspect-square w-full md:w-1/2", GameSquare { game: game_status.clone() } }
                div { class: "flex aspect-square w-full md:w-1/2", GameConsole { game: game_status } }
            }
        },
    }
}

async fn game_status_fetch_loop(
    mut game_status_signal: Signal<Option<Game>>,
    game_pubkey: &str,
) -> Result<()> {
    let client_optional = CLIENT.read();
    let client = client_optional
        .as_ref()
        .ok_or_else(|| Error::msg("client not inited"))?;
    let game_pubkey = Pubkey::from_str(game_pubkey)?;
    loop {
        match client.fetch_game_status(&game_pubkey).await {
            Ok(game_status) => *game_status_signal.write() = Some(game_status),
            Err(err) => log::error!("{err}"),
        };
        sleep(Duration::from_secs(1)).await;
    }
}
