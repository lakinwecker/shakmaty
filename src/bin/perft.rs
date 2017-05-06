// This file is part of the shakmaty library.
// Copyright (C) 2017 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Count legal move paths.
//!
//! # Examples
//!
//! ```
//! use shakmaty::Chess;
//! use shakmaty::perft::perft;
//!
//! let pos = Chess::default();
//! assert_eq!(perft(&pos, 1), 20);
//! assert_eq!(perft(&pos, 2), 400);
//! assert_eq!(perft(&pos, 3), 8902);
//! ```

extern crate shakmaty;
use shakmaty::uci::Uci;
use shakmaty::{Position, MoveList, Chess};

/// Counts legal move paths of a given length.
///
/// Paths with mate or stalemate are not counted unless it occurs in the final
/// position. Useful for comparing, testing and debugging move generation
/// correctness and performance.
pub fn perft<P: Position>(pos: &P, depth: u8) -> usize {
    if depth < 1 {
        1
    } else {
        let mut moves = MoveList::new();
        pos.legal_moves(&mut moves);

        if depth == 1 {
            moves.len()
        } else {
            moves.drain(..).map(|ref m| {
                let child = pos.clone().play_unchecked(m);
                perft(&child, depth - 1)
            }).sum()
        }
    }
}

/// Like `perft()`, but also prints the perft of each child for debugging.
pub fn debug_perft<P: Position>(pos: &P, depth: u8) -> usize {
    if depth < 1 {
        1
    } else {
        let mut moves = MoveList::new();
        pos.legal_moves(&mut moves);

        moves.iter().map(|m| {
            let child = pos.clone().play(m).expect("legal move");
            let nodes = perft(&child, depth - 1);
            let uci: Uci = m.into();
            println!("{} {} {}: {}", uci, m, depth - 1, nodes);
            nodes
        }).sum()
    }
}
fn main() {
    let pos = Chess::default();
    assert_eq!(perft(&pos, 5), 4865609);
}
