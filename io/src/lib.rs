#![no_std]
use gmeta::{In, InOut, Out};
use gstd::prelude::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
    Error(String), // 添加 Error 变体
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum Player {
    #[default]
    User,
    Program,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}

pub struct PebblesMetadata;

impl gmeta::Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
