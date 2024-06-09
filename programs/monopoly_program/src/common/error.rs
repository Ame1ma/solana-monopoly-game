use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    NotPlayer,
    NotRolling,
    HashCollectNotFinish,
    HashVerifyFailed,
    NotCurrentPlayer,
    AlreadyOwned,
    BalanceNotEnough,
    PositionOutOfBound,
    NotOwned,
    LevelNotAllowed,
    NotInSell,
    WaitAnotherPlayerBid,
    NotMortgaged,
    NotInAction,
    BidValueNotHigher,
}
