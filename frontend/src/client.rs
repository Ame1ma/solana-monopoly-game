use std::{ops::Deref, time::Duration};

use anchor_lang::{solana_program::pubkey::Pubkey, AccountDeserialize};
use anyhow::Result;
use gloo_timers::future::sleep;
use solana_client_wasm::{solana_sdk::signature::Signature, WasmClient};
use tap::prelude::*;

pub struct SolanaClient(WasmClient);

impl Deref for SolanaClient {
    type Target = WasmClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SolanaClient {
    pub fn new(endpoint: &str) -> Self {
        Self(WasmClient::new(endpoint))
    }
    pub async fn wait_tx_confirmed(&self, signature: &Signature) -> Result<()> {
        while !self.confirm_transaction(signature).await? {
            sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }
    pub async fn fetch_game_status(&self, game_pubkey: &Pubkey) -> Result<program::Game> {
        let game_status = self
            .get_account_data(game_pubkey)
            .await?
            .pipe(|vec| program::Game::try_deserialize(&mut vec.as_slice()))?;
        Ok(game_status)
    }
}
