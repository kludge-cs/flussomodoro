#![allow(clippy::declare_interior_mutable_const)]

use std::lazy::{Lazy, OnceCell};

use tui::{
	layout::Alignment,
	style::{Color, Modifier, Style},
	widgets::{Block, Borders},
};

pub const STD: Lazy<Style> = Lazy::new(|| Style::default().fg(Color::White));
pub const FOCUS: Lazy<Style> =
	Lazy::new(|| Style::default().fg(Color::LightRed));
pub const BREAK: Lazy<Style> =
	Lazy::new(|| Style::default().fg(Color::LightBlue));
pub const ELEM: Lazy<Style> = Lazy::new(|| Style::default().fg(Color::Green));
pub const ELEM_SEL: Lazy<Style> =
	Lazy::new(|| Style::default().fg(Color::Yellow));
pub const X_GAUGE: Lazy<Style> =
	Lazy::new(|| Style::default().fg(Color::Green).bg(Color::Black));
pub const HEADING: Lazy<Style> = Lazy::new(|| {
	Style::default().add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
});

const BLOCK_STD: OnceCell<Block> = OnceCell::new();

pub fn block_std() -> Block<'static> {
	BLOCK_STD
		.get_or_init(|| {
			Block::default()
				.borders(Borders::ALL)
				.style(*STD)
				.title_alignment(Alignment::Center)
		})
		.to_owned()
}
