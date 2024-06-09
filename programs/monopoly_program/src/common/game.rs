use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, PartialEq, Eq)]
pub struct Game {
    pub players: [Player; 2],
    pub current_player: WhichPlayer,
    pub player_status: PlayerStatus,
    pub dice_status: DiceStatus,
    pub board_status: [SquareStatus; 16],
}

impl Game {
    pub const SEED_MONEY: u16 = 1500;
    pub const SALARY: u16 = 200;
    pub const BOARD_INFO: [SquareInfo; 16] = [
        SquareInfo {
            price: 60,
            house_price: 50,
            rent: [2, 10, 30, 90, 160, 250],
            color: SquareColor::Red,
        },
        SquareInfo {
            price: 60,
            house_price: 50,
            rent: [4, 20, 60, 180, 320, 450],
            color: SquareColor::Yellow,
        },
        SquareInfo {
            price: 100,
            house_price: 50,
            rent: [6, 30, 90, 270, 400, 550],
            color: SquareColor::Blue,
        },
        SquareInfo {
            price: 100,
            house_price: 50,
            rent: [6, 30, 90, 270, 400, 550],
            color: SquareColor::Green,
        },
        SquareInfo {
            price: 120,
            house_price: 50,
            rent: [8, 40, 100, 300, 450, 600],
            color: SquareColor::Red,
        },
        SquareInfo {
            price: 140,
            house_price: 100,
            rent: [10, 50, 150, 450, 625, 750],
            color: SquareColor::Yellow,
        },
        SquareInfo {
            price: 140,
            house_price: 100,
            rent: [10, 50, 150, 450, 625, 750],
            color: SquareColor::Blue,
        },
        SquareInfo {
            price: 160,
            house_price: 100,
            rent: [12, 60, 180, 500, 700, 900],
            color: SquareColor::Green,
        },
        SquareInfo {
            price: 180,
            house_price: 100,
            rent: [14, 70, 200, 550, 750, 950],
            color: SquareColor::Red,
        },
        SquareInfo {
            price: 180,
            house_price: 100,
            rent: [14, 70, 200, 550, 750, 950],
            color: SquareColor::Yellow,
        },
        SquareInfo {
            price: 200,
            house_price: 100,
            rent: [16, 80, 220, 600, 800, 1000],
            color: SquareColor::Blue,
        },
        SquareInfo {
            price: 220,
            house_price: 150,
            rent: [18, 90, 250, 700, 875, 1050],
            color: SquareColor::Green,
        },
        SquareInfo {
            price: 220,
            house_price: 150,
            rent: [18, 90, 250, 700, 875, 1050],
            color: SquareColor::Red,
        },
        SquareInfo {
            price: 240,
            house_price: 150,
            rent: [20, 100, 300, 750, 925, 1100],
            color: SquareColor::Yellow,
        },
        SquareInfo {
            price: 260,
            house_price: 150,
            rent: [22, 110, 330, 800, 975, 1150],
            color: SquareColor::Blue,
        },
        SquareInfo {
            price: 260,
            house_price: 150,
            rent: [22, 110, 330, 800, 975, 1150],
            color: SquareColor::Green,
        },
    ];

    pub fn new(player_one_pubkey: Pubkey, player_two_pubkey: Pubkey) -> Self {
        Self {
            players: [
                Player {
                    pubkey: player_one_pubkey,
                    balance: Self::SEED_MONEY,
                    position: 0,
                },
                Player {
                    pubkey: player_two_pubkey,
                    balance: Self::SEED_MONEY,
                    position: 0,
                },
            ],
            current_player: WhichPlayer::PlayerOne,
            player_status: PlayerStatus::BeforeMoving,
            dice_status: DiceStatus::Rolled(6),
            board_status: Default::default(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Eq)]
pub struct Player {
    pub pubkey: Pubkey,
    pub balance: u16,
    pub position: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Eq)]
pub enum PlayerStatus {
    BeforeMoving,
    AfterMoving,
    Sell { position: u8, bid: Bid },
    Action(Bid),
    Lose,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq)]
pub enum WhichPlayer {
    PlayerOne,
    PlayerTwo,
}

impl WhichPlayer {
    pub fn as_index(&self) -> usize {
        *self as usize
    }
    pub fn either_index(&self) -> usize {
        match self {
            WhichPlayer::PlayerOne => WhichPlayer::PlayerTwo as usize,
            WhichPlayer::PlayerTwo => WhichPlayer::PlayerOne as usize,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Eq)]
pub enum DiceStatus {
    Rolling {
        hash_from_each: [Option<[u8; 32]>; 2],
        plain_from_either: Option<DicePlain>,
    },
    Rolled(u8),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DiceHash {
    player_one: Option<[u8; 32]>,
    player_two: Option<[u8; 32]>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Eq)]
pub struct DicePlain {
    pub from: WhichPlayer,
    pub random_num: u16,
    pub salt: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Eq)]
pub struct Bid {
    pub from: WhichPlayer,
    pub value: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Copy)]
pub struct Players {
    pub player_one: Pubkey,
    pub player_two: Pubkey,
}

impl Players {
    pub fn to_pda(&self) -> Pubkey {
        let seeds = [b"game", self.player_one.as_ref(), self.player_two.as_ref()];
        Pubkey::find_program_address(&seeds, &crate::ID).0
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Eq)]
pub enum SquareStatus {
    Unowned,
    Owned { by: WhichPlayer, level: u8 },
    Mortgaged { by: WhichPlayer },
}

impl Default for SquareStatus {
    fn default() -> Self {
        Self::Unowned
    }
}

pub struct SquareInfo {
    pub price: u16,
    pub house_price: u16,
    pub rent: [u16; 6],
    pub color: SquareColor,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SquareColor {
    Red,
    Yellow,
    Blue,
    Green,
}
