use std::lazy::{SyncLazy, SyncOnceCell};

use tui::{
	layout::Alignment,
	style::{Color, Modifier, Style},
	widgets::{Block, Borders},
};

pub static STD: SyncLazy<Style> =
	SyncLazy::new(|| Style::default().fg(Color::White));
pub static FOCUS: SyncLazy<Style> =
	SyncLazy::new(|| Style::default().fg(Color::LightRed));
pub static BREAK: SyncLazy<Style> =
	SyncLazy::new(|| Style::default().fg(Color::LightBlue));
pub static ELEM: SyncLazy<Style> =
	SyncLazy::new(|| Style::default().fg(Color::Green));
pub static ELEM_SEL: SyncLazy<Style> =
	SyncLazy::new(|| Style::default().fg(Color::Yellow));
pub static X_GAUGE: SyncLazy<Style> =
	SyncLazy::new(|| Style::default().fg(Color::Green).bg(Color::Black));
pub static HEADING: SyncLazy<Style> = SyncLazy::new(|| {
	Style::default().add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
});

static BLOCK_STD: SyncOnceCell<Block> = SyncOnceCell::new();

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
