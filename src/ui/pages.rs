#![allow(clippy::declare_interior_mutable_const)]

use std::{cmp::min, lazy::SyncOnceCell};

use time_fmt::FormattedTime;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	style::Modifier,
	text::{Span, Spans},
	widgets::{Gauge, Paragraph, Widget},
	Frame,
};
use tui_flusso_widgets::{Ascii, CircularGauge};

use super::{styles::*, Page};
use crate::app::App;

#[derive(Clone, Copy, Default)]
pub struct Main {}

impl Page for Main {
	fn render<B: Backend>(&self, area: Rect, f: &mut Frame<B>, app: &App) {
		let focus_time = *app.counter.focus_time();
		let original_focus_time = *app.counter.original_focus_time();

		let chunks = Layout::default()
			.constraints(vec![Constraint::Length(5), Constraint::Min(10)])
			.split(area);
		let status_session_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(50),
				Constraint::Percentage(50),
			])
			.split(chunks[0]);
		let focus_break_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(50),
				Constraint::Percentage(50),
			])
			.split(chunks[1]);

		f.render_widget(
			Paragraph::new(vec![
				Spans::from(Span::raw("Current task: TBD!")),
				Spans::from(Span::raw("Task progress: TBD!")),
				Spans::from(format!(
					"Status: {}",
					app.counter.work_state().to_string()
				)),
			])
			.block(block_std().title("Status"))
			.style(*STD),
			status_session_chunks[0],
		);
		f.render_widget(
			Gauge::default()
				.block(block_std().title("Session"))
				.gauge_style(*X_GAUGE)
				.label(format!("{}/4", app.counter.pom()))
				.use_unicode(true)
				.ratio(*app.counter.pom() as f64 / 4.0),
			status_session_chunks[1],
		);
		if app.opts.ascii {
			f.render_widget(focus_ascii(focus_time), focus_break_chunks[0]);
		} else {
			f.render_widget(
				focus_gauge(focus_time, original_focus_time),
				focus_break_chunks[0],
			);
		}
		f.render_widget(
			break_ascii(*app.counter.break_time()),
			focus_break_chunks[1],
		);
	}
}

fn focus_gauge(remaining: u16, initial: u16) -> impl Widget {
	let ratio = (initial - remaining) as f64 / initial as f64;

	CircularGauge::default()
		.block(block_std().title("Focus"))
		.gauge_style(*FOCUS)
		.label(Span::styled(
			format!(
				"{} ({:.1}%)",
				FormattedTime::from(remaining),
				ratio * 100.0
			),
			FOCUS.add_modifier(Modifier::ITALIC),
		))
		.ratio(ratio)
}

pub fn focus_ascii(focus_time: u16) -> impl Widget {
	Ascii::new(FormattedTime::from(focus_time))
		.block(block_std().title("Focus"))
		.style(*FOCUS)
}

fn break_ascii(break_time: u16) -> impl Widget {
	Ascii::new(FormattedTime::from(break_time))
		.block(block_std().title("Break"))
		.style(*BREAK)
}

#[derive(Clone, Copy, Default)]
pub struct Help {
	scroll: u16,
}

impl Help {
	const CONTENT: SyncOnceCell<Vec<Spans<'static>>> = SyncOnceCell::new();

	fn content() -> Vec<Spans<'static>> {
		Self::CONTENT
			.get_or_init(|| {
				vec![
					Spans::from(Span::styled("Help", *HEADING)),
					Spans::from(""),
					Spans::from("[h] - This menu"),
					Spans::from("[p] - Toggle pause"),
					Spans::from("[b] - Toggle break (while not paused)"),
				]
			})
			.to_owned()
	}

	pub fn scroll_to(&mut self, scroll: u16) {
		self.scroll = min(scroll, Self::content().len() as u16 - 1);
	}

	pub fn scroll_by(&mut self, scroll: i16) {
		if let Some(new_scroll_pos) = self.scroll.checked_add_signed(scroll) {
			self.scroll_to(new_scroll_pos);
		};
	}
}

impl Page for Help {
	fn render<B: Backend>(&self, area: Rect, f: &mut Frame<B>, _: &App) {
		f.render_widget(
			Paragraph::new(Self::content())
				.style(*STD)
				.block(block_std())
				.scroll((self.scroll, 0)),
			area,
		)
	}
}
