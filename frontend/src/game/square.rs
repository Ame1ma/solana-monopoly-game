use dioxus::prelude::*;
use program::{DiceStatus, Game, PlayerStatus, SquareColor, SquareStatus, WhichPlayer};

use super::WALLET_PUBKEY;

#[component]
pub fn GameSquare(game: Game) -> Element {
    let self_pubkey = (*WALLET_PUBKEY.read())?;

    let self_player = if self_pubkey == game.players[0].pubkey {
        Some(WhichPlayer::PlayerOne)
    } else if self_pubkey == game.players[1].pubkey {
        Some(WhichPlayer::PlayerTwo)
    } else {
        None
    };
    let current_player = game.current_player;
    let current_player_str = match current_player {
        WhichPlayer::PlayerOne => "1P".to_owned(),
        WhichPlayer::PlayerTwo => "2P".to_owned(),
    };
    let game_status_str = match game.player_status {
        PlayerStatus::BeforeMoving => format!("等待{current_player_str}投掷骰子"),
        PlayerStatus::AfterMoving => format!("等待{current_player_str}行动"),
        PlayerStatus::Sell { .. } => format!("{current_player_str}发起了地产交易"),
        PlayerStatus::Action(_) => format!("{current_player_str}放弃了购买，开始拍卖"),
        PlayerStatus::Lose => {
            let anothor_player_str = match game.current_player {
                WhichPlayer::PlayerOne => "2P".to_owned(),
                WhichPlayer::PlayerTwo => "1P".to_owned(),
            };
            format!("{anothor_player_str}赢了!")
        }
    };
    let p1_balance = game.players[0].balance;
    let p2_balance = game.players[1].balance;
    let p1_balance_str = format!(
        "1P余额: {p1_balance}{}",
        self_player
            .map(|self_player| matches!(self_player, WhichPlayer::PlayerOne))
            .unwrap_or(false)
            .then_some("(自己)")
            .unwrap_or_default()
    );
    let p2_balance_str = format!(
        "2P余额: {p2_balance}{}",
        self_player
            .map(|self_player| matches!(self_player, WhichPlayer::PlayerTwo))
            .unwrap_or(false)
            .then_some("(自己)")
            .unwrap_or_default()
    );
    let dice_str = match game.dice_status {
        DiceStatus::Rolling { .. } => "骰子: 投掷中...".to_string(),
        DiceStatus::Rolled(num) => format!("骰子: {num}"),
    };
    let squares = Game::BOARD_INFO
        .iter()
        .zip(SQUARE_ORDER_MAPPING.iter())
        .zip(game.board_status)
        .enumerate()
        .map(|(position, ((square_info, order_css), square_status))| {
            let status = match square_status {
                SquareStatus::Unowned => {
                    let price = square_info.price;
                    rsx! {
                        div { class: "text-white text-center text-xs", "{position}/无人" }
                        div { class: "text-white text-center text-xs", "售价{price}" }
                    }
                },
                SquareStatus::Owned { by, level } => {
                    let house_price = square_info.house_price;
                    let rent = square_info.rent[level as usize];
                    let owner_str = match by {
                        WhichPlayer::PlayerOne => "1P",
                        WhichPlayer::PlayerTwo => "2P",
                    };
                    rsx! {
                        p { class: "text-white text-center text-xs", "{position}/{owner_str}" }
                        p { class: "text-white text-center text-xs", "等级{level}" }
                        p { class: "text-white text-center text-xs", "过夜{rent}" }
                        p { class: "text-white text-center text-xs", "升级{house_price}" }
                    }
                },
                SquareStatus::Mortgaged { by } => {
                    let owner_str = match by {
                        WhichPlayer::PlayerOne => "1P",
                        WhichPlayer::PlayerTwo => "2P",
                    };
                    let mortgage_value = square_info.house_price / 2;
                    rsx! {
                        p { class: "text-white text-center text-xs", "{position}/{owner_str}" }
                        p { class: "text-white text-center text-xs", "抵押" }
                        p { class: "text-white text-center text-xs", "赎回{mortgage_value}" }
                    }
                },
            };
            let color_css = square_color_css(square_info.color);
            rsx! {
                div { class: "{order_css} {color_css} {SQUARE_BASE_CSS}",
                    if game.players[0].position as usize == position {
                        if game.current_player as usize == 0 {
                            div { class: "text-xs rounded-full bg-white fixed z-50 bottom-0 left-0 outline-black outline outline-dashed animate-bounce",
                                "1P"
                            }
                        } else {
                            div { class: "text-xs rounded-full bg-white fixed z-50 bottom-0 left-0 outline-black outline outline-dashed",
                                "1P"
                            }
                        }
                    }
                    if game.players[1].position as usize == position {
                        if game.current_player as usize == 1 {
                            div { class: "text-xs rounded-full bg-white fixed z-50 bottom-0 right-0 outline-black outline outline-dashed animate-bounce",
                                "2P"
                            }
                        } else {
                            div { class: "text-xs rounded-full bg-white fixed z-50 bottom-0 right-0 outline-black outline outline-dashed",
                                "2P"
                            }
                        }
                    }
                    {status}
                }
            }
        });
    rsx! {
        div { class: "grid grid-cols-5 gap-0 place-content-stretch w-full aspect-square",
            div { class: "order-[7] row-span-3 col-span-3 aspect-square flex flex-col justify-center",
                p { class: "text-center", "{game_status_str}" }
                p { class: "text-center", "{p1_balance_str}" }
                p { class: "text-center", "{p2_balance_str}" }
                p { class: "text-center", "{dice_str}" }
            }
            {squares}
        }
    }
}

const SQUARE_BASE_CSS: &str =
    "rounded-lg border-white border drop-shadow-2xl aspect-square flex flex-col justify-center";

const SQUARE_ORDER_MAPPING: [&str; 16] = [
    "order-[17]",
    "order-[16]",
    "order-[15]",
    "order-[14]",
    "order-[13]",
    "order-[11]",
    "order-[9]",
    "order-[6]",
    "order-[1]",
    "order-[2]",
    "order-[3]",
    "order-[4]",
    "order-[5]",
    "order-[8]",
    "order-[10]",
    "order-[12]",
];

fn square_color_css(color: SquareColor) -> &'static str {
    match color {
        SquareColor::Red => "bg-red-400",
        SquareColor::Yellow => "bg-yellow-400",
        SquareColor::Blue => "bg-blue-400",
        SquareColor::Green => "bg-green-400",
    }
}
