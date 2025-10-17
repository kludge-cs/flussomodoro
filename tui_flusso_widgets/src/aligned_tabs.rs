use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Rect},
	style::Style,
	symbols,
	text::{Line, Span},
	widgets::{Block, Widget},
};

#[derive(Debug, Clone)]
pub struct AlignedTabs<'a> {
	block: Option<Block<'a>>,
	titles: Vec<Line<'a>>,
	selected: usize,
	style: Style,
	highlight_style: Style,
	divider: Span<'a>,
	alignment: Alignment,
}

impl<'a> AlignedTabs<'a> {
	pub fn new(titles: Vec<Line<'a>>) -> Self {
		AlignedTabs {
			block: None,
			titles,
			selected: 0,
			style: Default::default(),
			highlight_style: Default::default(),
			divider: Span::raw(symbols::line::VERTICAL),
			alignment: Alignment::Left,
		}
	}

	pub fn block(mut self, block: Block<'a>) -> Self {
		self.block = Some(block);
		self
	}

	pub fn select(mut self, selected: usize) -> Self {
		self.selected = selected;
		self
	}

	pub fn style(mut self, style: Style) -> Self {
		self.style = style;
		self
	}

	pub fn highlight_style(mut self, style: Style) -> Self {
		self.highlight_style = style;
		self
	}

	pub fn divider<T: Into<Span<'a>>>(mut self, divider: T) -> Self {
		self.divider = divider.into();
		self
	}

	pub fn alignment(mut self, alignment: Alignment) -> Self {
		self.alignment = alignment;
		self
	}
}

impl<'a> Widget for AlignedTabs<'a> {
	fn render(mut self, area: Rect, buf: &mut Buffer) {
		buf.set_style(area, self.style);
		let tabs_area = match self.block.take() {
			Some(b) => {
				let inner_area = b.inner(area);
				b.render(area, buf);
				inner_area
			}
			None => area,
		};

		if tabs_area.height == 0 {
			return;
		}

		let titles_len = self.titles.len();
		let width = self.titles.iter().map(|x| x.width()).sum::<usize>()
			+ self.divider.width() * (titles_len - 1)
			+ 2 * titles_len;
		let mut x = match self.alignment {
			Alignment::Left => tabs_area.left(),
			Alignment::Center => {
				tabs_area.left() + tabs_area.width / 2 - width as u16 / 2 - 2
			}
			Alignment::Right => tabs_area.right() - width as u16,
		};
		for (idx, title) in self.titles.into_iter().enumerate() {
			let last_title = titles_len - 1 == idx;
			x = x.saturating_add(1);
			let remaining_width = tabs_area.right().saturating_sub(x);
			if remaining_width == 0 {
				break;
			}
			let pos = buf.set_line(x, tabs_area.top(), &title, remaining_width);
			if idx == self.selected {
				buf.set_style(
					Rect {
						x,
						y: tabs_area.top(),
						width: pos.0.saturating_sub(x),
						height: 1,
					},
					self.highlight_style,
				);
			}
			x = pos.0.saturating_add(1);
			let remaining_width = tabs_area.right().saturating_sub(x);
			if remaining_width == 0 || last_title {
				break;
			}
			let pos = buf.set_span(
				x,
				tabs_area.top(),
				&self.divider,
				remaining_width,
			);
			x = pos.0;
		}
	}
}
