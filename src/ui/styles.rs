use std::sync::{LazyLock, OnceLock};

use ratatui::{
	layout::Alignment,
	style::{Color, Modifier, Style},
	widgets::{Block, Borders},
};

pub static STD: LazyLock<Style> =
	LazyLock::new(|| Style::default().fg(Color::White));
pub static FOCUS: LazyLock<Style> =
	LazyLock::new(|| Style::default().fg(Color::LightRed));
pub static BREAK: LazyLock<Style> =
	LazyLock::new(|| Style::default().fg(Color::LightBlue));
pub static ELEM: LazyLock<Style> =
	LazyLock::new(|| Style::default().fg(Color::Green));
pub static ELEM_SEL: LazyLock<Style> =
	LazyLock::new(|| Style::default().fg(Color::Yellow));
pub static X_GAUGE: LazyLock<Style> =
	LazyLock::new(|| Style::default().fg(Color::Green).bg(Color::Black));
pub static HEADING: LazyLock<Style> = LazyLock::new(|| {
	Style::default().add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
});

static BLOCK_STD: OnceLock<Block> = OnceLock::new();

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
