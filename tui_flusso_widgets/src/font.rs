#![allow(dead_code)]

//! Bitmap font for [`Ascii`].
//!
//! Each character is a 5x5 grid, represented as a [`u32`] with 7 dummy bits.
//! Following the dummy sector, the bitmaps start at the top left cell in rows
//! of 5 until the least significant bit at the bottom right cell.
//!
//! For example, [`A`] is `01110_10001_11111_10001_10001`. This represents
//! the glyph shown below.
//!
//! ```
//! 01110 | .xxx.
//! 10001 | x...x
//! 11111 | xxxxx
//! 10001 | x...x
//! 10001 | x...x
//! ```

/// Initial bit mask for each glyph.
pub const START_MASK: u32 = 1 << 24;

pub const A: u32 = 0b01110_10001_11111_10001_10001;
pub const B: u32 = 0b11110_10001_11110_10001_11110;
pub const C: u32 = 0b01111_10000_10000_10000_01111;
pub const D: u32 = 0b11110_10001_10001_10001_11110;
pub const E: u32 = 0b11111_10000_11110_10000_11111;
pub const F: u32 = 0b11111_10000_11110_10000_10000;
pub const G: u32 = 0b01111_10000_10011_10001_11110;
pub const H: u32 = 0b10001_10001_11111_10001_10001;
pub const I: u32 = 0b11111_00100_00100_00100_11111;
pub const J: u32 = 0b11111_00100_00100_00100_11000;
pub const K: u32 = 0b10001_10011_11110_10011_10001;
pub const L: u32 = 0b10000_10000_10000_10000_11111;
pub const M: u32 = 0b10001_11011_10101_10001_10001;
pub const N: u32 = 0b10001_11001_10101_10011_10001;
pub const O: u32 = 0b01110_10001_10001_10001_01110;
pub const P: u32 = 0b11110_10001_11110_10000_10000;
pub const Q: u32 = 0b01111_10001_10101_10010_01101;
pub const R: u32 = 0b11110_10001_11110_10010_10001;
pub const S: u32 = 0b01111_10000_11111_00001_11110;
pub const T: u32 = 0b11111_00100_00100_00100_00100;
pub const U: u32 = 0b10001_10001_10001_10001_11111;
pub const V: u32 = 0b10001_10001_10001_01010_00100;
pub const W: u32 = 0b10001_10001_10101_11011_10001;
pub const X: u32 = 0b10001_01010_00100_01010_10001;
pub const Y: u32 = 0b10001_01010_00100_00100_00100;
pub const Z: u32 = 0b11111_00010_00100_01000_11111;

pub const DIGITS: [u32; 10] = [
	0b11111_10011_10101_11001_11111,
	0b00100_01100_00100_00100_11111,
	0b11111_00001_11111_10000_11111,
	0b11111_00001_11111_00001_11111,
	0b10001_10001_11111_00001_00001,
	0b11111_10000_11110_00001_11110,
	0b01111_10000_11110_10001_11110,
	0b11111_00010_00100_01000_10000,
	0b11111_10001_11111_10001_11111,
	0b11111_10001_11111_00001_11111,
];

pub const COLON: u32 = 0b01110_01110_00000_01110_01110;
pub const SPACE: u32 = 0b00000_00000_00000_00000_00000;

pub fn get_glyph(x: char) -> u32 {
	match x {
		'a' => A,
		'b' => B,
		'c' => C,
		'd' => D,
		'e' => E,
		'f' => F,
		'g' => G,
		'h' => H,
		'i' => I,
		'j' => J,
		'k' => K,
		'l' => L,
		'm' => M,
		'n' => N,
		'o' => O,
		'p' => P,
		'q' => Q,
		'r' => R,
		's' => S,
		't' => T,
		'u' => U,
		'v' => V,
		'w' => W,
		'x' => X,
		'y' => Y,
		'z' => Z,
		'0' => DIGITS[0],
		'1' => DIGITS[1],
		'2' => DIGITS[2],
		'3' => DIGITS[3],
		'4' => DIGITS[4],
		'5' => DIGITS[5],
		'6' => DIGITS[6],
		'7' => DIGITS[7],
		'8' => DIGITS[8],
		'9' => DIGITS[9],
		':' => COLON,
		_ => SPACE,
	}
}
