use std::{ops::Deref, str::FromStr};

use anchor_lang::{prelude::AccountMeta, solana_program::instruction::Instruction, system_program};
use anyhow::{Error, Result};
use dioxus::prelude::*;
use serde::Deserialize;
use solana_client_wasm::solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Signature, transaction::Transaction,
};
use tap::prelude::*;
use wasm_bindgen::{prelude::*, JsStatic};

use super::{CLIENT, PLAYERS, WALLET_PUBKEY};

#[wasm_bindgen]
extern "C" {
    pub type Solana;
    #[wasm_bindgen(js_name=solana)]
    pub static SOLANA: Solana;
    #[wasm_bindgen(method, catch)]
    pub async fn request(this: &Solana, params: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method)]
    pub async fn connect(this: &Solana) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    pub type PublicKey;
    #[wasm_bindgen(method, js_name=toBase58)]
    pub fn to_base58(this: &PublicKey) -> String;
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectWalletResp {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub public_key: JsValue,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTxResp {
    pub signature: String,
}

pub struct PhantomWallet(&'static JsStatic<Solana>);
pub static WALLET: PhantomWallet = PhantomWallet(&SOLANA);

impl Deref for PhantomWallet {
    type Target = JsStatic<Solana>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PhantomWallet {
    pub async fn connect_to_wallet(&self) -> Result<Pubkey> {
        self.connect()
            .await
            .pipe(serde_wasm_bindgen::from_value::<ConnectWalletResp>)
            .map_err(|err| Error::msg(err.to_string()))?
            .public_key
            .unchecked_into::<PublicKey>()
            .to_base58()
            .pipe(|base58| Pubkey::from_str(&base58))?
            .pipe(Ok)
    }
    async fn send_ix(&self, ix: Instruction) -> Result<Signature> {
        let client_optional = CLIENT.read();
        let client = client_optional
            .as_ref()
            .ok_or_else(|| Error::msg("client not inited"))?;
        let pubkey = WALLET_PUBKEY
            .read()
            .ok_or_else(|| Error::msg("wallet not connected"))?;
        let latest_blockhash = client.get_latest_blockhash().await?;
        let solana_message = Message::new_with_blockhash(&[ix], Some(&pubkey), &latest_blockhash);
        let tx = Transaction::new_unsigned(solana_message);
        let phantom_message = bs58::encode(bincode::serialize(&tx)?).into_string();
        let phantom_req_body = PhantomReqBody {
            method: "signAndSendTransaction".to_owned(),
            params: PhantomReqParams {
                message: phantom_message,
            },
        };
        let phantom_req_body = serde_wasm_bindgen::to_value(&phantom_req_body).map_err(|err| {
            Error::msg(format!(
                "serde_wasm_bindgen error {err:#?} {phantom_req_body:#?}"
            ))
        })?;
        let signature = self
            .request(phantom_req_body)
            .await
            .map_err(|err| Error::msg(format!("phantom error {err:#?}")))?
            .pipe(serde_wasm_bindgen::from_value::<SendTxResp>)
            .map_err(|err| Error::msg(format!("serde_wasm_bindgen error {err}")))?
            .signature
            .pipe(|str| Signature::from_str(&str))?;
        Ok(signature)
    }
    pub async fn send_create_ix(&self, ix_data: Vec<u8>) -> Result<Signature> {
        let pubkey = WALLET_PUBKEY
            .read()
            .ok_or_else(|| Error::msg("wallet not connected"))?;
        let players = PLAYERS
            .read()
            .ok_or_else(|| Error::msg("wallet not connected"))?;
        let game_pubkey = players.to_pda();
        let ix = Instruction {
            program_id: program::ID,
            accounts: vec![
                AccountMeta::new(game_pubkey, false),
                AccountMeta::new_readonly(pubkey, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: ix_data,
        };
        self.send_ix(ix).await
    }
    pub async fn send_play_ix(&self, ix_data: Vec<u8>) -> Result<Signature> {
        let pubkey = WALLET_PUBKEY
            .read()
            .ok_or_else(|| Error::msg("wallet not connected"))?;
        let players = PLAYERS
            .read()
            .ok_or_else(|| Error::msg("wallet not connected"))?;
        let game_pubkey = players.to_pda();
        let ix = Instruction {
            program_id: program::ID,
            accounts: vec![
                AccountMeta::new(game_pubkey, false),
                AccountMeta::new_readonly(pubkey, true),
            ],
            data: ix_data,
        };
        self.send_ix(ix).await
    }
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PhantomReqBody {
    method: String,
    params: PhantomReqParams,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PhantomReqParams {
    message: String,
}
