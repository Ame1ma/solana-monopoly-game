#![allow(unused_imports)]
use anchor_lang::prelude::*;

declare_id!("GMDedNzaiCffFmBNVBDUzd6Ub6XLQ4xhoWfswBRmYqbG");

mod accept_action_bid;
mod accept_sell_bid;
mod bid_for_action;
mod bid_for_sell;
mod buy_house;
mod buy_square;
mod commit_dice_hash;
mod commit_dice_plain;
mod common;
mod end_turn;
mod mortgage;
mod new_game;
mod reject_sell_bid;
mod roll_dice;
mod sell_house;
mod sell_square;
mod unmortgage;

pub use common::*;

use self::{
    accept_action_bid::*, accept_sell_bid::*, bid_for_action::*, bid_for_sell::*, buy_house::*,
    buy_square::*, commit_dice_hash::*, commit_dice_plain::*, end_turn::*, mortgage::*,
    new_game::*, reject_sell_bid::*, roll_dice::*, sell_house::*, sell_square::*, unmortgage::*,
};

#[program]
pub mod monopoly_program {
    use super::*;

    pub fn new_game(ctx: Context<NewGame>, player_two: Pubkey) -> Result<()> {
        new_game::new_game(ctx, player_two)
    }

    pub fn roll_dice(ctx: Context<Play>) -> Result<()> {
        roll_dice::roll_dice(ctx)
    }

    pub fn commit_dice_hash(ctx: Context<Play>, hash: [u8; 32]) -> Result<()> {
        commit_dice_hash::commit_dice_hash(ctx, hash)
    }

    pub fn commit_dice_plain(ctx: Context<Play>, random_num: u16, salt: [u8; 32]) -> Result<()> {
        commit_dice_plain::commit_dice_plain(ctx, random_num, salt)
    }

    pub fn buy_square(ctx: Context<Play>) -> Result<()> {
        buy_square::buy_square(ctx)
    }

    pub fn buy_house(ctx: Context<Play>, house_position: u8) -> Result<()> {
        buy_house::buy_house(ctx, house_position)
    }

    pub fn sell_house(ctx: Context<Play>, house_position: u8) -> Result<()> {
        sell_house::sell_house(ctx, house_position)
    }

    pub fn sell_square(ctx: Context<Play>, square_position: u8, bid_value: u16) -> Result<()> {
        sell_square::sell_square(ctx, square_position, bid_value)
    }

    pub fn bid_for_sell(ctx: Context<Play>, bid_value: u16) -> Result<()> {
        bid_for_sell::bid_for_sell(ctx, bid_value)
    }

    pub fn accept_sell_bid(ctx: Context<Play>) -> Result<()> {
        accept_sell_bid::accept_sell_bid(ctx)
    }

    pub fn reject_sell_bid(ctx: Context<Play>) -> Result<()> {
        reject_sell_bid::reject_sell_bid(ctx)
    }

    pub fn mortgage(ctx: Context<Play>, square_position: u8) -> Result<()> {
        mortgage::mortgage(ctx, square_position)
    }

    pub fn unmortgage(ctx: Context<Play>, square_position: u8) -> Result<()> {
        unmortgage::unmortgage(ctx, square_position)
    }

    pub fn bid_for_action(ctx: Context<Play>, bid_value: u16) -> Result<()> {
        bid_for_action::bid_for_action(ctx, bid_value)
    }

    pub fn accept_action_bid(ctx: Context<Play>) -> Result<()> {
        accept_action_bid::accept_action_bid(ctx)
    }

    pub fn end_turn(ctx: Context<Play>) -> Result<()> {
        end_turn::end_turn(ctx)
    }
}
