#![feature(test)]
#![feature(associated_consts)]
#![feature(cfg_target_feature)]
#![feature(platform_intrinsics)]

extern crate test;

#[macro_use]
extern crate lazy_static;

mod types;
mod bitboard;
mod board;

pub mod square;
pub mod attacks;
pub mod position;
pub mod perft;

pub use square::Square;
pub use types::{Color, Role, Piece, Move};
pub use bitboard::{Bitboard, CarryRippler};
pub use board::Board;
pub use position::Position;
