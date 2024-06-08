use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::signature::Signature;

use super::CLIENT;

pub trait WaitTxConfirmed {
    async fn wait_tx_confirmed(&self) -> Result<(), String>;
}

impl WaitTxConfirmed for Signature {
    async fn wait_tx_confirmed(&self) -> Result<(), String> {
        let client_optional = CLIENT.read();
        let client = client_optional
            .as_ref()
            .ok_or_else(|| "client not inited".to_owned())?;
        client
            .wait_tx_confirmed(self)
            .await
            .map_err(|err| err.to_string())?;
        Ok(())
    }
}
