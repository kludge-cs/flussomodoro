use std::{cmp::min, f64::consts::TAU};

use tui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color, Style},
	text::Span,
	widgets::{Block, Widget},
};

#[derive(Default)]
pub struct CircularGauge<'a> {
	block: Option<Block<'a>>,
	ratio: f64,
	origin_angle: Option<f64>,
	final_angle: Option<f64>,
	label: Option<Span<'a>>,
	style: Style,
	gauge_style: Style,
}

impl<'a> CircularGauge<'a> {
	pub fn block(mut self, block: Block<'a>) -> Self {
		self.block = Some(block);
		self
	}

	pub fn percent(mut self, percent: u8) -> Self {
		assert!(
			percent <= 100,
			"Percentage should be between 0 and 100 inclusively.",
		);
		self.ratio = percent as f64 / 100.0;
		self
	}

	pub fn ratio(mut self, ratio: f64) -> Self {
		assert!(
			ratio <= 1.0 && ratio >= 0.0,
			"Ratio should be between 0 and 1 inclusively."
		);
		self.ratio = ratio;
		self
	}

	pub fn label<T: Into<Span<'a>>>(mut self, label: T) -> Self {
		self.label = Some(label.into());
		self
	}

	pub fn style(mut self, style: Style) -> Self {
		self.style = style;
		self
	}

	pub fn gauge_style(mut self, style: Style) -> Self {
		self.gauge_style = style;
		self
	}
}

impl<'a> Widget for CircularGauge<'a> {
	fn render(mut self, area: Rect, buf: &mut Buffer) {
		buf.set_style(area, self.style);
		let gauge_area = match self.block.take() {
			Some(b) => {
				let inner_area = b.inner(area);
				b.render(area, buf);
				inner_area
			}
			None => area,
		};

		let origin = (
			gauge_area.x + gauge_area.width / 2,
			gauge_area.y + gauge_area.height / 2,
		);
		let rad = if let Some(x) =
			(min(gauge_area.height, gauge_area.width) / 2).checked_sub(2)
		{
			x as f64
		} else {
			return;
		};
		let diam = rad * 2.0;
		let rad_sq = rad.powi(2);

		// compute label value and its position
		// label is put at the center of the gauge_area
		let label = self.label.unwrap_or_else(|| {
			Span::from(format!("{}%", (self.ratio * 100.0).round()))
		});
		let clamped_label_width = min(diam as u16, label.width() as u16);

		// the gauge will be filled proportionally to the ratio
		let origin_angle = self.origin_angle.unwrap_or(0.0);
		let final_angle = self.final_angle.unwrap_or_else(|| TAU * self.ratio);

		for x in origin.0 + 1 - diam as u16..origin.0 + diam as u16 {
			for y in origin.1 + 1 - rad as u16..origin.1 + rad as u16 {
				let opp = (x as f64 - origin.0 as f64) * 0.5;
				let adj = y as f64 - origin.1 as f64;
				// determine if point lies within circle - pythagorean theorem
				if !(rad_sq - diam..=rad_sq + diam)
					.contains(&(opp.powi(2) + adj.powi(2)))
				{
					continue;
				}
				let mut mid_angle = opp.atan2(adj);
				if mid_angle < 0.0 {
					mid_angle += TAU;
				}
				if (origin_angle..final_angle).contains(&mid_angle) {
					// spaces are needed to apply the background styling
					buf.get_mut(x, y)
						.set_symbol("*")
						.set_fg(self.gauge_style.fg.unwrap_or(Color::Reset))
						.set_bg(self.gauge_style.bg.unwrap_or(Color::Reset));
				}
			}
		}
		buf.set_span(
			origin.0 - clamped_label_width / 2,
			origin.1,
			&label,
			clamped_label_width,
		);
	}
}
