use dioxus::prelude::*;
use tap::prelude::*;

use super::{client::SolanaClient, CLIENT, ERROR, WALLET, WALLET_PUBKEY};

#[component]
pub fn Connect() -> Element {
    let mut inputed_endpoint = use_signal(|| "".to_owned());
    rsx! {
        div { class: "flex flex-col h-screen items-center justify-center",
            h1 { class: "mb-60 text-center font-mono text-6xl font-bold", "=大富翁=" }
            input {
                class: "placeholder:text-center my-3 items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-gray-900 transition-all duration-200 bg-gray-100 border-2 border-gray-900 sm:w-auto rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                value: "{inputed_endpoint}",
                placeholder: "请输入 RPC 地址",
                oninput: move |event| {
                    *inputed_endpoint.write() = event.value();
                }
            }
            button {
                class: "my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-gray-900 border-2 border-transparent sm:w-auto rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                onclick: move |_| {
                    spawn(async move {
                        let endpoint = inputed_endpoint();
                        init_global_wallet_pubkey().await;
                        init_global_client(&endpoint);
                    });
                },
                "连接"
            }
        }
    }
}

fn init_global_client(endpoint: &str) {
    SolanaClient::new(endpoint).pipe(Some).pipe(|client| {
        *CLIENT.write() = client;
    });
}

async fn init_global_wallet_pubkey() {
    match WALLET.connect_to_wallet().await {
        Ok(pubkey) => *WALLET_PUBKEY.write() = Some(pubkey),
        Err(err) => *ERROR.write() = Some(err.to_string()),
    };
}
