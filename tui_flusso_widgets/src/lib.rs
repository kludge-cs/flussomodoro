mod aligned_tabs;
mod ascii;
mod circular_gauge;

pub use aligned_tabs::AlignedTabs;
pub use ascii::Ascii;
pub use circular_gauge::CircularGauge;
use tui::{
	buffer::Buffer,
	layout::{Alignment, Rect},
	style::Style,
	widgets::{Paragraph, Widget},
};

pub fn fallback_text<'a, T>(
	text: T,
	style: Style,
	area: Rect,
	buf: &mut Buffer,
) where
	T: Into<tui::text::Text<'a>>,
{
	Paragraph::new(text)
		.style(style)
		.alignment(Alignment::Center)
		.render(area, buf)
}
