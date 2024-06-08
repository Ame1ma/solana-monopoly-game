use dioxus::prelude::*;
use log::LevelFilter;
use program::Players;
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Signature};

mod client;
mod connect;
mod create;
mod error;
mod game;
mod home;
mod join;
mod signature;
mod wallet;

use self::{
    client::SolanaClient, connect::Connect, create::Create, error::ErrorPage, game::Game,
    home::Home, join::Join, signature::WaitTxConfirmed, wallet::WALLET,
};

static TAILWIND_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/tailwind.css"));

static WALLET_PUBKEY: GlobalSignal<Option<Pubkey>> = Signal::global(|| None);
static PLAYERS: GlobalSignal<Option<Players>> = Signal::global(|| None);
static CLIENT: GlobalSignal<Option<SolanaClient>> = Signal::global(|| None);
static ERROR: GlobalSignal<Option<String>> = Signal::global(|| None);
static TX_IN_PROGRESS: GlobalSignal<Option<Signature>> = Signal::global(|| None);

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let err = ERROR.read().clone();
    let is_connected = CLIENT.read().is_some() && WALLET_PUBKEY.read().is_some();
    rsx! {
        style { "{TAILWIND_CSS}" }
        match err {
            Some(err) => rsx! { ErrorPage { err: err }},
            None => if is_connected {
                rsx! { Router::<Route> {}}
            } else {
                rsx! { Connect {} }
            },
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/create")]
    Create {},
    #[route("/join")]
    Join {},
    #[route("/game/:game_pubkey")]
    Game { game_pubkey: String },
}
