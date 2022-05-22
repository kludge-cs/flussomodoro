use tui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color, Style},
	widgets::{Block, Widget},
};

use crate::fallback_text;

pub struct Ascii<'a> {
	block: Option<Block<'a>>,
	style: Style,
	text: String,
}

impl<'a> Ascii<'a> {
	pub fn new<T: ToString>(text: T) -> Self {
		let mut text = text.to_string();
		text.make_ascii_lowercase();
		Ascii { block: None, style: Style::default(), text }
	}

	pub fn block(mut self, block: Block<'a>) -> Self {
		self.block = Some(block);
		self
	}

	pub fn style(mut self, style: Style) -> Self {
		self.style = style;
		self
	}
}

impl<'a> Widget for Ascii<'a> {
	fn render(mut self, area: Rect, buf: &mut Buffer) {
		buf.set_style(area, self.style);
		let ascii_area = match self.block.take() {
			Some(b) => {
				let inner_area = b.inner(area);
				b.render(area, buf);
				inner_area
			}
			None => area,
		};

		let width = {
			let len = self.text.len();
			(len * 5 + (len - 1) * 3) as u16
		};
		// Use normal text if buffer is too small
		if ascii_area.height < 5 || ascii_area.width < width as u16 {
			return fallback_text(self.text, self.style, ascii_area, buf);
		}

		let origin: (u16, u16) = (
			ascii_area.x + ascii_area.width / 2 - width / 2,
			ascii_area.y + ascii_area.height / 2 - 2,
		);

		for (i, digit) in self.text.chars().enumerate() {
			let start_x = origin.0 + i as u16 * 8;
			let glyph = font::get_glyph(digit);
			let mut mask = font::START_MASK;
			for glyph_px in 0..25 {
				if glyph & mask > 0 {
					buf.get_mut(
						start_x + glyph_px % 5,
						origin.1 + glyph_px / 5,
					)
					.set_symbol(" ")
					.set_bg(self.style.fg.unwrap_or(Color::Reset))
					.set_fg(self.style.bg.unwrap_or(Color::Reset));
				}
				mask >>= 1;
			}
		}
	}
}

mod font {
	#![allow(dead_code)]

	//! Bitmap font for [`Ascii`].
	//!
	//! Each character is a 5x5 grid, represented as a [`u32`] with 7 dummy
	//! bits. Following the dummy sector, the bitmaps start at the top left
	//! cell in rows of 5 until the least significant bit at the bottom right
	//! cell.
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

	// Ordered by CP437 - anything marked N is non-standard

	pub const SPACE: u32 = 0b00000_00000_00000_00000_00000;
	pub const SMILE: u32 = 0b01010_00000_00100_10001_01110;
	pub const FROWN: u32 = 0b01010_00000_00100_00000_11111; // N - replaces neg smile
	pub const HEART: u32 = 0b11011_11111_11111_01110_00100;
	pub const DIMND: u32 = 0b01110_10001_10001_01010_00100;
	pub const CLUBS: u32 = 0b00100_01010_00100_00100_01110;
	pub const SPADE: u32 = 0b00100_01110_01110_00100_01110;
	pub const CIRCL: u32 = 0b00000_01110_01110_01110_00000;
	pub const NEG_CIRCL: u32 = 0b11111_10001_10001_10001_11111;
	pub const HOLO_CIRCL: u32 = 0b00000_01110_01010_01110_00000;
	pub const RIGHT_ARROW: u32 = 0b10100_10110_11111_00110_00100; // N - replaces male icon
	pub const LEFT_ARROW: u32 = 0b00101_01101_11111_01100_00100; // N - replaces female icon
	pub const SQAVR: u32 = 0b00011_00010_00010_01110_01110;
	pub const DQAVR: u32 = 0b01111_01001_01001_11011_11011;
	pub const SOLAR: u32 = 0b10101_01110_11111_01110_10101;
	pub const RHEAD: u32 = 0b11000_11110_11111_11110_11000;
	pub const LHEAD: u32 = 0b00011_01111_11111_01111_00011;
	pub const VAIFF: u32 = 0b01110_10101_00100_10101_01110;
	pub const TEXCL: u32 = 0b10101_10101_10101_00000_10101;
	pub const PLCRO: u32 = 0b11111_10101_11101_00101_00101;
	pub const SECTS: u32 = 0b01100_01000_00100_00010_00110;
	pub const BRECT: u32 = 0b00000_00000_11111_00000_00000;

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

	pub fn get_glyph(x: char) -> u32 {
		if let Some(x) = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
			.iter()
			.position(|&c| x == c)
		{
			return DIGITS[x];
		}
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
			':' => COLON,
			_ => SPACE,
		}
	}
}
